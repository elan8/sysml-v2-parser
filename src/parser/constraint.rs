use crate::ast::{
    CalcDef, CalcDefBody, CalcDefBodyElement, CalcUsage, ConstraintDef, ConstraintDefBody,
    ConstraintDefBodyElement, ConstraintUsage, Expression, Membership, Node, ParseErrorNode,
    ReturnDecl,
};
use crate::parser::action::in_out_decl;
use crate::parser::body::parse_structured_brace_members;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::expression;
use crate::parser::lex::{
    identification, name, qualified_reference, skip_statement_or_block, starts_with_any_keyword,
    starts_with_keyword, visibility_prefix, ws1, ws_and_comments, CALC_DEF_BODY_STARTERS,
    CONSTRAINT_DEF_BODY_STARTERS,
};
use crate::parser::usage::{feature_usage_header, redefinition};
use crate::parser::Input;
use crate::parser::{build_recovery_error_node_from_span, node_from_to};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::{IResult, Parser};

/// `def` is required: package level dispatches [`constraint_usage`] right after this parser in
/// the same `alt(...)` (mirroring `case_def`/`case_usage`, `requirement_def`/`requirement_usage`,
/// etc. -- see `definition_prefix.rs`'s module doc), so a `def`-less declaration is left for
/// `constraint_usage` instead of being misclassified as a definition (the PAR-001 bug class).
///
/// **History:** this was `Optional` until parser 0.40.0 specifically because no
/// `constraint_usage` parser existed yet to catch the standard library's bare, `def`-less
/// `constraint` usages at namespace level (e.g. `abstract constraint constraintChecks:
/// ConstraintCheck[0..*] nonunique :> booleanEvaluations { ... }` in
/// `Systems Library/Constraints.sysml`) -- see CHANGELOG 0.33.0 for an earlier, unsafe attempt
/// that made this `.def_required()` with no usage-parser fallback, which broke the full
/// `SYSML_V2_RELEASE_DIR` validation gate. [`constraint_usage`] below (built on
/// [`crate::parser::usage::feature_usage_header`], the same shared header parser
/// `allocation_usage`/`flow_usage`/`requirement_usage`/etc. already use for the identical
/// abstract/multiplicity/`nonunique`/subsetting shape) now covers that real-library form, so
/// requiring `def` here is safe -- verified against the full validation suite.
pub(crate) fn constraint_def(input: Input<'_>) -> IResult<Input<'_>, Node<ConstraintDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"constraint")
            .with_captured_visibility()
            .def_required(),
    )?;
    let (input, body) = constraint_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ConstraintDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

/// Constraint usage: `constraint` name (feature usage header)? body (SysML/KerML: a bare
/// `constraint` feature, package-level only). Mirrors `allocation_usage`'s shape (a baseline
/// `PartUsage`-family usage, not the richer `attribute_usage`), reusing `constraint_def_body` for
/// the body and `feature_usage_header` for typing/subsetting/multiplicity/`nonunique` so the
/// Systems Library's real bare forms all still parse: `constraint mc : MassConstraint4 { ... }`
/// (typing only), `abstract constraint constraintChecks: ConstraintCheck[0..*] nonunique :>
/// booleanEvaluations { ... }` (typing + trailing multiplicity + modifier + subsetting),
/// `abstract constraint assertedConstraintChecks :> constraintChecks, trueEvaluations { ... }`
/// (subsetting only, no typing, multi-target), and `constraint assumptions :>>
/// RequirementConstraintCheck::assumptions;` (redefinition with a qualified feature-chain
/// target). `abstract` is accepted and discarded, matching `ConstraintDef` itself, which has no
/// `is_abstract` field either.
pub(crate) fn constraint_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ConstraintUsage>> {
    let start = input;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"constraint"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, header) = feature_usage_header(input)?;
    let (input, body) = constraint_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ConstraintUsage {
                name: name_str,
                type_name: header.type_reference,
                subsets: header.subsets,
                redefines: header.redefines,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

fn constraint_def_body(input: Input<'_>) -> IResult<Input<'_>, ConstraintDefBody> {
    let (input, body) = structured_constraint_body(input)?;
    let body = match body {
        StructuredConstraintBody::Semicolon => ConstraintDefBody::Semicolon,
        StructuredConstraintBody::Brace { elements } => ConstraintDefBody::Brace { elements },
    };
    Ok((input, body))
}

pub(crate) enum StructuredConstraintBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ConstraintDefBodyElement>>,
    },
}

fn constraint_body_recovery_element(
    start: Input<'_>,
    end: Input<'_>,
) -> Node<ConstraintDefBodyElement> {
    if starts_with_any_keyword(start.fragment(), CONSTRAINT_DEF_BODY_STARTERS) {
        let recovery = build_recovery_error_node_from_span(
            start,
            end,
            CONSTRAINT_DEF_BODY_STARTERS,
            "constraint body",
            "recovered_constraint_body_element",
        );
        let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
        return node_from_to(start, end, ConstraintDefBodyElement::Error(node));
    }
    let preview = String::from_utf8_lossy(&start.fragment()[..start.fragment().len().min(120)])
        .trim()
        .to_string();
    node_from_to(start, end, ConstraintDefBodyElement::Other(preview))
}

pub(crate) fn structured_constraint_body(
    input: Input<'_>,
) -> IResult<Input<'_>, StructuredConstraintBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, StructuredConstraintBody::Semicolon));
    }
    let (input, elements) = parse_structured_brace_members(
        input,
        CONSTRAINT_DEF_BODY_STARTERS,
        "constraint body",
        "recovered_constraint_body_element",
        constraint_def_body_element,
        constraint_body_recovery_element,
    )?;
    Ok((input, StructuredConstraintBody::Brace { elements }))
}

pub(crate) fn constraint_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ConstraintDefBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = if starts_with_keyword(input.fragment(), b"doc") {
        map(
            crate::parser::requirement::doc_comment,
            ConstraintDefBodyElement::Doc,
        )
        .parse(input)?
    } else if input.fragment().starts_with(b"@") {
        map(
            crate::parser::metadata_annotation::metadata_annotation,
            ConstraintDefBodyElement::MetadataAnnotation,
        )
        .parse(input)?
    } else if starts_with_keyword(input.fragment(), b"in")
        || starts_with_keyword(input.fragment(), b"out")
        || starts_with_keyword(input.fragment(), b"inout")
    {
        if named_in_out_missing_type(input) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
        map(in_out_decl, ConstraintDefBodyElement::InOutDecl).parse(input)?
    } else if starts_with_keyword(input.fragment(), b"constraint") {
        map(constraint_usage, ConstraintDefBodyElement::Constraint).parse(input)?
    } else if input.fragment().starts_with(b":>>") || input.fragment().starts_with(b":>") {
        map(
            crate::parser::attribute::redefinition_feature_binding,
            |n| ConstraintDefBodyElement::AttributeUsage(Box::new(n)),
        )
        .parse(input)?
    } else {
        map(expression, ConstraintDefBodyElement::Expression).parse(input)?
    };
    let (input, _) = opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// Calculation usage: `calc` Identification (`:` type)? (`=` value)? body (SysML §7.19.2).
pub(crate) fn calc_usage(input: Input<'_>) -> IResult<Input<'_>, Node<CalcUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    // `abstract calc subcalculations: Calculation :> calculations, subactions { ... }` (Systems
    // Library `Calculations.sysml`); accepted and discarded, matching `RefDecl`'s "don't model
    // every shorthand" scope for feature modifiers.
    let (input, _) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"calc"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (identification, redefines)) = if input.fragment().starts_with(b":>>") {
        let (input, _) = tag(&b":>>"[..]).parse(input)?;
        let (target_start, _) = ws_and_comments(input)?;
        let (input, target) = qualified_reference(target_start)?;
        (
            input,
            (
                crate::ast::Identification {
                    short_name: None,
                    name: None,
                },
                Some(vec![target]),
            ),
        )
    } else {
        // `ws_and_comments` above already skipped gaps after `calc`; do not require `ws1`
        // here or `calc name …` fails once the space was consumed (validation `10c`).
        let (input, identification) = identification(input)?;
        (input, (identification, None))
    };
    let (input, type_name) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_reference),
    ))
    .parse(input)?;
    // `:>>` redefines may also follow the type instead of preceding the identification, e.g.
    // `calc self: Calculation :>> Action::self, Evaluation::self;` (Systems Library
    // `Calculations.sysml`) -- only retry if the earlier attempt (right after `calc`) didn't
    // already find one, and reuse the shared comma-separated multi-target parser.
    let (input, redefines) = if redefines.is_none() {
        opt(preceded(ws_and_comments, redefinition))
            .parse(input)
            .map(|(input, r)| (input, r.map(|node| node.value.target)))?
    } else {
        (input, redefines)
    };
    // `:>` subsets, independent of `:>>` redefines, e.g. `abstract calc subcalculations:
    // Calculation :> calculations, subactions { ... }` (Systems Library `Calculations.sysml`);
    // accepted and discarded -- `CalcUsage` doesn't model subsetting separately from redefines,
    // matching `RefDecl`'s "don't model every shorthand" scope for feature modifiers.
    let (input, _) =
        opt(preceded(ws_and_comments, crate::parser::usage::subsetting)).parse(input)?;
    let (input, value) = opt(preceded(
        ws_and_comments,
        crate::parser::feature_value::feature_value_part,
    ))
    .parse(input)?;
    let (input, body) = calc_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            CalcUsage {
                identification,
                type_name,
                redefines,
                value,
                direction: None,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// `def` is intentionally optional: the standard library uses bare, `def`-less `calc` usages at
/// namespace level (e.g. `abstract calc calculations: Calculation[0..*] nonunique :> actions,
/// evaluations { ... }` in `Systems Library/Calculations.sysml`). `calc_def` and `calc_usage` are
/// never dispatched together in the same alt today (`calc_usage` is only used standalone in part
/// bodies), so this is not the PAR-001 bug class, but do not add `.def_required()` here without
/// checking package-level content first.
pub(crate) fn calc_def(input: Input<'_>) -> IResult<Input<'_>, Node<CalcDef>> {
    parse_calc_def(input, false)
}

/// Calc definition with required `def` keyword, for contexts (e.g. nested inside a part
/// definition body) where `calc_usage` is already dispatched in the same `alt(...)` -- requiring
/// `def` here prevents a `def`-less calc usage from being misclassified as a definition, the same
/// bug class as PAR-001 in `attribute_def`. Unlike [`calc_def`] (kept `def`-optional for the
/// namespace-level bare form documented on that function), this variant is safe to stack ahead of
/// `calc_usage`.
pub(crate) fn calc_def_required(input: Input<'_>) -> IResult<Input<'_>, Node<CalcDef>> {
    parse_calc_def(input, true)
}

fn parse_calc_def(input: Input<'_>, require_def: bool) -> IResult<Input<'_>, Node<CalcDef>> {
    let start = input;
    let mut options = DefinitionPrefixOptions::new(b"calc").with_captured_visibility();
    if require_def {
        options = options.def_required();
    }
    let (input, prefix) = parse_definition_prefix(input, options)?;
    let (input, body) = calc_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            CalcDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

fn calc_body_recovery_element(start: Input<'_>, end: Input<'_>) -> Node<CalcDefBodyElement> {
    if starts_with_any_keyword(start.fragment(), CALC_DEF_BODY_STARTERS) {
        let recovery = build_recovery_error_node_from_span(
            start,
            end,
            CALC_DEF_BODY_STARTERS,
            "calc body",
            "recovered_calc_body_element",
        );
        let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
        return node_from_to(start, end, CalcDefBodyElement::Error(node));
    }
    let frag = start.fragment();
    let take = frag.len().min(120);
    let preview = String::from_utf8_lossy(&frag[..take]).trim().to_string();
    node_from_to(start, end, CalcDefBodyElement::Other(preview))
}

pub(crate) fn calc_def_body(input: Input<'_>) -> IResult<Input<'_>, CalcDefBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, CalcDefBody::Semicolon));
    }
    let (input, elements) = parse_structured_brace_members(
        input,
        CALC_DEF_BODY_STARTERS,
        "calc body",
        "recovered_calc_body_element",
        calc_def_body_element,
        calc_body_recovery_element,
    )?;
    Ok((input, CalcDefBody::Brace { elements }))
}

/// Skips an optional `(private|protected|public)?` `abstract`? prefix and reports whether either
/// was present, for the dispatch-gate peeks below (`calc_usage`/`parse_calc_def` themselves also
/// consume `abstract`/visibility, so this only needs to decide *which* branch to try).
fn skip_calc_modifiers(input: Input<'_>) -> IResult<Input<'_>, bool> {
    let (input, (_, visibility)) = visibility_prefix(input)?;
    let (input, is_abstract) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    Ok((input, visibility.is_some() || is_abstract.is_some()))
}

/// True when `input` is a nested `calc def` (e.g. plain `calc def Inner {` or modifier-prefixed
/// `private calc def Linear {`) -- distinct from the `def`-less `calc` usage form, which
/// `starts_with_keyword(.., b"calc")` alone can't tell apart from this one (both start with
/// `calc`).
fn calc_def_follows_visibility(input: Input<'_>) -> bool {
    let Ok((after_mods, _)) = skip_calc_modifiers(input) else {
        return false;
    };
    let Ok((after_calc, _)) = preceded(tag(&b"calc"[..]), ws1).parse(after_mods) else {
        return false;
    };
    starts_with_keyword(after_calc.fragment(), b"def")
}

/// True when `input` is a modifier-prefixed, `def`-less `calc` usage (e.g. `abstract calc
/// subcalculations: ...` / `private calc getElapsedUtcTime {`); `calc_usage` itself already
/// consumes those modifiers, this just widens the dispatch gate to reach it.
fn calc_usage_follows_visibility(input: Input<'_>) -> bool {
    let Ok((after_mods, had_modifiers)) = skip_calc_modifiers(input) else {
        return false;
    };
    had_modifiers && starts_with_keyword(after_mods.fragment(), b"calc")
}

/// KerML kinded parameter member: `in expr fn[0..*] { ... }`, `in bool test = expr;`,
/// `in feature clock : Clock[1] default localClock { ... }` (Kernel Function/Semantic
/// Libraries). The kind keyword distinguishes it from the keyword-less [`in_out_decl`]
/// parameter form, and its `{ ... }` body follows the calc-body member grammar (nested
/// parameters plus a `return` result).
pub(crate) fn typed_parameter_member(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::TypedParameterMember>> {
    crate::parser::span::reference_transaction(input, typed_parameter_member_inner)
}

fn typed_parameter_member_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::TypedParameterMember>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, direction) = crate::parser::attribute::direction_prefix(input)?;
    // `in abstract feature onOccurrence : Occurrence [1] { ... }`
    // (`FeatureReferencingPerformances.kerml`).
    let (input, is_abstract) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, kind) = alt((
        map(preceded(tag(&b"expr"[..]), ws1), |_| {
            crate::ast::KermlParameterKind::Expr
        }),
        map(preceded(tag(&b"bool"[..]), ws1), |_| {
            crate::ast::KermlParameterKind::Bool
        }),
        map(preceded(tag(&b"feature"[..]), ws1), |_| {
            crate::ast::KermlParameterKind::Feature
        }),
        map(preceded(tag(&b"calc"[..]), ws1), |_| {
            crate::ast::KermlParameterKind::Calc
        }),
        map(preceded(tag(&b"step"[..]), ws1), |_| {
            crate::ast::KermlParameterKind::Step
        }),
    ))
    .parse(input)?;
    // Leading redefinition may replace the name entirely: `in bool redefines ifTest { ... }`,
    // `inout feature redefines replacementValues[0..*] : ObserveChange;`.
    let (input, leading_redefines) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::redefinition,
    ))
    .parse(input)?;
    let (input, name_str) = if leading_redefines.is_some() {
        (input, String::new())
    } else {
        let (input, n) = preceded(ws_and_comments, name).parse(input)?;
        (input, n)
    };
    let (input, leading_multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (input, type_name) = opt(map(
        (
            preceded(ws_and_comments, tag(&b":"[..])),
            preceded(ws_and_comments, qualified_reference),
        ),
        |(_, tn)| tn,
    ))
    .parse(input)?;
    let (input, trailing_multiplicity) = if leading_multiplicity.is_none() {
        opt(preceded(
            ws_and_comments,
            crate::parser::usage::multiplicity_node,
        ))
        .parse(input)?
    } else {
        (input, None)
    };
    let (input, (ordered, nonunique)) = crate::parser::usage::usage_feature_modifier_flags(input)?;
    let (input, redefines) = if leading_redefines.is_none() {
        opt(preceded(
            ws_and_comments,
            crate::parser::usage::redefinition,
        ))
        .parse(input)?
    } else {
        (input, leading_redefines)
    };
    // The typing may trail a leading redefinition target: `in step redefines thenClause :
    // BooleanEvaluationResultToMonitorPerformance { ... }` (`Observation.kerml`).
    let (input, type_name) = if type_name.is_none() {
        opt(map(
            (
                preceded(ws_and_comments, tag(&b":"[..])),
                preceded(ws_and_comments, qualified_reference),
            ),
            |(_, tn)| tn,
        ))
        .parse(input)?
    } else {
        (input, type_name)
    };
    // The multiplicity may trail the redefinition target: `inout feature replacementValues :
    // Anything redefines values [*] nonunique;` (`FeatureReferencingPerformances.kerml`).
    let (input, post_redefines_multiplicity) =
        if leading_multiplicity.is_none() && trailing_multiplicity.is_none() {
            opt(preceded(
                ws_and_comments,
                crate::parser::usage::multiplicity_node,
            ))
            .parse(input)?
        } else {
            (input, None)
        };
    let (input, (post_ordered, post_nonunique)) =
        crate::parser::usage::usage_feature_modifier_flags(input)?;
    let (input, value) = opt(crate::parser::feature_value::feature_value_part).parse(input)?;
    let (input, body) = calc_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::TypedParameterMember {
                direction,
                is_abstract: is_abstract.is_some(),
                kind,
                name: name_str,
                redefines,
                type_name,
                multiplicity: leading_multiplicity
                    .or(trailing_multiplicity)
                    .or(post_redefines_multiplicity),
                ordered: ordered || post_ordered,
                nonunique: nonunique || post_nonunique,
                value,
                body,
            },
        ),
    ))
}

/// KerML feature member (type-body scope): optional `member`/`derived`/`abstract`/`composite`/
/// `portion`/`var`/`end` prefixes, the `feature` keyword, optional `all`, then the usual
/// declaration surface (name or leading redefinition, typing, multiplicity, `ordered`/
/// `nonunique`, subsets/redefines/references, `inverse of`, value, body). Kernel Semantic
/// Library `KerML.kerml`/`Occurrences.kerml`.
pub(crate) fn kerml_feature_member(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlFeatureMember>> {
    crate::parser::span::reference_transaction(input, kerml_feature_member_inner)
}

fn kerml_feature_member_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlFeatureMember>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, is_member) = opt(preceded(tag(&b"member"[..]), ws1)).parse(input)?;
    let (input, is_derived) = opt(preceded(tag(&b"derived"[..]), ws1)).parse(input)?;
    let (input, is_abstract) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, composite_or_portion) = opt(alt((
        map(preceded(tag(&b"composite"[..]), ws1), |_| true),
        map(preceded(tag(&b"portion"[..]), ws1), |_| false),
    )))
    .parse(input)?;
    let (input, is_var) = opt(preceded(tag(&b"var"[..]), ws1)).parse(input)?;
    let (input, is_end) = opt(preceded(tag(&b"end"[..]), ws1)).parse(input)?;
    let (input, kind) = alt((
        map(preceded(tag(&b"feature"[..]), ws1), |_| {
            crate::ast::KermlFeatureKind::Feature
        }),
        map(preceded(tag(&b"step"[..]), ws1), |_| {
            crate::ast::KermlFeatureKind::Step
        }),
        map(preceded(tag(&b"expr"[..]), ws1), |_| {
            crate::ast::KermlFeatureKind::Expr
        }),
        map(preceded(tag(&b"bool"[..]), ws1), |_| {
            crate::ast::KermlFeatureKind::Bool
        }),
    ))
    .parse(input)?;
    let (input, is_all) = opt(preceded(tag(&b"all"[..]), ws1)).parse(input)?;
    // Leading redefinition may replace the name: `portion feature redefines spaceBoundary [1];`.
    let (input, leading_redefines) = opt(preceded(ws_and_comments, redefinition)).parse(input)?;
    let (input, name_str) = if leading_redefines.is_some() {
        (input, String::new())
    } else {
        let (input, n) = preceded(ws_and_comments, name).parse(input)?;
        (input, n)
    };
    let (input, leading_multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (input, type_result) = crate::parser::usage::optional_typings(input)?;
    let typing = type_result.map(|(span, is_conjugated, targets)| {
        crate::ast::Node::new(
            span.clone(),
            crate::ast::TypingRelationship {
                target: targets,
                kind: crate::ast::TypingKind::Typing,
                span,
                is_conjugated,
                is_implied: false,
            },
        )
    });
    let (input, trailing_multiplicity) = if leading_multiplicity.is_none() {
        opt(preceded(
            ws_and_comments,
            crate::parser::usage::multiplicity_node,
        ))
        .parse(input)?
    } else {
        (input, None)
    };
    let (input, (ordered, nonunique)) = crate::parser::usage::usage_feature_modifier_flags(input)?;
    let (input, clauses) = crate::parser::usage::specialization_clauses(input)?;
    let subsets = clauses.subsets.map(|(target, _value)| target);
    let redefines = leading_redefines.or(clauses.redefines);
    let references = clauses.references;
    // `inverse of <chain>`: `feature all spaceShotOf: Occurrence[0..*] subsets spaceSliceOf
    // inverse of spaceShots { ... }` (`Occurrences.kerml`).
    let (input, inverse_of) = opt(preceded(
        (
            ws_and_comments,
            tag(&b"inverse"[..]),
            ws1,
            tag(&b"of"[..]),
            ws1,
        ),
        qualified_reference,
    ))
    .parse(input)?;
    let (input, value) = opt(crate::parser::feature_value::feature_value_part).parse(input)?;
    let (input, body) = calc_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::KermlFeatureMember {
                is_member: is_member.is_some(),
                is_derived: is_derived.is_some(),
                is_abstract: is_abstract.is_some(),
                is_composite: composite_or_portion == Some(true),
                is_portion: composite_or_portion == Some(false),
                is_var: is_var.is_some(),
                is_end: is_end.is_some(),
                kind,
                is_all: is_all.is_some(),
                name: name_str,
                typing,
                multiplicity: leading_multiplicity.or(trailing_multiplicity),
                ordered,
                nonunique,
                subsets,
                redefines,
                references,
                inverse_of,
                value,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// KerML invariant member: `inv` (`not`)? name? `{ boolean-expressions }`. The bare `inv
/// name;` forward-declaration form stays on `kerml_bare_declaration`, so this parser requires
/// the `{` body.
pub(crate) fn kerml_invariant_member(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlInvariantMember>> {
    crate::parser::span::reference_transaction(input, kerml_invariant_member_inner)
}

fn kerml_invariant_member_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlInvariantMember>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    if !starts_with_keyword(input.fragment(), b"inv") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, _) = tag(&b"inv"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, is_negated) = opt(preceded(tag(&b"not"[..]), ws1)).parse(input)?;
    let (input, name_str) = opt(preceded(ws_and_comments, name)).parse(input)?;
    let (peek, _) = ws_and_comments(input)?;
    if !peek.fragment().starts_with(b"{") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, body) = calc_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::KermlInvariantMember {
                is_negated: is_negated.is_some(),
                name: name_str.unwrap_or_default(),
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

fn calc_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<CalcDefBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = if starts_with_keyword(input.fragment(), b"doc") {
        map(
            crate::parser::requirement::doc_comment,
            CalcDefBodyElement::Doc,
        )
        .parse(input)?
    } else if input.fragment().starts_with(b"@") {
        map(
            crate::parser::metadata_annotation::metadata_annotation,
            CalcDefBodyElement::MetadataAnnotation,
        )
        .parse(input)?
    } else if starts_with_any_keyword(
        input.fragment(),
        &[
            b"member",
            b"derived",
            b"composite",
            b"portion",
            b"var",
            b"end",
            b"feature",
            b"step",
        ],
    ) || (starts_with_keyword(input.fragment(), b"abstract")
        && kerml_feature_member(input).is_ok())
    {
        map(kerml_feature_member, |n| {
            CalcDefBodyElement::KermlFeature(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_keyword(input.fragment(), b"inv") {
        map(kerml_invariant_member, |n| {
            CalcDefBodyElement::Invariant(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_keyword(input.fragment(), b"in")
        || starts_with_keyword(input.fragment(), b"out")
        || starts_with_keyword(input.fragment(), b"inout")
    {
        // Prefer directed `in part …` before plain InOutDecl (which rejects `in part`).
        if let Ok((input, part)) = crate::parser::part::part_usage(input) {
            (input, CalcDefBodyElement::PartUsage(Box::new(part)))
        } else if let Ok((input, param)) = typed_parameter_member(input) {
            // `in expr …` / `in bool …` / `in feature …` kinded parameters (Kernel
            // Function/Semantic Libraries), before plain InOutDecl, which would misread the
            // kind keyword as the parameter name.
            (input, CalcDefBodyElement::TypedParameter(Box::new(param)))
        } else if named_in_out_missing_type(input) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        } else {
            map(in_out_decl, |n| CalcDefBodyElement::InOutDecl(Box::new(n))).parse(input)?
        }
    } else if calc_def_follows_visibility(input) {
        // Nested `(private|protected|public)? calc def Name { ... }` rollup helper (Domain
        // Libraries `SampledFunctions.sysml`'s `Linear`), distinct from the def-less `calc`
        // usage form handled below.
        map(calc_def_required, |n| {
            CalcDefBodyElement::CalcDef(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_keyword(input.fragment(), b"calc") || calc_usage_follows_visibility(input)
    {
        map(calc_usage, |n| CalcDefBodyElement::CalcUsage(Box::new(n))).parse(input)?
    } else if starts_with_keyword(input.fragment(), b"return") {
        if named_return_missing_type(input) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
        if let Ok((input, decl)) = return_decl(input) {
            (input, CalcDefBodyElement::ReturnDecl(decl))
        } else if let Ok((input, expr)) = return_expression_stmt(input) {
            (input, CalcDefBodyElement::Expression(expr))
        } else {
            other_calc_return(input)?
        }
    } else {
        map(expression, CalcDefBodyElement::Expression).parse(input)?
    };
    let (input, _) = opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn named_in_out_missing_type(input: Input<'_>) -> bool {
    let Ok((after_ws, _)) = ws_and_comments(input) else {
        return false;
    };
    let direction_len = if starts_with_keyword(after_ws.fragment(), b"inout") {
        5
    } else if starts_with_keyword(after_ws.fragment(), b"out") {
        3
    } else {
        2
    };
    let mut rest = &after_ws.fragment()[direction_len..];
    while let Some(first) = rest.first() {
        if first.is_ascii_whitespace() {
            rest = &rest[1..];
        } else {
            break;
        }
    }
    if rest.starts_with(b":") {
        return false;
    }
    let mut name_len = 0usize;
    while name_len < rest.len()
        && (rest[name_len].is_ascii_alphanumeric() || rest[name_len] == b'_')
    {
        name_len += 1;
    }
    if name_len == 0 {
        return false;
    }
    rest = &rest[name_len..];
    while let Some(first) = rest.first() {
        if first.is_ascii_whitespace() {
            rest = &rest[1..];
        } else {
            break;
        }
    }
    if !rest.starts_with(b":") {
        return false;
    }
    rest = &rest[1..];
    while let Some(first) = rest.first() {
        if first.is_ascii_whitespace() {
            rest = &rest[1..];
        } else {
            break;
        }
    }
    rest.is_empty() || rest.starts_with(b";") || rest.starts_with(b"{") || rest.starts_with(b"}")
}

fn named_return_missing_type(input: Input<'_>) -> bool {
    let Ok((after_ws, _)) = ws_and_comments(input) else {
        return false;
    };
    let mut rest = *after_ws.fragment();
    if !starts_with_keyword(rest, b"return") {
        return false;
    }
    rest = &rest[6..];
    while let Some(first) = rest.first() {
        if first.is_ascii_whitespace() {
            rest = &rest[1..];
        } else {
            break;
        }
    }
    if rest.starts_with(b":") {
        return false;
    }
    let mut name_len = 0usize;
    while name_len < rest.len()
        && (rest[name_len].is_ascii_alphanumeric() || rest[name_len] == b'_')
    {
        name_len += 1;
    }
    if name_len == 0 {
        return false;
    }
    rest = &rest[name_len..];
    while let Some(first) = rest.first() {
        if first.is_ascii_whitespace() {
            rest = &rest[1..];
        } else {
            break;
        }
    }
    if rest.starts_with(b":") {
        rest = &rest[1..];
        while let Some(first) = rest.first() {
            if first.is_ascii_whitespace() {
                rest = &rest[1..];
            } else {
                break;
            }
        }
        return rest.is_empty()
            || rest.starts_with(b";")
            || rest.starts_with(b"{")
            || rest.starts_with(b"}");
    }
    false
}

/// `return sum(parts.massKg);` — expression return (SysML calc / return-ref body), not `return name : Type;`.
pub(crate) fn return_expression_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let (input, _) = preceded(ws_and_comments, tag(&b"return"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, expr) = expression(input)?;
    let (input, _) = opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(input)?;
    Ok((input, expr))
}

fn other_calc_return(input: Input<'_>) -> IResult<Input<'_>, CalcDefBodyElement> {
    let start_unknown = input;
    let (next, _) = skip_statement_or_block(input)?;
    if next.location_offset() == start_unknown.location_offset() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Many0,
        )));
    }
    let preview = String::from_utf8_lossy(
        &start_unknown.fragment()[..start_unknown.fragment().len().min(120)],
    )
    .trim()
    .to_string();
    Ok((next, CalcDefBodyElement::Other(preview)))
}

pub(crate) fn return_decl(input: Input<'_>) -> IResult<Input<'_>, Node<ReturnDecl>> {
    crate::parser::span::reference_transaction(input, return_decl_inner)
}

fn return_decl_inner(input: Input<'_>) -> IResult<Input<'_>, Node<ReturnDecl>> {
    let start = input;
    let (input, _) = tag(&b"return"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    // `return :>> name : Type = expr;`
    let (input, is_redefine) = if input.fragment().starts_with(b":>>") {
        let (input, _) = tag(&b":>>"[..]).parse(input)?;
        let (input, _) = ws_and_comments(input)?;
        (input, true)
    } else {
        (input, false)
    };
    // Anonymous typed form: `return : Type [= expr];`
    let (input, n) = if input.fragment().starts_with(b":") && !input.fragment().starts_with(b":>") {
        (input, String::new())
    } else {
        let (input, n) = name(input)?;
        (input, n)
    };
    let (input, _) = ws_and_comments(input)?;
    let (input, is_subsetting) = if input.fragment().starts_with(b":>") {
        let (input, _) = tag(&b":>"[..]).parse(input)?;
        (input, true)
    } else {
        let (input, _) = tag(&b":"[..]).parse(input)?;
        (input, false)
    };
    let (input, type_name) = preceded(ws_and_comments, qualified_reference).parse(input)?;
    // Multiplicity and `ordered`/`nonunique` after the type: `return : Real[1] = x;`,
    // `return : Anything[0..*] ordered nonunique;` (Kernel Function Library).
    let (input, multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (input, (ordered, nonunique)) = crate::parser::usage::usage_feature_modifier_flags(input)?;
    // `= expr` / `:= expr` / `default (=|:=)? expr` value clause: `return : Real default
    // NumericalFunctions::sum0(collection, 0.0);` (Kernel Function Library).
    let (input, value) = opt(crate::parser::feature_value::feature_value_part).parse(input)?;
    // `;` or a `{ ... }` result body (`return positionVector : Position3dVector[1] {
    // attribute :>> mRef = ...; }`, Domain Libraries `SpatialItems.sysml`).
    let (input, body) = calc_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ReturnDecl {
                name: n,
                type_name,
                is_redefine,
                is_subsetting,
                multiplicity,
                ordered,
                nonunique,
                value,
                body,
            },
        ),
    ))
}

#[cfg(test)]
mod membership_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // --- parser work item 4b (final sweep): Membership on ConstraintDef/CalcDef/CalcUsage ---

    #[test]
    fn constraint_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) =
            constraint_def(input("private constraint def C1;")).expect("constraint def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn constraint_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = constraint_def(input("constraint def C1;")).expect("constraint def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn calc_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = calc_def(input("protected calc def C1;")).expect("calc def");
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
    fn calc_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = calc_def(input("calc def C1;")).expect("calc def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn calc_usage_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = calc_usage(input("public calc c1;")).expect("calc usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn calc_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = calc_usage(input("calc c1;")).expect("calc usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }
}

#[cfg(test)]
mod constraint_usage_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // Package-level `def`/usage disambiguation (PAR-001 bug class, closed for `constraint` here).
    // `def` is now required for `ConstraintDef`, with `constraint_usage` (built on
    // `feature_usage_header`) catching every bare, `def`-less real-library form instead -- see
    // `constraint_def`'s doc comment and CHANGELOG 0.33.0 for the earlier, unsafe attempt this
    // supersedes.

    #[test]
    fn constraint_def_no_longer_accepts_a_bare_def_less_declaration() {
        // Previously folded into `ConstraintDef`; must now be left for `constraint_usage`.
        let result = constraint_def(input("constraint c : C;"));
        assert!(
            result.is_err(),
            "constraint_def must require the `def` keyword"
        );
    }

    #[test]
    fn constraint_usage_accepts_simple_typed_semicolon_form() {
        let (rest, node) = constraint_usage(input("constraint c : C;")).expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "c");
        assert!(node.value.type_name.is_some());
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn constraint_usage_accepts_typed_braced_form() {
        let (rest, node) = constraint_usage(input(
            "constraint mc : MassConstraint4 {\n    in totalMass : MassValue;\n}",
        ))
        .expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "mc");
        assert!(node.value.type_name.is_some());
        assert!(matches!(node.value.body, ConstraintDefBody::Brace { .. }));
    }

    #[test]
    fn constraint_usage_accepts_untyped_braced_form() {
        let (_, node) = constraint_usage(input(
            "constraint hasLegalProfileDepth {profileDepth >= 3.5 [mm]}",
        ))
        .expect("constraint usage");
        assert_eq!(node.value.name, "hasLegalProfileDepth");
        assert_eq!(node.value.type_name, None);
    }

    /// Regression: `Systems Library/Constraints.sysml`'s `constraintChecks` -- `abstract` +
    /// typing + trailing multiplicity + `nonunique` modifier + subsetting, all `def`-less. This
    /// exact shape is why `constraint_def` couldn't safely require `def` before this parser
    /// existed (CHANGELOG 0.33.0).
    #[test]
    fn constraint_usage_accepts_the_real_library_constraint_checks_form() {
        let (rest, node) = constraint_usage(input(
            "abstract constraint constraintChecks: ConstraintCheck[0..*] nonunique :> booleanEvaluations {\n}",
        ))
        .expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "constraintChecks");
        assert!(node.value.type_name.is_some());
    }

    /// Regression: `Systems Library/Constraints.sysml`'s `assertedConstraintChecks` -- `abstract`
    /// + multi-target subsetting only, no typing at all.
    #[test]
    fn constraint_usage_accepts_subsetting_only_multi_target_form() {
        let (rest, node) = constraint_usage(input(
            "abstract constraint assertedConstraintChecks :> constraintChecks, trueEvaluations {\n}",
        ))
        .expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "assertedConstraintChecks");
        assert_eq!(node.value.type_name, None);
    }

    /// Regression: `Systems Library/Requirements.sysml`'s `constraint assumptions[0..*] :>
    /// constraintChecks, subperformances` -- multiplicity directly after the name with no typing,
    /// then multi-target subsetting.
    #[test]
    fn constraint_usage_accepts_leading_multiplicity_then_subsetting() {
        let (rest, node) = constraint_usage(input(
            "constraint assumptions[0..*] :> constraintChecks, subperformances {\n}",
        ))
        .expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "assumptions");
        assert_eq!(node.value.type_name, None);
    }

    /// Regression: `Systems Library/Requirements.sysml`'s `constraint assumptions :>>
    /// RequirementConstraintCheck::assumptions;` -- redefinition with a qualified target,
    /// semicolon body.
    #[test]
    fn constraint_usage_accepts_redefinition_with_qualified_target() {
        let (rest, node) = constraint_usage(input(
            "constraint assumptions :>> RequirementConstraintCheck::assumptions;",
        ))
        .expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "assumptions");
        assert_eq!(node.value.type_name, None);
        assert!(matches!(node.value.body, ConstraintDefBody::Semicolon));
    }

    #[test]
    fn constraint_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = constraint_usage(input("public constraint c1;")).expect("constraint usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn constraint_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = constraint_usage(input("constraint c1;")).expect("constraint usage");
        assert_eq!(node.value.membership.visibility, None);
    }

    /// Package-level dispatch order (`package.rs`): `constraint def X;` must still classify as
    /// `ConstraintDef`, not fall through to `constraint_usage`.
    #[test]
    fn package_body_dispatch_still_classifies_constraint_def_correctly() {
        use crate::parser::package::package_body_element;

        let (_, node) = package_body_element(input("constraint def X;"))
            .expect("constraint def package member");
        assert!(matches!(
            node.value,
            crate::ast::PackageBodyElement::ConstraintDef(_)
        ));
    }

    /// Package-level dispatch order (`package.rs`): a bare, `def`-less `constraint c : X;` must
    /// now classify as `ConstraintUsage`, not be misclassified as `ConstraintDef`.
    #[test]
    fn package_body_dispatch_classifies_bare_constraint_as_usage() {
        use crate::parser::package::package_body_element;

        let (_, node) = package_body_element(input("constraint c : X;"))
            .expect("constraint usage package member");
        assert!(matches!(
            node.value,
            crate::ast::PackageBodyElement::ConstraintUsage(_)
        ));
    }

    #[test]
    fn failed_return_declaration_rolls_back_type_and_value_references() {
        let context = crate::parser::span::ParseContext::new();
        let parsed = return_decl(context.input(b"return value : Ghost::Result = Ghost::value"));
        assert!(parsed.is_err());
        assert!(context.finish().is_empty());
    }
}
