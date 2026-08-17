//! Attribute definition and usage parsing.

use crate::ast::{
    AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage, InOut, Membership,
    Multiplicity, Node, SubsettingKind, SubsettingRelationship, TypingKind,
};
use crate::parser::body::parse_structured_brace_members;
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::lex::{
    identification, name, qualified_reference, short_name_prefix, starts_with_keyword,
    subset_operator, ws1, ws_and_comments,
};
use crate::parser::node_from_to;
use crate::parser::usage::{
    multiplicity_node, optional_typings, prefix_redefinition_target, specialization_clauses,
    typing_node, typing_relationship_node, typings,
};
use crate::parser::with_span;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

const ATTRIBUTE_BODY_STARTERS: &[&[u8]] = &[
    b"doc",
    b"attribute",
    b"comment",
    b"@",
    b"#",
    b":>>",
    b":>",
    b":",
    b"ref",
    b"item",
    b"assert",
    b"constraint",
    b"private",
    b"derived",
    b"abstract",
    b"part",
    b"binding",
    b"connection",
    b"value",
    b"occurrence",
    b"feature",
    b"member",
    b"var",
    b"composite",
    b"portion",
    b"step",
    b"expr",
    b"bool",
    b"inv",
    b"connector",
    b"class",
    b"end",
    // KerML classifier-keyword family dispatched via `kerml_classifier_structured`
    // (spec42 Gap 38).
    b"classifier",
    b"struct",
    b"datatype",
    b"association",
    b"assoc",
    b"behavior",
    b"interaction",
    b"predicate",
    b"metaclass",
    b"function",
    b"multiplicity",
    b"type",
];

const ATTRIBUTE_OPAQUE_STARTERS: &[&[u8]] = &[
    // GH-90.1: `individual item ii : II1;` / `individual item :>> i : II2;` inside `item def`
    // bodies (Simple Tests/IndividualTest.sysml:4,15) are structured by `item_usage` itself
    // (which accepts the optional leading `individual`) via `AttributeBodyElement::ItemUsage`
    // below; `individual` stays here only to opaquely capture non-`item` individual forms that
    // reach this body.
    b"individual",
    b"constraint",
    b"private",
    b"derived",
    b"abstract",
    b"binding",
    b"connection",
    b":>>",
    b":>",
    b"attribute",
];

const METADATA_BODY_STARTERS: &[&[u8]] = &[
    b"doc",
    b"attribute",
    b"ref",
    b"comment",
    b":>",
    b":>>",
    b":",
    b"derived",
    b"item",
    b"abstract",
    // Structured members shared with `attribute_body_element` (spec42 Gap 40).
    b"part",
    b"connect",
    b"assert",
];

const METADATA_OPAQUE_STARTERS: &[&[u8]] = &[b"derived", b"item", b"abstract", b"ref"];

fn is_reserved_shorthand_starter(name: &str) -> bool {
    matches!(
        name,
        "interface"
            | "part"
            | "connect"
            | "bind"
            | "perform"
            | "allocate"
            | "port"
            | "state"
            | "satisfy"
            | "action"
            | "attribute"
            | "ref"
            | "doc"
            | "metadata"
            | "filter"
            | "use"
            | "view"
            | "viewpoint"
            | "render"
            | "rendering"
            | "requirement"
            | "require"
            | "concern"
            | "actor"
            | "item"
            | "individual"
            | "constraint"
            | "calc"
            | "enum"
            | "occurrence"
    )
}

/// Multiplicity-adjacent modifiers from `MultiplicityPart` (BNF §8.2.2.6.6): an optional
/// multiplicity range plus `ordered`/`nonunique` (and their negations `nonordered`/`unique`).
/// Results are OR-merged / first-wins across every occurrence -- callers that invoke this more
/// than once per declaration (leading and trailing modifier positions) fold the results together.
#[derive(Default, Clone)]
struct FeatureModifiers {
    multiplicity: Option<Node<Multiplicity>>,
    ordered: bool,
    nonunique: bool,
}

impl FeatureModifiers {
    fn merge(self, other: FeatureModifiers) -> FeatureModifiers {
        FeatureModifiers {
            multiplicity: self.multiplicity.or(other.multiplicity),
            ordered: self.ordered || other.ordered,
            nonunique: self.nonunique || other.nonunique,
        }
    }
}

/// Consume `literal` if the input starts with it, matching `nom`'s `tag` (no word boundary).
fn consume_literal<'a>(input: Input<'a>, literal: &[u8]) -> Option<Input<'a>> {
    input
        .fragment()
        .starts_with(literal)
        .then(|| nom::Input::take_from(&input, literal.len()))
}

/// Parse the multiplicity and ordering/uniqueness modifiers that may follow a feature.
///
/// Modifiers accumulate directly into the returned struct. Collecting them into a `Vec` first
/// would allocate on a path the grammar re-enters speculatively for every attribute declaration,
/// only to fold the result into these same three fields.
fn feature_modifiers(input: Input<'_>) -> IResult<Input<'_>, FeatureModifiers> {
    let mut result = FeatureModifiers::default();
    let mut input = input;
    loop {
        let (after_ws, _) = ws_and_comments(input)?;
        if let Ok((rest, node)) = multiplicity_node(after_ws) {
            // The first multiplicity wins; a repeated one is consumed and ignored.
            if result.multiplicity.is_none() {
                result.multiplicity = Some(node);
            }
            input = rest;
            continue;
        }
        // `unique` and `nonordered` are the defaults: recognized and consumed, but not recorded.
        // These match as bare prefixes, exactly as the `tag` alternatives they replace did.
        if let Some(rest) = consume_literal(after_ws, b"nonunique") {
            result.nonunique = true;
            input = rest;
        } else if let Some(rest) = consume_literal(after_ws, b"unique") {
            input = rest;
        } else if let Some(rest) = consume_literal(after_ws, b"ordered") {
            result.ordered = true;
            input = rest;
        } else if let Some(rest) = consume_literal(after_ws, b"nonordered") {
            input = rest;
        } else {
            // No modifier here: leave the whitespace before it unconsumed, as `preceded` did.
            return Ok((input, result));
        }
    }
}

enum MetadataBindingPrefix {
    Subsets,
    Redefines,
}

fn attribute_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<AttributeBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(
            crate::parser::body::annotating_member,
            AttributeBodyElement::Annotating,
        ),
        map(
            |i| attribute_def(i, true),
            AttributeBodyElement::AttributeDef,
        ),
        map(attribute_usage, AttributeBodyElement::AttributeUsage),
        // KerML type-body members: this body grammar also serves KerML `class`/`struct`/
        // `datatype` bodies via `class_def`, whose members are feature members (`feature x :
        // Natural[1];`, `member`/`derived`/`composite`/`portion`/`var` prefixed, `step`/`expr`/
        // `bool` kinds), invariants, connectors, and nested class definitions.
        map(crate::parser::constraint::kerml_feature_member, |n| {
            AttributeBodyElement::KermlFeature(Box::new(n))
        }),
        map(crate::parser::constraint::kerml_invariant_member, |n| {
            AttributeBodyElement::Invariant(Box::new(n))
        }),
        map(crate::parser::constraint::kerml_connector_member, |n| {
            AttributeBodyElement::KermlConnector(Box::new(n))
        }),
        map(crate::parser::package::class_def, |n| {
            AttributeBodyElement::ClassDef(Box::new(n))
        }),
        // The rest of the KerML classifier-keyword family (`struct`, `classifier`, `datatype`,
        // `assoc`, `behavior`, ...) nested in a type body (spec42 Gap 38); `class` stays on the
        // `class_def` arm above.
        map(crate::parser::package::kerml_classifier_structured, |n| {
            AttributeBodyElement::KermlClassifier(Box::new(n))
        }),
        map(value_keyword_binding, AttributeBodyElement::AttributeUsage),
        map(
            attribute_feature_binding,
            AttributeBodyElement::AttributeUsage,
        ),
        // §6 G27: this body is shared with `item def` / `item` usage bodies, which legally own
        // occurrence members. Placed after the bindings so a member merely *named* `occurrence`
        // still reaches them first.
        map(crate::parser::occurrence_body::occurrence_usage, |n| {
            AttributeBodyElement::OccurrenceUsage(Box::new(n))
        }),
        // GH-90.2: `timeslice`/`snapshot` portion usages inside attribute/item bodies, e.g.
        // `timeslice asPresident : Person [0..*] { ... }` (Individuals Examples/
        // JohnIndividualExample.sysml:11). Both already fully parse via `timeslice_usage`/
        // `snapshot_usage` (used elsewhere, e.g. part def bodies) -- just not dispatched here.
        map(crate::parser::occurrence_body::timeslice_usage, |n| {
            AttributeBodyElement::OccurrenceUsage(Box::new(n))
        }),
        map(crate::parser::occurrence_body::snapshot_usage, |n| {
            AttributeBodyElement::OccurrenceUsage(Box::new(n))
        }),
        // This body is also shared with `item def`/`item` usage bodies, which legally own
        // connector members (`connect a to b;`) and metadata tags (`#keyword`, bare or
        // prefixing the next member) -- see the OMG spec Annex `14c-Language Extensions.sysml`
        // FMEA library example, which uses both extensively.
        // Members `PartUsageBodyElement`/`PartDefBodyElement` already reach that this shared
        // body silently dropped to `Other` (spec42 Gap 49a): named/multiplicity-qualified
        // binds, named/typed connections, nested calcs, and plain (non-`assert`) constraints
        // (Geometry `ShapeItems.sysml`, Quantities and Units `Time.sysml`, Systems Library
        // `Items.sysml`). `calc_def_required` before `calc_usage` for the usual bare-`def`
        // reason; `connect_` rides in the same sub-alt to stay under nom's 21-branch limit.
        alt((
            map(crate::parser::part::connect_, AttributeBodyElement::Connect),
            map(crate::parser::part::bind_, |n| {
                AttributeBodyElement::Bind(Box::new(n))
            }),
            map(crate::parser::part::connection_usage_member, |n| {
                AttributeBodyElement::Connection(Box::new(n))
            }),
            map(crate::parser::constraint::calc_def_required, |n| {
                AttributeBodyElement::CalcDef(Box::new(n))
            }),
            map(crate::parser::constraint::calc_usage, |n| {
                AttributeBodyElement::CalcUsage(Box::new(n))
            }),
            map(crate::parser::constraint::constraint_usage, |n| {
                AttributeBodyElement::ConstraintUsage(Box::new(n))
            }),
        )),
        map(
            crate::parser::metadata_annotation::metadata_keyword_usage,
            AttributeBodyElement::MetadataKeywordUsage,
        ),
        map(
            crate::parser::metadata_annotation::metadata_keyword_prefix,
            AttributeBodyElement::MetadataKeywordUsage,
        ),
        // #72: `assert constraint` inside attribute / item bodies (15_01, 15_08).
        map(
            crate::parser::occurrence_body::assert_constraint_member,
            AttributeBodyElement::AssertConstraint,
        ),
        // `ref` / `ref part` / `ref :>> …` in attribute / item bodies (15_11, 15_19, 17a, 17b).
        // Before opaque capture so these no longer land in `Other`.
        map(
            crate::parser::connector::ref_decl,
            AttributeBodyElement::RefDecl,
        ),
        // Nested `part` in item / attribute bodies (validation `3e`, `14c`).
        map(crate::parser::part::part_usage, |n| {
            AttributeBodyElement::PartUsage(Box::new(n))
        }),
        // Nested `item` usage in item / attribute bodies, e.g. `item picture : Picture;` inside
        // `attribute def Show { ... }`. Before the opaque-capture fallback below so this common
        // shape gets a structured node -- see `AttributeBodyElement::ItemUsage`'s doc comment.
        map(crate::parser::item::item_usage, |n| {
            AttributeBodyElement::ItemUsage(Box::new(n))
        }),
        map(
            |i| {
                crate::parser::recovery::unsupported_member(
                    i,
                    ATTRIBUTE_OPAQUE_STARTERS,
                    "attribute body",
                )
            },
            AttributeBodyElement::Unsupported,
        ),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// §6 G9: `value :>> elements: Integer;` spells an attribute member with the `value` keyword.
/// Deliberately scoped to attribute-definition bodies (not the shared usage dispatch): the whole
/// OMG release contains exactly one occurrence, in `15_11-Variable Length Collection Types.sysml`,
/// so widening `attribute_usage` itself would make `value` a usage keyword everywhere for no gain.
/// The keyword carries no information the AST doesn't already hold, so it is consumed and dropped.
fn value_keyword_binding(input: Input<'_>) -> IResult<Input<'_>, Node<AttributeUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"value"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, binding) = attribute_feature_binding(input)?;
    Ok((input, node_from_to(start, input, binding.value)))
}

/// Attribute body feature binding: (`:>>` | `:>`)? name (`:` type)? (`=` value)? (`;` | `{` body `}`).
///
/// Catalog unit definitions use this shape for `unitConversion` redefinitions, e.g.
/// `:>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; }`.
pub(crate) fn attribute_feature_binding(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<AttributeUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, prefix) = nom::combinator::opt(alt((
        // GH-92.1: accept the literal `redefines` keyword too, not just `:>>` -- both are
        // synonyms per `redefine_operator` (already used elsewhere, e.g. `part_usage`), e.g.
        // `redefines mass = 1000 [kg];` (`Mass Roll-up Example/Vehicles.sysml:26`).
        map(
            preceded(ws_and_comments, crate::parser::lex::redefine_operator),
            |_| MetadataBindingPrefix::Redefines,
        ),
        map(preceded(ws_and_comments, subset_operator), |_| {
            MetadataBindingPrefix::Subsets
        }),
    )))
    .parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (_, (name_span, name_str)) = with_span(name).parse(input)?;
    // The prefixed forms take the same comma-separated multi-target list as every other
    // `:>>`/`:>` clause (`:>> A::x, B::y { ... }`, Quantities and Units `SI.kerml`; spec42
    // Gap 49b); the unprefixed binding keeps its single name-reference.
    let (input, targets) = if prefix.is_some() {
        crate::parser::usage::specialization_targets(input)?
    } else {
        let (input, target) = qualified_reference(input)?;
        (input, vec![target])
    };
    if is_reserved_shorthand_starter(&name_str) {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    // Covers the whole `(':>>' | ':>')? name` fragment -- the leading operator token itself
    // (before `name_span`) isn't separately tracked here, matching how this ad hoc prefix shape
    // (distinct from `usage::specialization_clauses`) has always worked.
    let prefix_span = crate::parser::span_from_to(start, input);
    let target_only = prefix.is_some();
    let (input, typing_result) = optional_typings(input)?;
    let (typing_span, typing) = typing_result
        .map(|(span, is_conj, s, sp)| (Some(span.clone()), Some(typing_node(span, is_conj, s, sp))))
        .unwrap_or((None, None));
    let (input, mods1) = feature_modifiers(input)?;
    let (input, value) =
        nom::combinator::opt(preceded(ws_and_comments, crate::parser::feature_value_part))
            .parse(input)?;
    let (input, mods2) = feature_modifiers(input)?;
    let mods = mods1.merge(mods2);
    let (input, body) = attribute_body(input)?;
    let (subsets, redefines) = match prefix {
        Some(MetadataBindingPrefix::Subsets) => (
            Some(crate::parser::usage::subsetting_relationship_node(
                targets,
                SubsettingKind::Subsets,
                prefix_span,
            )),
            None,
        ),
        Some(MetadataBindingPrefix::Redefines) => (
            None,
            Some(crate::parser::usage::subsetting_relationship_node(
                targets,
                SubsettingKind::Redefines,
                prefix_span,
            )),
        ),
        None => (None, None),
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            AttributeUsage {
                // A prefix form (`:>> target` / `:> target`) names a semantic target, not a new
                // declaration. Its spelling lives only in the arena-backed relationship.
                name: if target_only { String::new() } else { name_str },
                short_name: None,
                typing,
                subsets,
                redefines,
                references: None,
                crosses: None,
                intersects: None,
                value,
                body,
                name_span: (!target_only).then_some(name_span),
                typing_span,
                redefines_span: None,
                direction: None,
                multiplicity: mods.multiplicity,
                ordered: mods.ordered,
                nonunique: mods.nonunique,
                is_derived: false,
                usage_prefix: None,
                is_constant: false,
                is_reference: false,
                is_end: false,
                // No visibility prefix exists on this ad hoc `(':>>' | ':>')? name` feature-binding
                // shape (see `prefix_span`'s comment above) -- always `FeatureMembership`, no
                // explicit visibility.
                membership: Membership::feature(None, crate::ast::Span::dummy()),
            },
        ),
    ))
}

/// §6 G26: keyword-less `name = expr;` feature binding, i.e. a [`crate::ast::DefaultReferenceUsage`]
/// that carries a value but no typing clause. Distinct from [`attribute_usage_shorthand`], which
/// requires the `: Type` half; that stricter form stays the one dispatched from part bodies so
/// their existing AST shape is untouched.
///
/// Real usage: `measurement = testVehicle.mass;` inside the perform body of the OMG spec Annex
/// `9-Verification-simplified.sysml`.
///
/// The value is mandatory here deliberately: action bodies have a *targeted* recovery diagnostic
/// for a bare identifier with no value at all (`invalid_bare_identifier_in_action_body`, "did you
/// mean `perform batCap`?" -- `tests/recovery_actions.rs`), which a permissive bare-`name;` arm
/// would silently swallow before that diagnostic ever runs. [`bare_or_valued_feature_binding`] is
/// the value-optional sibling used outside action bodies, where no such diagnostic exists.
/// `{ (doc | nested binding)* }` body for a keyword-less feature binding.
fn feature_binding_body(
    input: Input<'_>,
) -> IResult<Input<'_>, Vec<crate::ast::Node<crate::ast::FeatureBodyElement>>> {
    let (mut input, _) = tag(&b"{"[..]).parse(input)?;
    let mut elements = Vec::new();
    loop {
        let (next, _) = ws_and_comments(input)?;
        input = next;
        if input.fragment().starts_with(b"}") {
            let (input, _) = tag(&b"}"[..]).parse(input)?;
            return Ok((input, elements));
        }
        let member_start = input;
        if let Ok((next, member)) = crate::parser::body::annotating_member(input) {
            elements.push(crate::parser::node_from_to(
                member_start,
                next,
                crate::ast::FeatureBodyElement::Annotating(member),
            ));
            input = next;
            continue;
        }
        let (next, nested) = feature_value_binding(input)?;
        let span = nested.span.clone();
        elements.push(crate::ast::Node::new(
            span,
            crate::ast::FeatureBodyElement::Binding(Box::new(nested)),
        ));
        input = next;
    }
}

pub(crate) fn feature_value_binding(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::DefaultReferenceUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    // Anonymous leading-redefinition binding: `:>> mRef = transformation.source;` (Domain
    // Libraries `VectorCalculations.sysml`). The redefinition target stands in for the name;
    // `specialization_clauses` below captures it.
    let (input, (name_span, name_str)) = if input.fragment().starts_with(b":>>") {
        (input, (crate::ast::Span::dummy(), String::new()))
    } else {
        with_span(name).parse(input)?
    };
    if !name_str.is_empty() && is_reserved_shorthand_starter(&name_str) {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    // GH-87: optional `:>`/`:>>` clause between the name and the value, e.g. `torquePerCurrent
    // :> Quantities::scalarQuantities = ISQ::torque / ISQ::electricCurrent;` (State Space
    // Representation Examples/EVSample.sysml:47). `subsetting()` (called by
    // `specialization_clauses`) already optionally consumes its own trailing `= expr` as part of
    // the `:>` clause itself -- reuse that captured value (via `wrap_bind_expression`, the same
    // helper `attribute_feature_binding` already uses for this exact "subsets target = expr"
    // shape) instead of also requiring a *second*, separate `= expr` afterward.
    let (input, spec) = crate::parser::usage::specialization_clauses(input)?;
    let leading_value = spec.subsets.as_ref().and_then(|(_, v)| v.clone());
    let (input, value) =
        nom::combinator::opt(preceded(ws_and_comments, crate::parser::feature_value_part))
            .parse(input)?;
    let value = value.or(leading_value.map(crate::parser::feature_value::wrap_bind_expression));
    let (input, value) = match value {
        Some(v) => (input, v),
        None => {
            return Err(nom::Err::Error(nom::error::Error::new(
                start,
                nom::error::ErrorKind::Tag,
            )))
        }
    };
    // `;`, or a `{ ... }` body of nested bindings/documentation: `:>> mRef =
    // transformation.target { :>> dimensions = ...; }` (Domain Libraries
    // `VectorCalculations.sysml`).
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = if input.fragment().starts_with(b"{") {
        let (input, elements) = feature_binding_body(input)?;
        (input, Some(elements))
    } else {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        (input, None)
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::DefaultReferenceUsage {
                name: name_str,
                typing: None,
                subsets: spec.subsets.map(|(target, _value)| target),
                redefines: spec.redefines,
                multiplicity: None,
                value: Some(value),
                name_span: Some(name_span),
                typing_span: None,
                membership: Membership::feature(None, crate::ast::Span::dummy()),
                has_feature_keyword: false,
                body,
            },
        ),
    ))
}

/// GH-87: the same keyword-less binding as [`feature_value_binding`], but with the value made
/// optional -- a fully bare `name;` (no type, no value) is a legal `DefaultReferenceUsage` too,
/// e.g. `part def V { m; }` (Simple Tests/AnalysisTest.sysml:4). Kept as a separate function
/// (rather than widening `feature_value_binding` itself) so action bodies' targeted bare-identifier
/// recovery diagnostic keeps firing -- see that function's doc comment.
pub(crate) fn bare_or_valued_feature_binding(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::DefaultReferenceUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (name_span, name_str)) = with_span(name).parse(input)?;
    if is_reserved_shorthand_starter(&name_str) {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    // GH-87: optional `:>`/`:>>` clause between the name and the (also optional) value, e.g.
    // `inflationPressure :> pressure;` (v1 Spec Examples/8.4.1 Wheel Hub Assembly/Wheel
    // Package.sysml:21) -- no `=` value at all, the value is inherited from what it subsets.
    // `subsetting()` (called by `specialization_clauses`) already optionally consumes its own
    // trailing `= expr` as part of the `:>` clause itself (e.g. `torquePerCurrent :>
    // Quantities::scalarQuantities = ISQ::torque / ISQ::electricCurrent;`, EVSample.sysml:47) --
    // reuse that captured value (via `wrap_bind_expression`) instead of requiring a *second*,
    // separate `= expr` afterward.
    let (input, spec) = crate::parser::usage::specialization_clauses(input)?;
    let leading_value = spec.subsets.as_ref().and_then(|(_, v)| v.clone());
    let (input, value) =
        opt(preceded(ws_and_comments, crate::parser::feature_value_part)).parse(input)?;
    let value = value.or(leading_value.map(crate::parser::feature_value::wrap_bind_expression));
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::DefaultReferenceUsage {
                name: name_str,
                typing: None,
                subsets: spec.subsets.map(|(target, _value)| target),
                redefines: spec.redefines,
                multiplicity: None,
                value,
                name_span: Some(name_span),
                typing_span: None,
                membership: Membership::feature(None, crate::ast::Span::dummy()),
                has_feature_keyword: false,
                body: None,
            },
        ),
    ))
}

/// §6 G15: the same feature binding as [`attribute_feature_binding`], but with the `:>>` / `:>`
/// prefix *required*, so it can be offered in body dispatchers whose members are otherwise
/// keyword-led. Without the mandatory prefix the underlying parser would also swallow bare
/// `name : Type;` members and shadow the kind-keyword arms around it.
///
/// Real usage: `:>> mass = m;` and `:>> t = t0 { ... }` inside the snapshot bodies of the OMG
/// spec Annex `6-Individual and Snapshots.sysml`.
pub(crate) fn redefinition_feature_binding(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<AttributeUsage>> {
    let (peek, _) = ws_and_comments(input)?;
    // GH-92.1: bare `redefines <target> = <value>;` (the literal keyword, not just the `:>>`
    // symbol) with no `attribute`/`part` keyword at all, e.g. `redefines mass = 1000 [kg];`
    // (`Mass Roll-up Example/Vehicles.sysml:26`).
    if !peek.fragment().starts_with(b":>") && !starts_with_keyword(peek.fragment(), b"redefines") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    attribute_feature_binding(input)
}

fn attribute_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<AttributeBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        ATTRIBUTE_BODY_STARTERS,
        "attribute body",
        "recovered_attribute_body_element",
    );
    node_from_to(
        start,
        end,
        AttributeBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

/// Attribute body: `;` or `{` AttributeBodyElement* `}`.
pub(crate) fn attribute_body(input: Input<'_>) -> IResult<Input<'_>, AttributeBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let semicolon_start = input;
        let (input, _) = tag(&b";"[..]).parse(semicolon_start)?;
        return Ok((
            input,
            AttributeBody::Semicolon {
                semicolon_span: crate::parser::span::span_from_to(semicolon_start, input),
            },
        ));
    }
    let (input, members) = parse_structured_brace_members(
        input,
        ATTRIBUTE_BODY_STARTERS,
        "attribute body",
        "recovered_attribute_body_element",
        attribute_body_element,
        attribute_body_recovery,
    )?;
    Ok((input, members.into_body()))
}

/// Attribute definition: 'attribute' 'def' name ( ':>' | ':' )? qualified_name? body
///
/// When `disambiguate_from_usage` is true (definition bodies that also accept usages), any
/// declaration without an explicit `def` keyword is left for [`attribute_usage`] — the `def`
/// keyword is the sole discriminator between an `AttributeDef` and an `AttributeUsage`, never
/// inferred from typing, modifiers, or the presence of a value. Package-level attributes pass
/// false, since only definitions are legal there.
pub(crate) fn attribute_def(
    input: Input<'_>,
    disambiguate_from_usage: bool,
) -> IResult<Input<'_>, Node<AttributeDef>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"attribute"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, has_def) = nom::combinator::opt(preceded(tag(&b"def"[..]), ws1)).parse(input)?;
    let has_def = has_def.is_some();
    if disambiguate_from_usage && !has_def {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let ident_start = input;
    let (input, ident) = identification(input)?;
    let name_span = ident
        .name
        .as_ref()
        .map(|_| crate::parser::span_from_to(ident_start, input));
    let name_str = ident
        .name
        .clone()
        .or_else(|| ident.short_name.clone())
        .ok_or_else(|| {
            nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))
        })?;
    let short_name = ident.short_name.clone();
    let (input, typing_result) = optional_typings(input)?;
    let (typing_span, typing) = typing_result
        .map(|(span, is_conj, s, sp)| (Some(span.clone()), Some(typing_node(span, is_conj, s, sp))))
        .unwrap_or((None, None));
    let (input, mods1) = feature_modifiers(input)?;
    let (input, leading_clauses) = specialization_clauses(input)?;
    let leading_subset = leading_clauses.subsets;
    let (typing_span, typing, leading_value) = if typing.is_none() {
        leading_subset
            .map(|(rel, value)| {
                // A leading `:>` subset clause with no separate `:` typing doubles as this
                // attribute def's type (existing behavior); it's a subclassification-shaped
                // target. Reuses the SubsettingRelationship node's own span, which does cover
                // the `:>`/`subsets` fragment (unlike the dummy span this used before subsetting
                // clauses were themselves typed with real spans).
                (
                    Some(rel.span.clone()),
                    Some(typing_relationship_node(
                        rel.span.clone(),
                        TypingKind::Subclassification,
                        false,
                        rel.value.target,
                        crate::ast::TypingSpelling::Operator,
                    )),
                    value,
                )
            })
            .unwrap_or((typing_span, typing, None))
    } else {
        (typing_span, typing, None)
    };
    let (input, value) =
        nom::combinator::opt(preceded(ws_and_comments, crate::parser::feature_value_part))
            .parse(input)?;
    let value = value.or(leading_value.map(crate::parser::feature_value::wrap_bind_expression));
    let value_span = value.as_ref().map(|node| node.span.clone());
    let (input, _) = specialization_clauses(input)?;
    let (input, mods2) = feature_modifiers(input)?;
    let mods = mods1.merge(mods2);
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            AttributeDef {
                name: name_str,
                short_name,
                typing,
                value,
                body,
                name_span,
                typing_span,
                value_span,
                ordered: mods.ordered,
                nonunique: mods.nonunique,
                membership: Membership::owning(visibility, visibility_span),
            },
        ),
    ))
}

pub(crate) fn direction_prefix(input: Input<'_>) -> IResult<Input<'_>, InOut> {
    alt((
        map(preceded(tag(&b"in"[..]), ws1), |_| InOut::In),
        map(preceded(tag(&b"out"[..]), ws1), |_| InOut::Out),
        map(preceded(tag(&b"inout"[..]), ws1), |_| InOut::InOut),
    ))
    .parse(input)
}

/// `in`/`out`/`inout attribute` usage (port def bodies): direction + [`attribute_usage`].
pub(crate) fn directed_attribute_usage(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<AttributeUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, direction) = direction_prefix(input)?;
    let (input, mut usage) = attribute_usage(input)?;
    usage.value.direction = Some(direction);
    Ok((input, node_from_to(start, input, usage.value)))
}

/// Attribute usage:
/// - (`private` | `protected` | `public`)? `attribute` name ( (`:>` | `:`) type )? ( `redefines`
///   qualified_name )? ( '=' value )? body
/// - `attribute :>>` qualified_name ( '=' value )? body
///
/// The visibility prefix is accepted (and discarded, like `attribute_def`'s) so a `def`-less
/// declaration such as `private attribute foo: Type = value;` still dispatches here instead of
/// falling through to an opaque/recovered element. This shape is used in the official Systems
/// Library catalog (e.g. `IntervalScale`'s `private attribute zeroDegreeCelsiusInKelvin: ...`).
pub(crate) fn attribute_usage(input: Input<'_>) -> IResult<Input<'_>, Node<AttributeUsage>> {
    enum AttributeUsageHead {
        Named {
            name_span: crate::ast::Span,
            name: String,
        },
        PrefixRedefines {
            redefines_span: crate::ast::Span,
            redefines: Node<SubsettingRelationship>,
        },
        /// `attribute ::> m = ms.totalMass;` (GH-88.1, `Simple Tests/CalculationTest.sysml:14`):
        /// the `::>` reference-subsetting target stands in for the name, same pattern as
        /// `PrefixRedefines` for `:>>`.
        PrefixReferences {
            references: Node<SubsettingRelationship>,
        },
        /// `attribute :> differencesOf[1] { ... }` (GH-88.5, `Geometry Examples/
        /// CarWithShapeAndCSG.sysml:84`): the `:>` subsets target stands in for the name, same
        /// pattern as `PrefixRedefines`/`PrefixReferences` for `:>>`/`::>`.
        PrefixSubsets {
            subsets: Node<SubsettingRelationship>,
            subsets_value: Option<Node<crate::ast::Expression>>,
        },
    }

    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    // `UnextendedUsagePrefix : Usage = EndUsagePrefix | BasicUsagePrefix` (BNF §8.2.2.6.2):
    // `end` and the `RefPrefix` keywords below are mutually exclusive alternatives, not
    // combinable -- a usage is either an `EndUsagePrefix` (`end` + optional cross-feature
    // member, not modeled here since attribute usages have no cross-feature member syntax) or a
    // `RefPrefix` (`derived`/`constant`). Distinct from the unrelated `EndDecl`/`end_decl`
    // construct (a separate named-connector-end declaration, `end name : Type;`).
    let (input, is_end) = nom::combinator::opt(preceded(tag(&b"end"[..]), ws1)).parse(input)?;
    let is_end = is_end.is_some();
    // `BasicUsagePrefix = RefPrefix ('ref')?`, `RefPrefix = 'derived'? ('abstract'|'variation')?
    // 'constant'?` (BNF §8.2.2.6.2) -- usage-only prefix keywords, no `Definition` equivalent.
    // GH-88.2/88.3: `abstract`/`variation`/`ref` are all legal here per the BNF (confirmed real
    // usage: `derived constant ref attribute y :> x;`, Simple Tests/PartTest.sysml:9; `abstract
    // attribute minMass :> ISQ::mass;`, Mass Roll-up Example/MassRollup.sysml:21) -- a previous
    // assumption that `abstract`/`variation` "aren't legal on an attribute usage" was incorrect.
    // Skipped entirely when `end` already matched, since the two are alternatives.
    let (input, prefix, is_reference) = if is_end {
        (input, crate::parser::usage::RefPrefix::default(), false)
    } else {
        let (input, prefix) = crate::parser::usage::ref_prefix(input)?;
        let (input, is_reference) =
            nom::combinator::opt(preceded(tag(&b"ref"[..]), ws1)).parse(input)?;
        (input, prefix, is_reference.is_some())
    };
    let crate::parser::usage::RefPrefix {
        direction: prefix_direction,
        is_derived,
        usage_prefix,
        is_constant,
    } = prefix;
    let (input, _) = tag(&b"attribute"[..]).parse(input)?;
    // SysML allows anonymous attribute usages: `attribute: Real;` (Identification may be empty).
    let (after_kw, _) = ws_and_comments(input)?;
    let input = if (after_kw.fragment().starts_with(b":")
        && !after_kw.fragment().starts_with(b":>")
        && !after_kw.fragment().starts_with(b":>>"))
        || starts_with_keyword(after_kw.fragment(), b"defined")
    {
        after_kw
    } else {
        let (input, _) = ws1(input)?;
        input
    };
    // `Identification`'s `( '<' ShortName '>' )?` half (BNF §8.2.2.2), shared with `attribute_def`
    // via `identification` -- confirmed real usage in the OMG Geometry domain library's
    // `VehicleGeometryAndCoordinateFrames.sysml` (`attribute <wcf> wheelCoordinateFrame : ...`,
    // `attribute <lbpr> lugBoltPlacementRadius :>> radius ...`), previously unparseable here.
    let (input, short_name) = short_name_prefix(input)?;
    // Consume (not just peek) any whitespace/comments between the short name's closing `>` and
    // whatever follows -- `short_name_prefix` only skips ws *inside* the `< ... >` brackets, so a
    // short name leaves fresh un-consumed whitespace here that wasn't present in the no-short-name
    // path (where `ws1` above already fully consumed it).
    let (input, _) = ws_and_comments(input)?;
    let peek = input;
    let (input, usage_head) =
        if let Ok((input, references)) = crate::parser::usage::reference_subsetting(peek) {
            // Checked before the generic `:`-anonymous branch below: `::>` starts with `:` but not
            // `:>`/`:>>`, so without this it would wrongly fall into that branch and leave `::>`
            // itself unconsumed.
            (input, AttributeUsageHead::PrefixReferences { references })
        } else if (peek.fragment().starts_with(b":")
            && !peek.fragment().starts_with(b":>")
            && !peek.fragment().starts_with(b":>>"))
            || starts_with_keyword(peek.fragment(), b"defined")
        {
            (
                input,
                AttributeUsageHead::Named {
                    name_span: crate::ast::Span::dummy(),
                    name: String::new(),
                },
            )
        } else {
            alt((
                map(
                    preceded(ws_and_comments, prefix_redefinition_target),
                    |(redefines_span, redefines)| AttributeUsageHead::PrefixRedefines {
                        redefines_span,
                        redefines,
                    },
                ),
                // GH-88.5: `attribute :> differencesOf[1] { ... }` -- tried after `:>>` above (whose
                // own tag would otherwise be partially matched by this one) since `:>` is a strict
                // prefix of `:>>`.
                map(
                    preceded(ws_and_comments, crate::parser::usage::subsetting),
                    |(subsets, subsets_value)| AttributeUsageHead::PrefixSubsets {
                        subsets,
                        subsets_value,
                    },
                ),
                map(with_span(name), |(name_span, name)| {
                    AttributeUsageHead::Named { name_span, name }
                }),
            ))
            .parse(input)?
        };
    let (
        input,
        name_span,
        name_str,
        typing_span,
        typing,
        redefines_span,
        redefines,
        head_references,
        head_subsets,
        head_subsets_value,
        mods0,
    ) = match usage_head {
        AttributeUsageHead::PrefixRedefines {
            redefines_span,
            redefines,
        } => {
            let (input, pre_typing_mods) = feature_modifiers(input)?;
            let (input, typing_result) = optional_typings(input)?;
            let (typing_span, typing) = typing_result
                .map(|(span, is_conj, s, sp)| {
                    (Some(span.clone()), Some(typing_node(span, is_conj, s, sp)))
                })
                .unwrap_or((None, None));
            let (input, mods0) = feature_modifiers(input)?;
            let mods0 = pre_typing_mods.merge(mods0);
            (
                input,
                None,
                String::new(),
                typing_span,
                typing,
                Some(redefines_span),
                Some(redefines),
                None,
                None,
                None,
                mods0,
            )
        }
        AttributeUsageHead::PrefixReferences { references } => {
            let (input, pre_typing_mods) = feature_modifiers(input)?;
            let (input, typing_result) = optional_typings(input)?;
            let (typing_span, typing) = typing_result
                .map(|(span, is_conj, s, sp)| {
                    (Some(span.clone()), Some(typing_node(span, is_conj, s, sp)))
                })
                .unwrap_or((None, None));
            let (input, mods0) = feature_modifiers(input)?;
            let mods0 = pre_typing_mods.merge(mods0);
            (
                input,
                None,
                String::new(),
                typing_span,
                typing,
                None,
                None,
                Some(references),
                None,
                None,
                mods0,
            )
        }
        AttributeUsageHead::PrefixSubsets {
            subsets,
            subsets_value,
        } => {
            let (input, pre_typing_mods) = feature_modifiers(input)?;
            let (input, typing_result) = optional_typings(input)?;
            let (typing_span, typing) = typing_result
                .map(|(span, is_conj, s, sp)| {
                    (Some(span.clone()), Some(typing_node(span, is_conj, s, sp)))
                })
                .unwrap_or((None, None));
            let (input, mods0) = feature_modifiers(input)?;
            let mods0 = pre_typing_mods.merge(mods0);
            (
                input,
                None,
                String::new(),
                typing_span,
                typing,
                None,
                None,
                None,
                Some(subsets),
                subsets_value,
                mods0,
            )
        }
        AttributeUsageHead::Named { name_span, name } => {
            // §6 G10: the multiplicity may precede the typing clause -- `attribute
            // occurs[0..1]: Real;` (OMG spec Annex `14c-Language Extensions.sysml`). Only the
            // trailing position (`attribute a : Real[0..1];`) was accepted before, so the
            // leading one left `: Real` unconsumed and the member fell through to recovery.
            // `FeatureModifiers::merge` is first-wins, so reading both positions is safe.
            let (input, pre_typing_mods) = feature_modifiers(input)?;
            let (input, typing_result) = optional_typings(input)?;
            let (typing_span, typing) = typing_result
                .map(|(span, is_conj, s, sp)| {
                    (Some(span.clone()), Some(typing_node(span, is_conj, s, sp)))
                })
                .unwrap_or((None, None));
            let (input, mods0) = feature_modifiers(input)?;
            let mods0 = pre_typing_mods.merge(mods0);
            (
                input,
                Some(name_span),
                name,
                typing_span,
                typing,
                None,
                None,
                None,
                None,
                None,
                mods0,
            )
        }
    };
    let (input, leading_clauses) = specialization_clauses(input)?;
    let (input, mods1) = feature_modifiers(input)?;
    let leading_subsets_value = leading_clauses
        .subsets
        .as_ref()
        .and_then(|(_, value)| value.clone())
        .or(head_subsets_value);
    let (input, value) =
        nom::combinator::opt(preceded(ws_and_comments, crate::parser::feature_value_part))
            .parse(input)?;
    let (input, trailing_clauses) = specialization_clauses(input)?;
    let (input, mods2) = feature_modifiers(input)?;
    let mods = mods0.merge(mods1).merge(mods2);
    let redefines = trailing_clauses
        .redefines
        .or(leading_clauses.redefines)
        .or(redefines);
    let subsets = trailing_clauses
        .subsets
        .or(leading_clauses.subsets)
        .map(|(target, _)| target)
        .or(head_subsets);
    let references = trailing_clauses
        .references
        .or(leading_clauses.references)
        .or(head_references);
    let crosses = trailing_clauses.crosses.or(leading_clauses.crosses);
    let intersects = trailing_clauses.intersects.or(leading_clauses.intersects);
    let value =
        value.or(leading_subsets_value.map(crate::parser::feature_value::wrap_bind_expression));
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            AttributeUsage {
                name: name_str,
                short_name,
                typing,
                subsets,
                redefines,
                references,
                crosses,
                intersects,
                value,
                body,
                name_span,
                typing_span,
                redefines_span,
                // `RefPrefix`'s direction slot. `directed_attribute_usage` still overwrites this
                // with the direction it consumed itself, which is the same value.
                direction: prefix_direction,
                multiplicity: mods.multiplicity,
                ordered: mods.ordered,
                nonunique: mods.nonunique,
                is_derived,
                usage_prefix,
                is_constant,
                is_reference,
                is_end,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// Metadata usage body binding: `ref`? (`:>` | `:>>`)? name (`:` type)? (`=` value)? `;`
///
/// Covers §7.27.2 forms such as `approved = true;`, `ref :>> approved = true;`,
/// `:> annotatedElement : Type;`, and `:>> baseType = expr meta Type;`.
fn metadata_binding(input: Input<'_>) -> IResult<Input<'_>, Node<AttributeUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) =
        nom::combinator::opt(preceded(ws_and_comments, tag(&b"ref"[..]))).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, prefix) = nom::combinator::opt(alt((
        map(preceded(ws_and_comments, tag(&b":>>"[..])), |_| {
            MetadataBindingPrefix::Redefines
        }),
        map(preceded(ws_and_comments, subset_operator), |_| {
            MetadataBindingPrefix::Subsets
        }),
    )))
    .parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (_, (name_span, name_str)) = with_span(name).parse(input)?;
    // The prefixed forms take the same comma-separated multi-target list as every other
    // `:>>`/`:>` clause (`:>> A::x, B::y { ... }`, Quantities and Units `SI.kerml`; spec42
    // Gap 49b); the unprefixed binding keeps its single name-reference.
    let (input, targets) = if prefix.is_some() {
        crate::parser::usage::specialization_targets(input)?
    } else {
        let (input, target) = qualified_reference(input)?;
        (input, vec![target])
    };
    if is_reserved_shorthand_starter(&name_str) {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    // See attribute_feature_binding's identical comment: covers the whole
    // `(':>>' | ':>')? name` fragment, not just the operator token.
    let prefix_span = crate::parser::span_from_to(start, input);
    let (input, typing_result) = optional_typings(input)?;
    let (typing_span, typing) = typing_result
        .map(|(span, is_conj, s, sp)| (Some(span.clone()), Some(typing_node(span, is_conj, s, sp))))
        .unwrap_or((None, None));
    let (input, mods) = feature_modifiers(input)?;
    let (input, value) =
        nom::combinator::opt(preceded(ws_and_comments, crate::parser::feature_value_part))
            .parse(input)?;
    let (semicolon_start, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b";"[..]).parse(semicolon_start)?;
    let semicolon_span = crate::parser::span::span_from_to(semicolon_start, input);
    let (subsets, redefines) = match prefix {
        Some(MetadataBindingPrefix::Subsets) => (
            Some(crate::parser::usage::subsetting_relationship_node(
                targets,
                SubsettingKind::Subsets,
                prefix_span,
            )),
            None,
        ),
        Some(MetadataBindingPrefix::Redefines) => (
            None,
            Some(crate::parser::usage::subsetting_relationship_node(
                targets,
                SubsettingKind::Redefines,
                prefix_span,
            )),
        ),
        None => (None, None),
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            AttributeUsage {
                name: name_str,
                short_name: None,
                typing,
                subsets,
                redefines,
                references: None,
                crosses: None,
                intersects: None,
                value,
                body: AttributeBody::Semicolon { semicolon_span },
                name_span: Some(name_span),
                typing_span,
                redefines_span: None,
                direction: None,
                multiplicity: mods.multiplicity,
                ordered: mods.ordered,
                nonunique: mods.nonunique,
                is_derived: false,
                usage_prefix: None,
                is_constant: false,
                is_reference: false,
                is_end: false,
                // No visibility prefix on a metadata binding shape either (see
                // `attribute_feature_binding`'s identical note above).
                membership: Membership::feature(None, crate::ast::Span::dummy()),
            },
        ),
    ))
}

fn metadata_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<AttributeBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(
            crate::parser::body::annotating_member,
            AttributeBodyElement::Annotating,
        ),
        map(
            |i| attribute_def(i, true),
            AttributeBodyElement::AttributeDef,
        ),
        map(attribute_usage, AttributeBodyElement::AttributeUsage),
        map(metadata_binding, AttributeBodyElement::AttributeUsage),
        // The same structured members `attribute_body_element` already dispatches (spec42
        // Gap 40): previously this narrower grammar dropped e.g. `ref self : MetadataItem
        // redefines Metaobject::self, Item::self;` (Systems Library `Metadata.sysml`) to
        // opaque capture. Before the opaque fallback so they get structured nodes.
        map(
            crate::parser::connector::ref_decl,
            AttributeBodyElement::RefDecl,
        ),
        map(crate::parser::part::part_usage, |n| {
            AttributeBodyElement::PartUsage(Box::new(n))
        }),
        map(crate::parser::item::item_usage, |n| {
            AttributeBodyElement::ItemUsage(Box::new(n))
        }),
        map(crate::parser::part::connect_, AttributeBodyElement::Connect),
        map(
            crate::parser::occurrence_body::assert_constraint_member,
            AttributeBodyElement::AssertConstraint,
        ),
        map(
            |i| {
                crate::parser::recovery::unsupported_member(
                    i,
                    METADATA_OPAQUE_STARTERS,
                    "metadata body",
                )
            },
            AttributeBodyElement::Unsupported,
        ),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn metadata_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<AttributeBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        METADATA_BODY_STARTERS,
        "metadata body",
        "recovered_metadata_body_element",
    );
    node_from_to(
        start,
        end,
        AttributeBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

/// Metadata annotation/usage body: `;` or `{` members `}` (structured attribute bindings).
pub(crate) fn metadata_body(input: Input<'_>) -> IResult<Input<'_>, AttributeBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let semicolon_start = input;
        let (input, _) = tag(&b";"[..]).parse(semicolon_start)?;
        return Ok((
            input,
            AttributeBody::Semicolon {
                semicolon_span: crate::parser::span::span_from_to(semicolon_start, input),
            },
        ));
    }
    let (input, members) = parse_structured_brace_members(
        input,
        METADATA_BODY_STARTERS,
        "metadata body",
        "recovered_metadata_body_element",
        metadata_body_element,
        metadata_body_recovery,
    )?;
    Ok((input, members.into_body()))
}

/// SysML `DefaultReferenceUsage` shorthand: bare `name : Type (= value)?;` without the
/// `attribute` keyword (commonly used inside part bodies).
///
/// Supports:
/// - `name : Type ;`
/// - `name : Type = expr ;`
/// - `:>> name : Type = expr ;` (leading `:>>` ignored; treated as a usage)
pub(crate) fn attribute_usage_shorthand(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::DefaultReferenceUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) =
        nom::combinator::opt(preceded(ws_and_comments, tag(&b":>>"[..]))).parse(input)?;
    let (input, (name_span, name_str)) = with_span(name).parse(input)?;
    if is_reserved_shorthand_starter(&name_str) {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, (typing_span, is_conjugated, targets, spelling)) = typings(input)?;
    let typing = Some(typing_node(
        typing_span.clone(),
        is_conjugated,
        targets,
        spelling,
    ));
    // Keep shorthand values on the shared expression path so precedence/parentheses are preserved.
    let (input, value) =
        nom::combinator::opt(preceded(ws_and_comments, crate::parser::feature_value_part))
            .parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::DefaultReferenceUsage {
                name: name_str,
                typing,
                // Leading `:>>` (before the name) is a different shape from GH-87's `name :>
                // Target` (target after the name) -- see this function's doc comment; that form's
                // target isn't captured here today, unchanged from before GH-87.
                subsets: None,
                redefines: None,
                value,
                multiplicity: None,
                name_span: Some(name_span),
                typing_span: Some(typing_span),
                // No visibility prefix on the no-keyword shorthand form.
                membership: Membership::feature(None, crate::ast::Span::dummy()),
                has_feature_keyword: false,
                body: None,
            },
        ),
    ))
}

#[cfg(test)]
mod attribute_body_tests {
    use super::*;
    use crate::ast::{Expression, QualifiedReferenceId};
    use crate::parser::Input;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    fn target_texts(source: Input<'_>, targets: &[QualifiedReferenceId]) -> Vec<String> {
        targets
            .iter()
            .map(|id| crate::parser::usage::reference_text(source, *id).expect("reference text"))
            .collect()
    }

    /// Spec42 Gap 49b: the bare `:>>`/`:>` shorthand (no `attribute` keyword) accepts the same
    /// comma-separated multi-target list as every other redefinition clause (`SI.kerml`'s
    /// `kelvin` member).
    #[test]
    fn bare_redefinition_shorthand_accepts_multiple_targets() {
        let (rest, node) = attribute_feature_binding(input(":>> A::x::factors, B::y::factors;"))
            .expect("bare multi-target redefinition");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let redefines = node.value.redefines.as_ref().expect("redefines clause");
        assert_eq!(redefines.value.target.len(), 2);
        assert!(node.value.name.is_empty());
    }

    /// Spec42 Gap 40: `metadata def` bodies dispatch the same structured members as other
    /// attribute-shaped bodies instead of dropping them to opaque capture (`ref self :
    /// MetadataItem redefines Metaobject::self, Item::self;`, Systems Library `Metadata.sysml`).
    #[test]
    fn metadata_body_dispatches_structured_members() {
        let (rest, body) = metadata_body(input(
            "{ ref self : M redefines Metaobject::self, Item::self; item picture : Picture; }",
        ))
        .expect("metadata body");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let AttributeBody::Brace { elements, .. } = body else {
            panic!("expected brace body");
        };
        assert!(matches!(
            elements[0].value,
            AttributeBodyElement::RefDecl(_)
        ));
        assert!(matches!(
            elements[1].value,
            AttributeBodyElement::ItemUsage(_)
        ));
    }

    /// Spec42 Gap 38: KerML classifier-keyword declarations (other than `class`, which stays on
    /// `class_def`) nested inside an attribute-shaped body dispatch to the typed
    /// `KermlClassifierDecl` production.
    #[test]
    fn attribute_body_dispatches_nested_kerml_classifiers() {
        let (rest, node) =
            attribute_body_element(input("classifier C1;")).expect("nested classifier");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let AttributeBodyElement::KermlClassifier(decl) = node.value else {
            panic!("expected KermlClassifier");
        };
        assert_eq!(
            decl.value.keyword,
            crate::ast::KermlClassifierKeyword::Classifier
        );
        assert_eq!(decl.value.identification.name.as_deref(), Some("C1"));
    }

    #[test]
    fn attribute_usage_captures_intersects() {
        let source = input("attribute reading : Weight intersects a, b;");
        let (rest, node) = attribute_usage(source).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .intersects
                .as_ref()
                .map(|n| target_texts(source, &n.value.target)),
            Some(vec!["a".to_string(), "b".to_string()])
        );
    }

    #[test]
    fn feature_binding_parses_unit_conversion_prefix_form() {
        let text =
            ":>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; }";
        let source = input(text);
        let (rest, node) = attribute_feature_binding(source).expect("feature binding");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .redefines
                .as_ref()
                .map(|n| target_texts(source, &n.value.target)),
            Some(vec!["unitConversion".to_string()])
        );
        assert_eq!(
            node.value
                .typing
                .as_ref()
                .map(|n| target_texts(source, &n.value.target)),
            Some(vec!["ConversionByPrefix".to_string()])
        );
        let AttributeBody::Brace { elements, .. } = &node.value.body else {
            panic!("expected brace body");
        };
        assert_eq!(elements.len(), 2);
    }

    #[test]
    fn attribute_usage_accepts_leading_visibility_modifier() {
        let text = "private attribute zeroDegreeCelsiusInKelvin: ThermodynamicTemperatureValue = 273.15 [K];";
        let source = input(text);
        let (rest, node) = attribute_usage(source).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "zeroDegreeCelsiusInKelvin");
        assert_eq!(
            node.value
                .typing
                .as_ref()
                .map(|n| target_texts(source, &n.value.target)),
            Some(vec!["ThermodynamicTemperatureValue".to_string()])
        );
        assert!(node.value.value.is_some());
    }

    // --- parser work item 4b: Membership captures the previously-discarded visibility prefix ---

    #[test]
    fn attribute_usage_visibility_prefix_is_captured_on_membership() {
        let text = "private attribute zeroDegreeCelsiusInKelvin: ThermodynamicTemperatureValue = 273.15 [K];";
        let (_, node) = attribute_usage(input(text)).expect("attribute usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn attribute_usage_without_visibility_prefix_has_no_membership_visibility() {
        let text = "attribute mass: Real;";
        let (_, node) = attribute_usage(input(text)).expect("attribute usage");
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn attribute_def_visibility_prefix_is_captured_on_membership() {
        let text = "protected attribute def Samples: Real;";
        let (rest, node) = attribute_def(input(text), false).expect("attribute def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn attribute_def_public_visibility_prefix_is_captured_on_membership() {
        let text = "public attribute def Samples: Real;";
        let (rest, node) = attribute_def(input(text), false).expect("attribute def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
    }

    #[test]
    fn attribute_body_element_dispatches_visibility_prefixed_declaration_to_usage_not_def() {
        let text = "private attribute zeroDegreeCelsiusInKelvin: ThermodynamicTemperatureValue = 273.15 [K];";
        let (_, node) = attribute_body_element(input(text)).expect("attribute body element");
        match node.value {
            AttributeBodyElement::AttributeUsage(usage) => {
                assert_eq!(usage.value.name, "zeroDegreeCelsiusInKelvin");
            }
            other => panic!("expected AttributeUsage, got {other:?}"),
        }
    }

    // --- PAR-003b item 1: ordered/nonunique/derived/constant retained as typed fields ---

    #[test]
    fn attribute_usage_retains_ordered_modifier() {
        let text = "attribute readings: Real[0..*] ordered;";
        let (rest, node) = attribute_usage(input(text)).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.ordered);
        assert!(!node.value.nonunique);
        let multiplicity = node.value.multiplicity.expect("multiplicity retained");
        assert!(matches!(
            multiplicity.value.lower.as_deref().map(|node| &node.value),
            Some(Expression::LiteralInteger(0))
        ));
        assert!(multiplicity.value.upper.is_none());
    }

    #[test]
    fn attribute_usage_retains_nonunique_modifier() {
        let text = "attribute tags: String[0..*] nonunique;";
        let (rest, node) = attribute_usage(input(text)).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.nonunique);
        assert!(!node.value.ordered);
        let multiplicity = node.value.multiplicity.expect("multiplicity retained");
        assert!(matches!(
            multiplicity.value.lower.as_deref().map(|node| &node.value),
            Some(Expression::LiteralInteger(0))
        ));
        assert!(multiplicity.value.upper.is_none());
    }

    #[test]
    fn attribute_usage_retains_multiplicity_without_type() {
        let text = "attribute a [0..1] ordered;";
        let (rest, node) = attribute_usage(input(text)).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.ordered);
        let multiplicity = node.value.multiplicity.expect("multiplicity retained");
        assert!(matches!(
            multiplicity.value.lower.as_deref().map(|node| &node.value),
            Some(Expression::LiteralInteger(0))
        ));
        assert!(matches!(
            multiplicity.value.upper.as_deref().map(|node| &node.value),
            Some(Expression::LiteralInteger(1))
        ));
    }

    #[test]
    fn attribute_def_retains_ordered_and_nonunique_modifiers() {
        let text = "attribute def Samples :> Real[0..*] ordered nonunique;";
        let (rest, node) = attribute_def(input(text), false).expect("attribute def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.ordered);
        assert!(node.value.nonunique);
    }

    #[test]
    fn attribute_usage_retains_derived_prefix() {
        let text = "derived attribute total: Real = a + b;";
        let (rest, node) = attribute_usage(input(text)).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.is_derived);
        assert!(!node.value.is_constant);
        assert_eq!(node.value.name, "total");
    }

    #[test]
    fn attribute_usage_retains_constant_prefix() {
        let text = "constant attribute pi: Real = 3.14159;";
        let (rest, node) = attribute_usage(input(text)).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.is_constant);
        assert!(!node.value.is_derived);
        assert_eq!(node.value.name, "pi");
    }

    #[test]
    fn attribute_usage_without_modifiers_defaults_to_false() {
        let text = "attribute mass: Real;";
        let (rest, node) = attribute_usage(input(text)).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.multiplicity.is_none());
        assert!(!node.value.ordered);
        assert!(!node.value.nonunique);
        assert!(!node.value.is_derived);
        assert!(!node.value.is_constant);
    }

    // --- gaps-doc item 3: `end` (`EndUsagePrefix`) retained as a typed field ---

    #[test]
    fn attribute_usage_retains_end_prefix() {
        let text = "end attribute mass: Real;";
        let (rest, node) = attribute_usage(input(text)).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.is_end);
        // `end` and `derived`/`constant` are mutually exclusive alternatives per BNF
        // `UnextendedUsagePrefix : Usage = EndUsagePrefix | BasicUsagePrefix`.
        assert!(!node.value.is_derived);
        assert!(!node.value.is_constant);
        assert_eq!(node.value.name, "mass");
    }

    #[test]
    fn attribute_usage_without_end_prefix_defaults_to_false() {
        let text = "attribute mass: Real;";
        let (rest, node) = attribute_usage(input(text)).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(!node.value.is_end);
    }

    // --- short-name (`<shortName>`) support on `attribute_usage`, confirmed real-usage gap: the
    // OMG Geometry domain library's `VehicleGeometryAndCoordinateFrames.sysml` example contains
    // `attribute <wcf> wheelCoordinateFrame : CoordinateFrame;` and
    // `attribute <lbpr> lugBoltPlacementRadius :>> radius default 60 [mm];`, both of which
    // previously failed with `recovered_part_def_body_element` -- `attribute_usage` had no
    // `<shortName>` handling at all, unlike `attribute_def` (shared `Identification` BNF
    // production, §8.2.2.2).

    #[test]
    fn attribute_usage_captures_short_name() {
        let text = "attribute <wcf> wheelCoordinateFrame : CoordinateFrame;";
        let source = input(text);
        let (rest, node) = attribute_usage(source).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.short_name.as_deref(), Some("wcf"));
        assert_eq!(node.value.name, "wheelCoordinateFrame");
        assert_eq!(
            node.value
                .typing
                .as_ref()
                .map(|n| target_texts(source, &n.value.target)),
            Some(vec!["CoordinateFrame".to_string()])
        );
    }

    #[test]
    fn attribute_usage_captures_short_name_with_redefines_and_value() {
        let text = "attribute <lbpr> lugBoltPlacementRadius :>> radius default 60 [mm];";
        let source = input(text);
        let (rest, node) = attribute_usage(source).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.short_name.as_deref(), Some("lbpr"));
        assert_eq!(node.value.name, "lugBoltPlacementRadius");
        assert_eq!(
            node.value
                .redefines
                .as_ref()
                .map(|n| target_texts(source, &n.value.target)),
            Some(vec!["radius".to_string()])
        );
        assert!(node.value.value.is_some());
    }

    #[test]
    fn attribute_usage_without_short_name_has_none() {
        let text = "attribute mass: Real;";
        let (_, node) = attribute_usage(input(text)).expect("attribute usage");
        assert_eq!(node.value.short_name, None);
    }

    /// `AttributeBody` is shared with `item def`/`item` usage bodies (OMG spec Annex
    /// `14c-Language Extensions.sysml`'s FMEA library example uses both `connect a to b;` and
    /// `#tag`-prefixed members extensively inside `item def` bodies) -- previously neither was
    /// recognized here at all.
    #[test]
    fn attribute_body_accepts_standalone_connect() {
        let (rest, node) =
            attribute_body_element(input("connect a to b;")).expect("standalone connect");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, AttributeBodyElement::Connect(_)));
    }

    #[test]
    fn attribute_body_accepts_bare_metadata_tag() {
        let (rest, node) = attribute_body_element(input("#Tag;")).expect("bare metadata tag");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            AttributeBodyElement::MetadataKeywordUsage(_)
        ));
    }

    #[test]
    fn attribute_body_accepts_metadata_tag_prefixing_a_connect() {
        let (rest, node) = attribute_body_element(input("#prevention connect a to b;"))
            .expect("prefix-form metadata tag");
        // Only the tag is consumed; the prefixed connect is left for the next element.
        assert_eq!(rest.fragment(), b"connect a to b;");
        assert!(matches!(
            node.value,
            AttributeBodyElement::MetadataKeywordUsage(_)
        ));
    }

    #[test]
    fn attribute_body_accepts_assert_constraint() {
        let (rest, node) = attribute_body_element(input(
            "assert constraint { round(e * 1E20) == 271828182845904523536.0 }",
        ))
        .expect("assert constraint in attribute body");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            AttributeBodyElement::AssertConstraint(_)
        ));
    }

    /// Attribute / item bodies own `ref` members (validation 15_11 / 17a).
    #[test]
    fn attribute_body_accepts_ref_part_redefines() {
        let (rest, node) =
            attribute_body_element(input("ref part :>> elements: Person;")).expect("ref part :>>");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, AttributeBodyElement::RefDecl(_)));
    }

    #[test]
    fn attribute_body_accepts_ref_part_name() {
        let (rest, node) =
            attribute_body_element(input("ref part subscriber;")).expect("ref part name");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match &node.value {
            AttributeBodyElement::RefDecl(r) => assert_eq!(r.value.name, "subscriber"),
            other => panic!("expected RefDecl, got {other:?}"),
        }
    }
}
