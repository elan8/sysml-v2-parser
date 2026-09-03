use crate::ast::{
    CalcDef, CalcDefBody, CalcDefBodyElement, CalcDefBodyLibraryPackageMember,
    CalcDefBodyPackageMember, CalcUsage, ConstraintDef, ConstraintDefBody,
    ConstraintDefBodyElement, ConstraintUsage, Expression, Membership, Node, ParseErrorNode,
    ReturnDecl,
};
use crate::parser::action::in_out_decl;
use crate::parser::body::{parse_structured_brace_members, semicolon_body};
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
            .def_required()
            .individual_allowed(),
    )?;
    let (input, body) = constraint_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ConstraintDef {
                is_individual: prefix.is_individual,
                definition_prefix: prefix.basic_prefix,
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
/// `ConstraintUsage = OccurrenceUsagePrefix 'constraint' ConstraintUsageDeclaration
/// CalculationBody` (SysML BNF 1382).
///
/// Wrapped in a reference transaction because the prefix's `UsageExtensionKeyword*` allocates an
/// arena entry per `#tag` before the production is known to apply.
pub(crate) fn constraint_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ConstraintUsage>> {
    // Speculated at member starts it does not own; refuse by lookahead before entering an
    // arena transaction. See [`kind_keyword_follows`](crate::parser::occurrence_prefix::kind_keyword_follows).
    if !crate::parser::occurrence_prefix::kind_keyword_follows(input, b"constraint") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    crate::parser::span::reference_transaction(input, constraint_usage_inner)
}

fn constraint_usage_inner(input: Input<'_>) -> IResult<Input<'_>, Node<ConstraintUsage>> {
    let start = input;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    // The whole shared `OccurrenceUsagePrefix`, not a leading `abstract` consumed and thrown
    // away: `ref constraint self : ConstraintCheck :>> BooleanEvaluation::self;` (Systems Library
    // `Constraints.sysml:20`) was refused here and shredded into two members by the scope's
    // expression fallback.
    let (input, prefix) = crate::parser::occurrence_prefix::occurrence_usage_prefix(input)?;
    let (input, _) = tag(&b"constraint"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, short_name) = crate::parser::lex::short_name_prefix(input)?;
    let (input, _) = ws_and_comments(input)?;
    // Anonymous body-only form: `constraint { stateSpace.order == order }` (Domain Libraries
    // `StateSpaceRepresentation.sysml`; spec42 Gap 49a).
    let (input, name_str) = {
        let (peek, _) = ws_and_comments(input)?;
        if peek.fragment().starts_with(b"{") {
            (input, None)
        } else {
            let (input, n) = name(input)?;
            (input, Some(n))
        }
    };
    let (input, header) = feature_usage_header(input)?;
    let (input, body) = constraint_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ConstraintUsage {
                prefix,
                short_name,
                name: name_str,
                type_name: header.type_reference,
                multiplicity: header.multiplicity,
                subsets: header.subsets,
                redefines: header.redefines,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

fn constraint_body_recovery_element(
    start: Input<'_>,
    end: Input<'_>,
) -> Node<ConstraintDefBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        CONSTRAINT_DEF_BODY_STARTERS,
        "constraint body",
        "recovered_constraint_body_element",
    );
    let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
    node_from_to(start, end, ConstraintDefBodyElement::Error(node))
}

/// A constraint body in any scope that accepts one: `;` or brace-delimited constraint members.
pub(crate) fn constraint_def_body(input: Input<'_>) -> IResult<Input<'_>, ConstraintDefBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let semicolon_start = input;
        let (input, _) = tag(&b";"[..]).parse(semicolon_start)?;
        return Ok((
            input,
            ConstraintDefBody::Semicolon {
                semicolon_span: crate::parser::span::span_from_to(semicolon_start, input),
            },
        ));
    }
    let (input, members) = parse_structured_brace_members(
        input,
        CONSTRAINT_DEF_BODY_STARTERS,
        "constraint body",
        "recovered_constraint_body_element",
        constraint_def_body_element,
        constraint_body_recovery_element,
    )?;
    Ok((input, members.into_body()))
}

pub(crate) fn constraint_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ConstraintDefBodyElement>> {
    let start = input;
    // Member boundary: `ws_and_notes` leaves a bare `/* ... */` for this scope's
    // annotating member, which is the `Comment` production's keyword-less spelling.
    let (input, _) = crate::parser::lex::ws_and_notes(input)?;
    // Ahead of every guard below; see `body::starts_bare_comment` and the matching branch in
    // `calc_def_body_element`.
    if crate::parser::body::starts_bare_comment(input) {
        let comment_start = input;
        let (input, member) = crate::parser::body::annotating_member(comment_start)?;
        let elem = ConstraintDefBodyElement::Annotating(member);
        return Ok((input, node_from_to(comment_start, input, elem)));
    }
    // `MemberPrefix` precedes the member's own keyword, so the usage guards below look past it;
    // each arm's parser re-reads it. Without this a `private ref constraint hidden;` matched no
    // guard and reached the expression fallback.
    let after_visibility = crate::parser::lex::visibility_prefix(input)
        .map(|(rest, _)| rest)
        .unwrap_or(input);
    let (input, elem) = if starts_with_keyword(input.fragment(), b"doc")
        || starts_with_keyword(input.fragment(), b"comment")
        || starts_with_keyword(input.fragment(), b"rep")
        || starts_with_keyword(input.fragment(), b"language")
        || input.fragment().starts_with(b"@")
    {
        map(
            crate::parser::body::annotating_member,
            ConstraintDefBodyElement::Annotating,
        )
        .parse(input)?
    } else if let Some((next, usage)) =
        (starts_with_keyword(after_visibility.fragment(), b"constraint")
            || starts_with_any_keyword(after_visibility.fragment(), PART_USAGE_PREFIX_STARTERS)
            || crate::parser::occurrence_prefix::starts_contended_prefix(after_visibility))
        .then(|| constraint_usage(input))
        .and_then(Result::ok)
    {
        // The whole `OccurrenceUsagePrefix` may precede the kind keyword, so the guard cannot be
        // `constraint` alone: `abstract constraint constraintChecks : …` and `ref constraint
        // self : …` (Systems Library `Constraints.sysml`) both reached the expression fallback
        // and were shredded.
        (next, ConstraintDefBodyElement::Constraint(Box::new(usage)))
    } else if let Some((next, usage)) = (starts_with_keyword(after_visibility.fragment(), b"part")
        || starts_with_any_keyword(after_visibility.fragment(), PART_USAGE_PREFIX_STARTERS)
        || crate::parser::occurrence_prefix::starts_contended_prefix(after_visibility))
    .then(|| crate::parser::part::part_usage(input))
    .and_then(Result::ok)
    {
        // See `CalcDefBodyElement::PartUsage`: the same `CalculationBody` reaches the same
        // production, and this scope shredded it the same way.
        (next, ConstraintDefBodyElement::PartUsage(Box::new(usage)))
    } else if input.fragment().starts_with(b"#") {
        // Both `#` productions: the `ExtendedUsage` member spelling (which owns a `;`/`{}`
        // body) is tried before the `PrefixMetadataMember` spelling, which owns no body and
        // leaves the prefixed declaration for the next member iteration.
        alt((
            map(
                crate::parser::metadata_annotation::metadata_keyword_usage,
                ConstraintDefBodyElement::MetadataKeywordUsage,
            ),
            map(
                crate::parser::metadata_annotation::metadata_keyword_prefix,
                ConstraintDefBodyElement::MetadataKeywordUsage,
            ),
        ))
        .parse(input)?
    } else if starts_with_keyword(after_visibility.fragment(), b"alias") {
        // Both ConstraintDefinition and ConstraintUsage own CalculationBody, whose
        // CalculationBodyItem -> ActionBodyItem -> NonBehaviorBodyItem production admits an
        // AliasMember (SysML BNF 1359-1368, 1378-1382, and 901-917). Reuse AliasDef's
        // source-backed identification/target/body rather than letting expression fallback
        // split the member into unrelated expressions.
        map(
            crate::parser::alias::alias_def,
            ConstraintDefBodyElement::AliasDef,
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
        map(in_out_decl, |n| {
            ConstraintDefBodyElement::InOutDecl(Box::new(n))
        })
        .parse(input)?
    } else if input.fragment().starts_with(b":>>") || input.fragment().starts_with(b":>") {
        map(
            crate::parser::attribute::redefinition_feature_binding,
            |n| ConstraintDefBodyElement::AttributeUsage(Box::new(n)),
        )
        .parse(input)?
    } else if starts_with_keyword(input.fragment(), b"require")
        || starts_with_keyword(input.fragment(), b"assume")
    {
        // Ahead of the terminal `expression` arm below, which would otherwise read `require v`
        // as an expression and leave a brace body unaccounted for.
        map(crate::parser::requirement::require_constraint, |n| {
            ConstraintDefBodyElement::RequireConstraint(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_keyword(input.fragment(), b"return") {
        // `ConstraintDefinition`/`ConstraintUsage` end in `CalculationBody`, whose
        // `CalculationBodyItem = ActionBodyItem | ReturnParameterMember` (SysML BNF 1378, 1382,
        // 1359, 1366, 1370). Ahead of the terminal expression arm, which read `return` as a
        // name. The same three parsers the calculation scope dispatches, in the same order and
        // behind the same guard -- the two scopes are the one production.
        if named_return_missing_type(input) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
        if let Ok((next, decl)) = return_decl(input) {
            (next, ConstraintDefBodyElement::ReturnDecl(Box::new(decl)))
        } else if let Ok((next, expr)) = return_expression_stmt(input) {
            (next, ConstraintDefBodyElement::Expression(expr))
        } else {
            let (next, node) = other_return_recovery(
                input,
                CONSTRAINT_DEF_BODY_STARTERS,
                "constraint body",
                "recovered_constraint_body_element",
            )?;
            (next, ConstraintDefBodyElement::Error(node))
        }
    } else if let Ok((rest, binding)) = calc_named_binding(input) {
        // A constraint definition body is a `DefinitionBody`, so a keyword-less feature
        // declaration (`mass : Real;`) is a member of it, not an expression statement.
        (
            rest,
            ConstraintDefBodyElement::FeatureDecl(Box::new(binding)),
        )
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
    // `BasicUsagePrefix`'s `RefPrefix`: the direction (`in calc eval : EvaluationFunction { ... }`,
    // `sysml.library/Domain Libraries/Analysis/TradeStudies.sysml:61`) and `abstract` (`abstract
    // calc subcalculations: Calculation :> calculations, subactions { ... }`, Systems Library
    // `Calculations.sysml`) are slots of one production. `CalcUsage::direction` existed but was
    // never populated, and `abstract` was consumed and dropped.
    let (input, prefix) = crate::parser::usage::ref_prefix(input)?;
    // `BasicUsagePrefix = RefPrefix ( isReference ?= 'ref' )?`. Without this the keyword was
    // never consumed, `tag("calc")` failed on `ref calc ...`, and the enclosing calculation body
    // fell through to its expression parser, which happily took the bare word `ref` as an
    // expression statement of its own.
    let (input, is_reference) = opt(preceded(tag(&b"ref"[..]), ws1))
        .parse(input)
        .map(|(input, found)| (input, found.is_some()))?;
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
    let (input, multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
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
    // Calculation :> calculations, subactions { ... }` (Systems Library `Calculations.sysml`).
    let (input, subsets) = opt(preceded(ws_and_comments, crate::parser::usage::subsetting))
        .parse(input)
        .map(|(input, clause)| (input, clause.map(|(relationship, _value)| relationship)))?;
    let (input, value) = opt(preceded(
        ws_and_comments,
        crate::parser::feature_value::feature_value_part,
    ))
    .parse(input)?;
    let (input, body) = calculation_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            CalcUsage {
                is_reference,
                identification,
                is_abstract: prefix.usage_prefix == Some(crate::ast::DefinitionPrefix::Abstract),
                type_name,
                multiplicity,
                subsets,
                redefines,
                value,
                direction: prefix.direction,
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
    let mut options = DefinitionPrefixOptions::new(b"calc")
        .with_captured_visibility()
        .individual_allowed();
    if require_def {
        options = options.def_required();
    }
    let (input, prefix) = parse_definition_prefix(input, options)?;
    let (input, body) = calculation_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            CalcDef {
                is_individual: prefix.is_individual,
                definition_prefix: prefix.basic_prefix,
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

fn calc_body_recovery_element(start: Input<'_>, end: Input<'_>) -> Node<CalcDefBodyElement> {
    // Every unparseable member becomes an explicit recovery node with a diagnostic. Content
    // whose first token is not a known calc starter previously became a diagnostic-silent
    // opaque `Other(preview)` capture instead -- a swallow that made malformed input look
    // successfully parsed.
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        CALC_DEF_BODY_STARTERS,
        "calc body",
        "recovered_calc_body_element",
    );
    let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
    node_from_to(start, end, CalcDefBodyElement::Error(node))
}

/// KerML `TypeBodyElement -> NonFeatureMember -> MemberPrefix Package|LibraryPackage`.
///
/// `Package` itself has no membership slot: visibility belongs to the containing type-body
/// member. Parse the prefix once, then invoke the existing source-backed package parser from the
/// exact remainder. The enclosing transaction makes a refused library/package alternative leave
/// no qualified-reference arena entries behind.
fn calc_def_body_package_member(input: Input<'_>) -> IResult<Input<'_>, CalcDefBodyElement> {
    crate::parser::span::reference_transaction(input, |input| {
        let (input, _) = ws_and_comments(input)?;
        let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
        let membership = Membership::owning(visibility, visibility_span);
        if starts_with_keyword(input.fragment(), b"package") {
            let (input, package) = crate::parser::package::package_(input)?;
            Ok((
                input,
                CalcDefBodyElement::Package(CalcDefBodyPackageMember {
                    membership,
                    package,
                }),
            ))
        } else {
            let (input, package) = crate::parser::package::library_package_(input)?;
            Ok((
                input,
                CalcDefBodyElement::LibraryPackage(CalcDefBodyLibraryPackageMember {
                    membership,
                    package,
                }),
            ))
        }
    })
}

pub(crate) fn calc_def_body(input: Input<'_>) -> IResult<Input<'_>, CalcDefBody> {
    calc_body_with_element_parser(input, calc_def_body_element)
}

/// `CalculationBody : Type = ';' | '{' CalculationBodyPart '}'`, whose items are
/// `CalculationBodyItem = ActionBodyItem | ReturnParameterMember` (SysML 8.2.2.19).
///
/// Distinct from [`calc_def_body`], which serves KerML type bodies as well: `TypeBodyElement` has
/// no action-node alternative, so only a calculation reaches the action dispatcher. Sharing one
/// element enum across the two scopes is pre-existing; this keeps the *parser* precise about
/// which of them can produce an action member.
pub(crate) fn calculation_body(input: Input<'_>) -> IResult<Input<'_>, CalcDefBody> {
    calc_body_with_element_parser(input, calculation_body_element)
}

fn calc_body_with_element_parser<'a, F>(
    input: Input<'a>,
    element: F,
) -> IResult<Input<'a>, CalcDefBody>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, Node<CalcDefBodyElement>>,
{
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let semicolon_start = input;
        let (input, _) = tag(&b";"[..]).parse(semicolon_start)?;
        return Ok((
            input,
            CalcDefBody::Semicolon {
                semicolon_span: crate::parser::span::span_from_to(semicolon_start, input),
            },
        ));
    }
    let (input, members) = parse_structured_brace_members(
        input,
        CALC_DEF_BODY_STARTERS,
        "calc body",
        "recovered_calc_body_element",
        element,
        calc_body_recovery_element,
    )?;
    Ok((input, members.into_body()))
}

/// An action-body member first, then everything a calc body already owned.
///
/// The order is what matters. `calc_def_body_element` ends in a keyword-less
/// `DefaultReferenceUsage` binding, which happily reads `first` as a feature name -- so before
/// this, `first f;` in a calculation body parsed as two invented members, `'first';` and `f;`,
/// with no diagnostic at all, and formatted back that way.
fn calculation_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<CalcDefBodyElement>> {
    let (peek, _) = ws_and_comments(input)?;
    // `MemberPrefix` is outside `StructureUsageMember` (SysML BNF 130-131, 262-264), so the
    // action-family gate must look through it to recognize `private message …`.  Keep parsing
    // from `peek`, rather than this lookahead remainder: `action_def_body_element` owns the
    // prefix and records its source-backed visibility in the resulting membership.
    let action_peek = visibility_prefix(peek)
        .map(|(rest, _)| rest)
        .unwrap_or(peek);
    if crate::parser::lex::starts_with_any_keyword(
        action_peek.fragment(),
        crate::parser::lex::CALCULATION_ACTION_STARTERS,
    ) {
        let start = input;
        match crate::parser::action::action_def_body_element(peek) {
            Ok((next, member)) => {
                return Ok((
                    next,
                    node_from_to(
                        start,
                        next,
                        CalcDefBodyElement::ActionMember(Box::new(member)),
                    ),
                ));
            }
            // `ref` is unambiguously an ActionBodyItem in this owner. If its typed parser has
            // begun and rejected a malformed relationship, preserve the error for calculation
            // body recovery instead of letting the keyword-less feature fallback shred it into
            // separate `ref` and name expressions.
            Err(error) if starts_with_keyword(action_peek.fragment(), b"ref") => return Err(error),
            Err(_) => {}
        }
    }
    if starts_with_keyword(peek.fragment(), b"in")
        || starts_with_keyword(peek.fragment(), b"out")
        || starts_with_keyword(peek.fragment(), b"inout")
    {
        // `CalculationBodyItem = ActionBodyItem | ReturnParameterMember` keeps the directed
        // parameter spelling in its SysML action-owned production. Do this before the KerML
        // TypeBody fallback below: `calc def F { in p : T; }` must remain `InOutDecl`, whereas
        // `behavior B { in p : T; }` reaches `calc_def_body_element` and is a keyword-less
        // `Feature` (SysML BNF 1359-1370; KerML BNF 519-527, 562-608).
        // A direction may instead belong to a full occurrence usage. Preserve the pre-existing
        // `in calc …` / `in part …` action-body routes before testing the bare parameter
        // declaration; an `InOutDecl` parser is intentionally not allowed to treat either kind
        // keyword as a parameter name.
        if let Ok((next, usage)) = calc_usage(input) {
            let member = node_from_to(
                input,
                next,
                crate::ast::ActionDefBodyElement::CalcUsage(Box::new(usage)),
            );
            return Ok((
                next,
                node_from_to(
                    input,
                    next,
                    CalcDefBodyElement::ActionMember(Box::new(member)),
                ),
            ));
        }
        if let Ok((next, usage)) = crate::parser::part::part_usage(input) {
            return Ok((
                next,
                node_from_to(input, next, CalcDefBodyElement::PartUsage(Box::new(usage))),
            ));
        }
        let start = input;
        // Do not call the whole action dispatcher here: its directed `calc` usage probe may
        // accept the `calc` prefix of an ordinary parameter name before this production gets a
        // chance to retain the declaration. The grammar-owned parameter parser is already
        // transactional and supplies the exact `ActionDefBodyElement` alternative directly.
        //
        // A *kinded* directed parameter -- `in expr p : Boolean;`, `in bool redefines a;`,
        // `in feature p : Boolean;` -- is neither an `InOutDecl` nor a keyword-less `Feature`:
        // it is a KerML `Feature` whose kind keyword names its production
        // (`FeaturePrefix ( 'feature' | 'expr' | 'bool' | 'step' ) FeatureDeclaration`, KerML BNF
        // 562/863/895/908). `in_out_decl` deliberately refuses to read a kind keyword as a
        // parameter name, so it fails here rather than mis-parsing -- and committing to it with
        // `?` dropped the whole member to recovery. Fall through to the KerML route this body
        // already owns, which is what `behavior B { in expr p : Boolean; }` has always taken.
        let Ok((next, declaration)) = in_out_decl(input) else {
            return calc_def_body_element(input);
        };
        let member = node_from_to(
            start,
            next,
            crate::ast::ActionDefBodyElement::InOutDecl(declaration),
        );
        return Ok((
            next,
            node_from_to(
                start,
                next,
                CalcDefBodyElement::ActionMember(Box::new(member)),
            ),
        ));
    }
    calc_def_body_element(input)
}

/// Skips an optional `(private|protected|public)?` `abstract`? prefix and reports whether either
/// was present, for the dispatch-gate peeks below (`calc_usage`/`parse_calc_def` themselves also
/// consume `abstract`/visibility, so this only needs to decide *which* branch to try).
fn skip_calc_modifiers(input: Input<'_>) -> IResult<Input<'_>, bool> {
    let (input, (_, visibility)) = visibility_prefix(input)?;
    let (input, is_abstract) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    // `BasicUsagePrefix = RefPrefix ( isReference ?= 'ref' )?`. Without `ref` here the gate below
    // never fired for `ref calc self : Calculation :>> Action::self;` (Systems Library
    // `Calculations.sysml`), so the member fell through to the body's expression parser and the
    // bare word `ref` became an expression statement of its own.
    let (input, is_reference) = opt(preceded(tag(&b"ref"[..]), ws1)).parse(input)?;
    Ok((
        input,
        visibility.is_some() || is_abstract.is_some() || is_reference.is_some(),
    ))
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

/// One end of a connector/binding/succession member: `[mult]?` feature chain.
pub(crate) fn kerml_connector_end(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlConnectorEnd>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (input, target) =
        preceded(ws_and_comments, crate::parser::lex::reference_path).parse(input)?;
    // `references <chain>` / `::> <chain>` on the end itself: `from [0..*]
    // separateOccurrenceToo references elements.notIntersection to ...` (`Occurrences.kerml`),
    // `connector a ::> a.x to b;` (KerML class bodies).
    let (input, references) = opt(preceded(
        alt((
            map((ws_and_comments, tag(&b"references"[..]), ws1), |_| ()),
            map((ws_and_comments, tag(&b"::>"[..]), ws_and_comments), |_| ()),
        )),
        crate::parser::lex::reference_path,
    ))
    .parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::KermlConnectorEnd {
                multiplicity,
                target,
                references,
            },
        ),
    ))
}

/// KerML connector member: `connector` `all`? name? `:` type mult? (`from` end `to` end)? body,
/// e.g. `connector :HappensDuring from [1] self to [1] this;` or `private connector all during:
/// HappensDuring[0..1] from self to occ;` (Kernel Semantic Library `Occurrences.kerml`).
pub(crate) fn kerml_connector_member(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlConnectorMember>> {
    // Speculated at member starts it does not own; refuse unless one of this production's
    // leading words follows the trivia, before entering an arena transaction.
    {
        let (cursor, _) = ws_and_comments(input)?;
        if !(starts_with_keyword(cursor.fragment(), b"connector")
            || starts_with_keyword(cursor.fragment(), b"public")
            || starts_with_keyword(cursor.fragment(), b"private")
            || starts_with_keyword(cursor.fragment(), b"protected"))
        {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
    }
    crate::parser::span::reference_transaction(input, kerml_connector_member_inner)
}

fn kerml_connector_member_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlConnectorMember>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = tag(&b"connector"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, is_all) = opt(preceded(tag(&b"all"[..]), ws1)).parse(input)?;
    // Binary shorthand with no declaration at all: `connector [0..1] transitionLink to [1..*]
    // trigger;` (`TransitionPerformances.kerml`), `connector a ::> a.x to b;` (KerML class
    // bodies) -- the leading token is the first *end*, not the connector's name. Tried before
    // the named/typed declaration so an end name followed by `::>`/`to` is not misread as the
    // connector's own name.
    let (after_all_ws, _) = ws_and_comments(input)?;
    let from_keyword_next = starts_with_keyword(after_all_ws.fragment(), b"from");
    if let (false, Ok((rest, (from, to)))) = (
        from_keyword_next,
        map(
            (
                kerml_connector_end,
                preceded(ws_and_comments, tag(&b"to"[..])),
                ws1,
                kerml_connector_end,
            ),
            |(from, _, _, to)| (from, to),
        )
        .parse(input),
    ) {
        let (rest, body) = calc_def_body(rest)?;
        return Ok((
            rest,
            node_from_to(
                start,
                rest,
                crate::ast::KermlConnectorMember {
                    is_all: is_all.is_some(),
                    name: None,
                    typing: None,
                    multiplicity: None,
                    from: Some(from),
                    to: Some(to),
                    body,
                    membership: Membership::feature(visibility, visibility_span),
                },
            ),
        ));
    }
    // Name is optional: the common library shapes are the anonymous `connector :Type` and the
    // declaration-less `connector (all)? from a to b;`.
    let (peek, _) = ws_and_comments(input)?;
    let (input, name_str) = if peek.fragment().starts_with(b":")
        || peek.fragment().starts_with(b"[")
        || from_keyword_next
    {
        (input, None)
    } else {
        let (input, n) = preceded(ws_and_comments, name).parse(input)?;
        (input, Some(n))
    };
    let (input, typing) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_reference),
    ))
    .parse(input)?;
    let (input, multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    // Ends: `from end to end`, or the `from`-less binary shorthand `connector [0..1]
    // transitionLink to [1..*] trigger;` (`TransitionPerformances.kerml`).
    let (input, ends) = opt(map(
        (
            preceded(ws_and_comments, tag(&b"from"[..])),
            kerml_connector_end,
            preceded(ws_and_comments, tag(&b"to"[..])),
            kerml_connector_end,
        ),
        |(_, from, _, to)| (from, to),
    ))
    .parse(input)?;
    let (from, to) = match ends {
        Some((from, to)) => (Some(from), Some(to)),
        None => (None, None),
    };
    let (input, body) = calc_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::KermlConnectorMember {
                is_all: is_all.is_some(),
                name: name_str,
                typing,
                multiplicity,
                from,
                to,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// A declared binding head is accepted only when its following token proves that interpretation.
/// This transaction matters for `binding left = right;`: a multiplicity parsed while probing a
/// declaration may allocate reference identities, so falling back to the anonymous end pair must
/// roll those entries back before parsing the end for real.
fn kerml_binding_decl_head(
    input: Input<'_>,
) -> IResult<
    Input<'_>,
    (
        crate::ast::DeclarationName,
        Option<Node<crate::ast::Multiplicity>>,
    ),
> {
    let (input, declared_name) = preceded(ws_and_comments, name).parse(input)?;
    let (input, multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (next, _) = ws_and_comments(input)?;
    if !(starts_with_keyword(next.fragment(), b"of")
        || next.fragment().starts_with(b"{")
        || next.fragment().starts_with(b";"))
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    Ok((input, (declared_name, multiplicity)))
}

fn kerml_binding_end_pair(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlBindingEndPair>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let of_start = input;
    let (input, of_span) = if starts_with_keyword(input.fragment(), b"of") {
        let (rest, _) = tag(&b"of"[..]).parse(input)?;
        (rest, Some(crate::parser::span_from_to(of_start, rest)))
    } else {
        (input, None)
    };
    let (input, left) = kerml_connector_end(input)?;
    let (input, _) = ws_and_comments(input)?;
    let equals_start = input;
    let (input, _) = tag(&b"="[..]).parse(input)?;
    let equals_span = crate::parser::span_from_to(equals_start, input);
    let (input, right) = kerml_connector_end(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::KermlBindingEndPair {
                of_span,
                left,
                equals_span,
                right,
            },
        ),
    ))
}

/// KerML binding connector member: a declared feature with an optional inline binary pair, or an
/// optional `all` plus an optional anonymous pair, followed by a type body (KerML BNF 875-881).
pub(crate) fn kerml_binding_member(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlBindingMember>> {
    crate::parser::span::reference_transaction(input, kerml_binding_member_inner)
}

fn kerml_binding_member_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlBindingMember>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = tag(&b"binding"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (after_ws, _) = ws_and_comments(input)?;
    let all_start = after_ws;
    let (input, all_span) = if starts_with_keyword(after_ws.fragment(), b"all") {
        let (rest, _) = tag(&b"all"[..]).parse(after_ws)?;
        (rest, Some(crate::parser::span_from_to(all_start, rest)))
    } else {
        (after_ws, None)
    };
    let (input, declared) = if all_span.is_none() {
        match crate::parser::span::reference_transaction(input, kerml_binding_decl_head) {
            Ok((rest, declared)) => (rest, Some(declared)),
            Err(_) => (input, None),
        }
    } else {
        (input, None)
    };
    let (name_str, decl_multiplicity) = match declared {
        Some((declared_name, multiplicity)) => (Some(declared_name), multiplicity),
        None => (None, None),
    };
    let (next, _) = ws_and_comments(input)?;
    let body_next = next.fragment().starts_with(b"{") || next.fragment().starts_with(b";");
    let (input, inline_ends) = if body_next {
        (input, None)
    } else {
        let (input, pair) = kerml_binding_end_pair(input)?;
        if name_str.is_some() && pair.value.of_span.is_none() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
        (input, Some(pair))
    };
    let (input, body) = calc_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::KermlBindingMember {
                all_span,
                name: name_str,
                multiplicity: decl_multiplicity,
                inline_ends,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// KerML succession member: `succession` end `then` end `;`
/// (`succession [1] ifTest then [0..1] elseClause;`, `private succession [1] entry then [*]
/// middle;`). The named/typed succession forms stay on their existing parsers.
pub(crate) fn kerml_succession_member(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlSuccessionMember>> {
    crate::parser::span::reference_transaction(input, kerml_succession_member_inner)
}

fn kerml_succession_member_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlSuccessionMember>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = tag(&b"succession"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, is_all) = opt(preceded(tag(&b"all"[..]), ws1)).parse(input)?;
    // Named form, only with the `first` keyword: `succession triggerAfter [taNum] first
    // [0..1] transitionLinkSource then ...;` (`TransitionPerformances.kerml`).
    let (input, named) = opt(map(
        (
            preceded(ws_and_comments, name),
            opt(preceded(
                ws_and_comments,
                crate::parser::usage::multiplicity_node,
            )),
            preceded(ws_and_comments, tag(&b"first"[..])),
            ws1,
        ),
        |(n, m, _, _)| (n, m),
    ))
    .parse(input)?;
    let (name_str, decl_multiplicity) = match named {
        Some((n, m)) => (Some(n), m),
        None => (None, None),
    };
    let (input, first) = kerml_connector_end(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"then"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, then) = kerml_connector_end(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::KermlSuccessionMember {
                is_all: is_all.is_some(),
                name: name_str,
                multiplicity: decl_multiplicity,
                first,
                then,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// KerML end member with an owned cross feature: `end` name? mult? (`subsets` targets)?
/// `feature` <feature member> (`end happensDuring [1..*] feature longerOccurrence: Occurrence
/// redefines targetOccurrence;`, Kernel Semantic Library `Occurrences.kerml`). Plain
/// `end feature ...` stays on [`kerml_feature`] (its name parse fails on `feature`
/// here, so dispatch order tries that first).
/// Keyword-less named member binding in a type body: (visibility)? name mult? (`:` type)? mult?
/// (`:>>`/`redefines` targets)? value? `;` -- `private instantNum: Natural[1] = if isInstant? 1
/// else 0;`, `private thisClock : Clock :>> self;` (Kernel Semantic Library
/// `Occurrences.kerml`/`Observation.kerml`). Requires a typing clause, a redefinition, or a value,
/// so plain expression statements stay on the expression arm.
pub(crate) fn calc_named_binding(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::DefaultReferenceUsage>> {
    // Its head is a mandatory declared name token. The inner parser deliberately accepts a
    // reserved spelling there (dispatch order gives keyword-led siblings first refusal), so this
    // guard only mirrors the lexical shape: something name-like must lead at all.
    {
        let (cursor, _) = ws_and_comments(input)?;
        let (cursor, _) = visibility_prefix(cursor)?;
        let (cursor, _) = ws_and_comments(cursor)?;
        let head = cursor.fragment().first();
        let name_like = matches!(head, Some(b'\''))
            || head.is_some_and(|byte| byte.is_ascii_alphabetic() || *byte == b'_');
        if !name_like {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
    }
    crate::parser::span::reference_transaction(input, calc_named_binding_inner)
}

fn calc_named_binding_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::DefaultReferenceUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, name_str) = name(input)?;
    let (input, leading_multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    // `FeatureTyping`'s target list is comma-separated (`private y: A, '2'[0..*];`,
    // `Classes.kerml`), so the shared multi-target parser owns the clause.
    let (input, typing) = crate::parser::usage::optional_typings(input)?;
    let typing = typing.map(|(span, is_conjugated, targets, spelling)| {
        crate::ast::Node::new(
            span,
            crate::ast::TypingRelationship {
                target: targets,
                kind: crate::ast::TypingKind::Typing,
                span,
                is_conjugated,
                is_implied: false,
                spelling,
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
    let (input, redefines) = opt(preceded(ws_and_comments, redefinition)).parse(input)?;
    let (input, value) = opt(crate::parser::feature_value::feature_value_part).parse(input)?;
    // A declaration needs something an expression statement cannot have: a typing clause, a
    // redefinition, or a value. `mass : Real;` is the type-only form (`constraint c { mass :
    // Real; }`); a bare `mass` stays on the expression arm.
    if typing.is_none() && value.is_none() && redefines.is_none() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        )));
    }
    // This production accepts only the semicolon body alternative. Delegate its token capture to
    // the shared body parser so the serialized body delimiter remains source-backed.
    let (input, body) = semicolon_body(input)?;
    let typing_span = typing.as_ref().map(|t| t.span);
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::DefaultReferenceUsage {
                prefix: crate::ast::RefPrefix::default(),
                name: Some(name_str),
                short_name: None,
                typing,
                subsets: None,
                redefines,
                references: None,
                crosses: None,
                intersects: None,
                value,
                multiplicity: leading_multiplicity.or(trailing_multiplicity),
                multiplicity_modifiers: crate::ast::MultiplicityModifiers::default(),
                typing_span,
                membership: Membership::feature(visibility, visibility_span),
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
/// The feature-kind keyword of `Feature` (562), `Step` (863), `Expression` (895) and
/// `BooleanExpression` (908) -- the one column in which those four productions differ -- with its
/// exact span.
///
/// Optional, because the first alternative of `Feature` makes `FeatureDeclaration` optional and
/// the prefixed keyword-less spelling `portion redefines portionOfLife = ...;`
/// (`Occurrences.kerml`) reaches this node through a prefix alone. Takes trivia-free input.
fn feature_kind_keyword(
    input: Input<'_>,
) -> (Input<'_>, Option<Node<crate::ast::KermlFeatureKind>>) {
    for (keyword, value) in [
        (&b"feature"[..], crate::ast::KermlFeatureKind::Feature),
        (&b"step"[..], crate::ast::KermlFeatureKind::Step),
        (&b"expr"[..], crate::ast::KermlFeatureKind::Expr),
        (&b"bool"[..], crate::ast::KermlFeatureKind::Bool),
    ] {
        if let Some((rest, span)) = crate::parser::occurrence_prefix::slot_keyword(input, keyword) {
            return (rest, Some(Node::new(span, value)));
        }
    }
    (input, None)
}

/// Which member keyword a directed body element introduces, looking past the whole
/// `BasicFeaturePrefix` (KerML BNF 577) rather than just the direction.
///
/// Looking past the *whole* prefix is the point: `in derived feature q;`, `in composite feature
/// o;`, `in var feature p;` and `in portion feature s;` are all legal, and all reached recovery
/// while the directed spelling had its own node modelling only `direction` + `abstract`.
///
/// Returns `None` when no direction was authored, or when no member keyword follows it -- the
/// plain `in x;` parameter, which belongs to `InOutDecl`. Consumes nothing.
fn directed_member_keyword(input: Input<'_>) -> Option<&'static [u8]> {
    // A lookahead, so the prefix metadata run it walks past is allocated and discarded rather
    // than kept: the arm this guard picks re-parses the same bytes.
    crate::parser::span::reference_probe(input, |input| {
        let (input, _) = ws_and_comments(input).ok()?;
        let (rest, prefix) = crate::parser::feature_prefix::feature_prefix(input).ok()?;
        prefix.direction()?;
        [&b"feature"[..], b"step", b"expr", b"bool", b"calc"]
            .into_iter()
            .find(|keyword| starts_with_keyword(rest.fragment(), keyword))
    })
}

/// `OwnedCrossFeature : Feature = BasicFeaturePrefix FeatureDeclaration` (KerML BNF 595), carried
/// by `OwnedCrossFeatureMember` (592) between `end` and the feature's own kind keyword:
/// `end guardedLink [0..1] feature constrainedHBLink: HappensBefore;`
/// (`Kernel Semantic Library/TransitionPerformances.kerml:61`).
///
/// Fails -- so the caller keeps its input -- unless a kind keyword actually follows. That check is
/// the whole disambiguation, and it is forced by `Feature` (562) having a *keyword-less* second
/// alternative: `end x;` is `EndFeaturePrefix FeatureDeclaration`, a feature named `x` with no
/// cross, while `end x feature y;` is `FeaturePrefix 'feature' FeatureDeclaration` with `x` as the
/// cross. Only the trailing keyword tells them apart. Callers wrap this in
/// [`crate::parser::span::reference_transaction`] so a refused attempt rolls its arena entries
/// back.
fn owned_cross_feature(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::OwnedCrossFeature>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, prefix) = crate::parser::feature_prefix::basic_feature_prefix(input);
    // `FeatureIdentification` is optional here: `end [1] feature transferSource references
    // source;` (`Transfers.kerml`) crosses a multiplicity with no name.
    let (input, name_str) = if input.fragment().starts_with(b"[") {
        (input, None)
    } else {
        let (input, parsed) = name(input)?;
        (input, Some(parsed))
    };
    let (input, multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (input, multiplicity_modifiers) = crate::parser::usage::multiplicity_modifier_slots(input)?;
    let (input, subsets) =
        opt(preceded(ws_and_comments, crate::parser::usage::subsetting)).parse(input)?;
    let subsets = subsets.map(|(target, _value)| target);
    if name_str.is_none()
        && multiplicity.is_none()
        && subsets.is_none()
        && !prefix.is_authored()
        && !multiplicity_modifiers.is_authored()
    {
        // Nothing stands between `end` and the keyword, so there is no cross feature to own.
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    // The cross feature ends where the owning feature's kind keyword begins, so the trivia between
    // them is consumed here: every slot parser downstream takes trivia-free input.
    let (rest, _) = ws_and_comments(input)?;
    if !starts_with_any_keyword(
        rest.fragment(),
        &[&b"feature"[..], b"step", b"expr", b"bool"],
    ) {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    Ok((
        rest,
        node_from_to(
            start,
            input,
            crate::ast::OwnedCrossFeature {
                prefix,
                name: name_str,
                multiplicity,
                multiplicity_modifiers,
                subsets,
            },
        ),
    ))
}

/// One `TypeRelationshipPart` in a KerML feature declaration tail.
///
/// This deliberately consumes one clause only. Its caller owns
/// `FeatureRelationshipPart*`, where type relationships can interleave with
/// `chains`, `inverse of`, and `featured by`.
fn kerml_feature_type_relationship_part(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlTypeRelationship>> {
    let start = input;
    let (input, keyword) = if starts_with_keyword(input.fragment(), b"disjoint") {
        let (input, _) = tag(&b"disjoint"[..]).parse(input)?;
        let (input, _) = ws1(input)?;
        let (input, _) = tag(&b"from"[..]).parse(input)?;
        let (input, _) = ws1(input)?;
        (
            input,
            crate::ast::KermlTypeRelationshipKeyword::DisjointFrom,
        )
    } else if starts_with_keyword(input.fragment(), b"unions") {
        let (input, _) = tag(&b"unions"[..]).parse(input)?;
        let (input, _) = ws1(input)?;
        (input, crate::ast::KermlTypeRelationshipKeyword::Unions)
    } else if starts_with_keyword(input.fragment(), b"intersects") {
        let (input, _) = tag(&b"intersects"[..]).parse(input)?;
        let (input, _) = ws1(input)?;
        (input, crate::ast::KermlTypeRelationshipKeyword::Intersects)
    } else if starts_with_keyword(input.fragment(), b"differences") {
        let (input, _) = tag(&b"differences"[..]).parse(input)?;
        let (input, _) = ws1(input)?;
        (input, crate::ast::KermlTypeRelationshipKeyword::Differences)
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };
    let (input, first) = qualified_reference(input)?;
    let (input, more) = nom::multi::many0(preceded(
        preceded(ws_and_comments, tag(&b","[..])),
        preceded(ws_and_comments, qualified_reference),
    ))
    .parse(input)?;
    let mut targets = vec![first];
    targets.extend(more);
    let span = crate::parser::span_from_to(start, input);
    Ok((
        input,
        Node::new(
            span,
            crate::ast::KermlTypeRelationship {
                keyword,
                targets,
                span,
            },
        ),
    ))
}

/// `FeatureRelationshipPart*` on a KerML `FeatureDeclaration`.
///
/// KerML permits all alternatives in arbitrary authored order and permits
/// repetitions. The AST therefore retains one ordered, exhaustive list rather
/// than independent fixed slots for selected alternatives.
fn kerml_feature_relationship_parts(
    input: Input<'_>,
    intersecting: Option<Node<crate::ast::SubsettingRelationship>>,
) -> IResult<Input<'_>, Vec<Node<crate::ast::FeatureRelationshipPart>>> {
    let mut parts = Vec::new();
    if let Some(intersecting) = intersecting {
        let span = intersecting.span;
        let relationship = Node::new(
            span,
            crate::ast::KermlTypeRelationship {
                keyword: crate::ast::KermlTypeRelationshipKeyword::Intersects,
                targets: intersecting.value.target,
                span,
            },
        );
        parts.push(Node::new(
            span,
            crate::ast::FeatureRelationshipPart::TypeRelationship(relationship),
        ));
    }

    let mut input = input;
    loop {
        let (after_ws, _) = ws_and_comments(input)?;
        if starts_with_keyword(after_ws.fragment(), b"chains") {
            let start = after_ws;
            let (rest, _) = tag(&b"chains"[..]).parse(after_ws)?;
            let (rest, _) = ws1(rest)?;
            let (rest, target) = crate::parser::lex::reference_path(rest)?;
            parts.push(Node::new(
                crate::parser::span_from_to(start, rest),
                crate::ast::FeatureRelationshipPart::Chaining { target },
            ));
            input = rest;
        } else if starts_with_keyword(after_ws.fragment(), b"inverse") {
            let start = after_ws;
            let (rest, _) = tag(&b"inverse"[..]).parse(after_ws)?;
            let (rest, _) = ws1(rest)?;
            let (rest, _) = tag(&b"of"[..]).parse(rest)?;
            let (rest, _) = ws1(rest)?;
            let (rest, target) = crate::parser::lex::reference_path(rest)?;
            parts.push(Node::new(
                crate::parser::span_from_to(start, rest),
                crate::ast::FeatureRelationshipPart::Inverting { target },
            ));
            input = rest;
        } else if starts_with_keyword(after_ws.fragment(), b"featured") {
            let start = after_ws;
            let (rest, _) = tag(&b"featured"[..]).parse(after_ws)?;
            let (rest, _) = ws1(rest)?;
            let (rest, _) = tag(&b"by"[..]).parse(rest)?;
            let (rest, _) = ws1(rest)?;
            let (rest, first) = qualified_reference(rest)?;
            let (rest, more) = nom::multi::many0(preceded(
                preceded(ws_and_comments, tag(&b","[..])),
                preceded(ws_and_comments, qualified_reference),
            ))
            .parse(rest)?;
            let mut targets = vec![first];
            targets.extend(more);
            let span = crate::parser::span_from_to(start, rest);
            let featuring = Node::new(span, crate::ast::TypeFeaturingPart { targets });
            parts.push(Node::new(
                span,
                crate::ast::FeatureRelationshipPart::TypeFeaturing(featuring),
            ));
            input = rest;
        } else if starts_with_any_keyword(
            after_ws.fragment(),
            &[b"disjoint", b"unions", b"intersects", b"differences"],
        ) {
            let (rest, relationship) = kerml_feature_type_relationship_part(after_ws)?;
            let span = relationship.span;
            parts.push(Node::new(
                span,
                crate::ast::FeatureRelationshipPart::TypeRelationship(relationship),
            ));
            input = rest;
        } else {
            return Ok((input, parts));
        }
    }
}

pub(crate) fn kerml_feature(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlFeature>> {
    // The kind keyword is optional when a `FeaturePrefix` marks the member, so the guard asks
    // whether any word this production can begin with leads; a `true` only admits the parser.
    {
        const STARTERS: &[&[u8]] = &[
            b"public",
            b"private",
            b"protected",
            b"member",
            b"feature",
            b"step",
            b"expr",
            b"bool",
            b"in",
            b"out",
            b"inout",
            b"composite",
            b"const",
            b"derived",
            b"var",
            b"portion",
            b"abstract",
            b"end",
            b"readonly",
        ];
        let (cursor, _) = ws_and_comments(input)?;
        let fragment = cursor.fragment();
        if !STARTERS
            .iter()
            .any(|keyword| starts_with_keyword(fragment, keyword))
        {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
    }
    crate::parser::span::reference_transaction(input, kerml_feature_inner)
}

fn kerml_feature_inner(input: Input<'_>) -> IResult<Input<'_>, Node<crate::ast::KermlFeature>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, is_member) = opt(preceded(tag(&b"member"[..]), ws1)).parse(input)?;
    // `FeaturePrefix` (KerML BNF 584). `member` is not part of it -- `TypeFeatureMember` (523)
    // puts that keyword on the membership, ahead of the whole prefix.
    let (input, _) = ws_and_comments(input)?;
    let (input, mut prefix) = crate::parser::feature_prefix::feature_prefix(input)?;
    // `OwnedCrossFeatureMember` (592) hangs off the `end` alternative only, and needs the
    // declaration parsers this scope already has, so the choice is filled in here.
    let input = if let crate::ast::FeaturePrefixHead::End { cross, .. } = &mut prefix.head {
        match crate::parser::span::reference_transaction(input, owned_cross_feature) {
            Ok((rest, parsed)) => {
                *cross = Some(Box::new(parsed));
                rest
            }
            Err(_) => input,
        }
    } else {
        input
    };
    let had_prefix = is_member.is_some() || prefix.is_authored();
    // The kind keyword is optional when a prefix already marks this as a feature member:
    // `portion redefines portionOfLife = (that as Occurrence).portionOfLife;`
    // (`Occurrences.kerml`).
    let (input, kind) = feature_kind_keyword(input);
    if kind.is_none() && !had_prefix {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, is_all) = opt(preceded(tag(&b"all"[..]), ws1)).parse(input)?;
    // Leading redefinition may replace the name: `portion feature redefines spaceBoundary [1];`.
    let (input, leading_redefines) = opt(preceded(ws_and_comments, redefinition)).parse(input)?;
    // The name itself is also optional when a specialization/typing/multiplicity/body follows
    // directly: `step :> monitor.startObservation { ... }`, `private composite step : add
    // { ... }` (`Observation.kerml`, `SequenceFunctions.kerml`).
    let (input, name_str) = if leading_redefines.is_some() {
        (input, None)
    } else {
        let (peek, _) = ws_and_comments(input)?;
        if peek.fragment().starts_with(b":")
            || peek.fragment().starts_with(b"[")
            || peek.fragment().starts_with(b"{")
        {
            (input, None)
        } else {
            let (input, n) = preceded(ws_and_comments, name).parse(input)?;
            (input, Some(n))
        }
    };
    let (input, leading_multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (input, early_specializations) = crate::parser::usage::feature_specializations(input)?;
    let (input, trailing_multiplicity) = if leading_multiplicity.is_none() {
        opt(preceded(
            ws_and_comments,
            crate::parser::usage::multiplicity_node,
        ))
        .parse(input)?
    } else {
        (input, None)
    };
    let (input, modifiers) = crate::parser::usage::multiplicity_modifier_slots(input)?;
    let (input, trailing_specializations) = crate::parser::usage::feature_specializations(input)?;
    // `FeatureSpecializationPart` offers a second multiplicity position, after the specialization
    // clauses: `inout feature replacementValues : Anything redefines values [*] nonunique;`
    // (Kernel Semantic Library `FeatureReferencingPerformances.kerml:89`). Only the directed node
    // this one absorbed used to read that position, so it has to be read here or the spelling
    // regresses to recovery.
    let (input, post_specialization_multiplicity) =
        if leading_multiplicity.is_none() && trailing_multiplicity.is_none() {
            opt(preceded(
                ws_and_comments,
                crate::parser::usage::multiplicity_node,
            ))
            .parse(input)?
        } else {
            (input, None)
        };
    // Threaded as an accumulator rather than parsed twice and folded: each `MultiplicityPart`
    // keyword slot holds one authored spelling, and a second independent read would silently drop
    // whichever position lost.
    let (input, modifiers) =
        crate::parser::usage::multiplicity_modifier_slots_after(modifiers, input)?;
    let mut specializations = Vec::with_capacity(
        usize::from(leading_redefines.is_some())
            + early_specializations.len()
            + trailing_specializations.len(),
    );
    if let Some(relationship) = leading_redefines {
        specializations.push(crate::ast::FeatureSpecialization::Redefinition(
            relationship,
        ));
    }
    specializations.extend(early_specializations);
    specializations.extend(trailing_specializations);
    let (input, relationship_parts) = kerml_feature_relationship_parts(input, None)?;
    let (input, value) = opt(crate::parser::feature_value::feature_value_part).parse(input)?;
    let (input, body) = calc_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::KermlFeature {
                is_member: is_member.is_some(),
                prefix,
                kind,
                is_all: is_all.is_some(),
                name: name_str,
                specializations,
                multiplicity: leading_multiplicity
                    .or(trailing_multiplicity)
                    .or(post_specialization_multiplicity),
                multiplicity_modifiers: modifiers,
                relationship_parts,
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
                name: name_str,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// The `OccurrenceUsagePrefix` slots that may precede `part` here and head no production this
/// scope claims first. `ref` and `#` are deliberately absent: they head `ReferenceUsage` and
/// `PrefixMetadataMember`, and `occurrence_prefix::starts_contended_prefix` is what decides
/// whether a run reaching one of them is a prefix or a member of its own.
const PART_USAGE_PREFIX_STARTERS: &[&[u8]] = &[
    b"abstract",
    b"constant",
    b"derived",
    b"in",
    b"individual",
    b"inout",
    b"out",
    b"snapshot",
    b"timeslice",
    b"variation",
];

fn calc_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<CalcDefBodyElement>> {
    let start = input;
    // Member boundary: `ws_and_notes` leaves a bare `/* ... */` for this scope's
    // annotating member, which is the `Comment` production's keyword-less spelling.
    let (input, _) = crate::parser::lex::ws_and_notes(input)?;
    // Ahead of every guard below, including the two computed eagerly: they all reach a parser
    // that skips `/* ... */` as trivia, which then reads the *following* member's declaration as
    // its own. `/* c */ feature f : T unions x;` lost the comment, took `feature f` as a
    // declaration whose specialization tail no longer fit, and dropped the member into recovery.
    // See `body::starts_bare_comment`.
    if crate::parser::body::starts_bare_comment(input) {
        let comment_start = input;
        let (input, member) = crate::parser::body::annotating_member(comment_start)?;
        let elem = CalcDefBodyElement::Annotating(member);
        return Ok((input, node_from_to(comment_start, input, elem)));
    }
    // Keyword dispatch looks past an optional visibility prefix (`private attribute ...`,
    // `private connector all ...`); each arm's parser re-parses the prefix itself.
    let after_visibility = crate::parser::lex::visibility_prefix(input)
        .map(|(rest, _)| *rest.fragment())
        .unwrap_or_else(|_| *input.fragment());
    let prefixed_metadata_feature = (input.fragment().starts_with(b"#")
        || starts_with_keyword(input.fragment(), b"metadata"))
    .then(|| crate::parser::metadata_annotation::metadata_annotation(input))
    .and_then(Result::ok);
    // `CalculationBodyItem -> ActionBodyItem -> NonBehaviorBodyItem -> StructureUsageMember`
    // (SysML BNF 1366, 901, 910) admits a `PartUsage` here, and this scope had no arm for one:
    // `part p;` fell through to the bare-expression fallback and was shredded into two
    // expressions -- `'part';` and `p;` -- with no diagnostic, which a round trip then wrote
    // back out as two members.
    //
    // Tried before every other arm because the whole `OccurrenceUsagePrefix` may precede the
    // kind keyword, and two of its FIRST tokens (`#`, `ref`) head productions the arms below
    // claim first. The guard keeps the attempt off members that cannot be one -- and the attempt
    // itself is transactional, so a member that really is a metadata annotation or a KerML
    // feature falls through unchanged.
    let after_visibility_input = crate::parser::lex::visibility_prefix(input)
        .map(|(rest, _)| rest)
        .unwrap_or(input);
    let part_usage_member = (starts_with_keyword(after_visibility, b"part")
        || starts_with_any_keyword(after_visibility, PART_USAGE_PREFIX_STARTERS)
        || crate::parser::occurrence_prefix::starts_contended_prefix(after_visibility_input))
    .then(|| crate::parser::part::part_usage(input))
    .and_then(Result::ok);
    let (input, elem) = if let Some((next, usage)) = part_usage_member {
        (next, CalcDefBodyElement::PartUsage(Box::new(usage)))
    } else if let Some((next, annotation)) = prefixed_metadata_feature {
        (
            next,
            CalcDefBodyElement::Annotating(crate::ast::AnnotatingMember::MetadataAnnotation(
                annotation,
            )),
        )
    } else if starts_with_keyword(after_visibility, b"package")
        || starts_with_keyword(after_visibility, b"library")
        || starts_with_keyword(after_visibility, b"standard")
    {
        let (next, member) = calc_def_body_package_member(input)?;
        (next, member)
    } else if crate::parser::lex::starts_with_any_keyword(
        after_visibility,
        &[
            b"specialization",
            b"subclassifier",
            b"subtype",
            b"subset",
            b"redefinition",
            b"typing",
            b"conjugation",
            b"conjugate",
            b"disjoining",
            b"disjoint",
            b"inverting",
            b"inverse",
            b"featuring",
        ],
    ) {
        map(crate::parser::package::kerml_relationship_decl, |node| {
            CalcDefBodyElement::KermlRelationship(Box::new(node))
        })
        .parse(input)?
    } else if starts_with_keyword(input.fragment(), b"doc")
        || starts_with_keyword(input.fragment(), b"comment")
        || starts_with_keyword(input.fragment(), b"rep")
        || starts_with_keyword(input.fragment(), b"language")
        || input.fragment().starts_with(b"@")
    {
        map(
            crate::parser::body::annotating_member,
            CalcDefBodyElement::Annotating,
        )
        .parse(input)?
    } else if input.fragment().starts_with(b"#") {
        // Both `#` productions: the `ExtendedUsage` member spelling (which owns a `;`/`{}`
        // body) is tried before the `PrefixMetadataMember` spelling, which owns no body and
        // leaves the prefixed declaration for the next member iteration.
        alt((
            map(
                crate::parser::metadata_annotation::metadata_keyword_usage,
                CalcDefBodyElement::MetadataKeywordUsage,
            ),
            map(
                crate::parser::metadata_annotation::metadata_keyword_prefix,
                CalcDefBodyElement::MetadataKeywordUsage,
            ),
        ))
        .parse(input)?
    } else if starts_with_keyword(after_visibility, b"import") {
        map(crate::parser::import::import_, |n| {
            CalcDefBodyElement::Import(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_keyword(after_visibility, b"alias") {
        // KerML `TypeBodyElement` admits `AliasMember` directly (textual BNF 431-438), and a
        // SysML `CalculationBody` reaches the same member through `ActionBodyItem ->
        // NonBehaviorBodyItem` (SysML textual BNF 1359-1368, 901-917). `alias_def` owns the
        // source-backed target and runs its qualified-reference allocation transaction before
        // this scope accepts the member.
        map(
            crate::parser::alias::alias_def,
            CalcDefBodyElement::AliasDef,
        )
        .parse(input)?
    } else if starts_with_keyword(after_visibility, b"connector") {
        map(kerml_connector_member, |n| {
            CalcDefBodyElement::Connector(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_keyword(after_visibility, b"binding") {
        map(kerml_binding_member, |n| {
            CalcDefBodyElement::Binding(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_keyword(after_visibility, b"succession") {
        map(kerml_succession_member, |n| {
            CalcDefBodyElement::Succession(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_keyword(after_visibility, b"flow") {
        // `TypeBodyElement -> FeatureMember -> OwnedFeatureMember -> FeatureElement -> Flow`
        // (KerML BNF 434, 519, 526, 360/369, 1303), whose `FlowDeclaration` (BNF 1311) has the
        // endpoint-only spelling `FlowEndMember 'to' FlowEndMember`. This scope had no arm for
        // it, so `flow a.y to b.x1;` in a `classifier`/`struct`/`class`/`behavior`/`datatype`/
        // `function` body reached the terminal bare-expression fallback and was shredded into
        // four members -- `'flow';`, `a.y;`, `'to';`, `b.x1;` -- with no diagnostic at all.
        //
        // `flow_usage_member` is the parser every SysML sibling scope already dispatches for the
        // identical surface syntax. Only the `flow` keyword routes here, so its two other
        // spellings are untouched: `message` is SysML-only (`Message`, SysML BNF 805, which no
        // KerML `FeatureElement` alternative reaches) and reaches a SysML calculation body one
        // level earlier through `CALCULATION_ACTION_STARTERS`; `succession flow` (KerML
        // `SuccessionFlow`, BNF 370/1307) is still claimed by the `succession` arm above, which
        // is a pre-existing gap this slice does not widen.
        map(crate::parser::flow::flow_usage_member, |n| {
            CalcDefBodyElement::FlowUsage(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_keyword(after_visibility, b"attribute") {
        map(crate::parser::attribute::attribute_usage, |n| {
            CalcDefBodyElement::AttributeUsage(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_keyword(input.fragment(), b"redefines") {
        // Keyword-led, nameless redefinition: `redefines predecessors [0];`. A `Feature` (KerML
        // BNF 562) may be nothing but its `FeatureDeclaration` (601), and that declaration's
        // second alternative is a bare `FeatureSpecializationPart` (632) = `FeatureSpecialization+
        // MultiplicityPart?`, reaching `Redefinitions` (663) -> `Redefines` (666) = `REDEFINES
        // OwnedRedefinition`. `REDEFINES` is spelled `:>>` or `redefines`.
        //
        // The `:>>` spelling had an arm below; the word spelling had none, so `redefines
        // predecessors [0];` was shredded into `'redefines';` and `predecessors ['0'];` with no
        // diagnostic. `redefinition_feature_binding` is the parser an attribute body, an
        // occurrence body and a constraint body already dispatch for this exact production, and
        // it demands the `:>>`/`:>`/`redefines` head, so it cannot shadow the keyword arms.
        map(
            crate::parser::attribute::redefinition_feature_binding,
            |n| CalcDefBodyElement::AttributeUsage(Box::new(n)),
        )
        .parse(input)?
    } else if input.fragment().starts_with(b":>>") {
        // Anonymous leading-redefinition binding: `:>> dimension = size(components);`
        // (`VectorValues.kerml`).
        map(
            crate::parser::attribute::default_reference_value_binding,
            |n| CalcDefBodyElement::DefaultReferenceUsage(Box::new(n)),
        )
        .parse(input)?
    } else if crate::parser::package::starts_abstract_kerml_classifier(input) {
        // `abstract` is both BasicFeaturePrefix's third slot (KerML BNF 577) and an optional
        // classifier modifier. Resolve that FIRST-set overlap before the feature arm: an
        // `abstract struct/class/...` must remain a classifier, while an `abstract end feature`
        // chain below is malformed FeaturePrefix input and belongs to recovery.
        map(crate::parser::package::kerml_classifier_structured, |n| {
            CalcDefBodyElement::KermlClassifier(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_any_keyword(
        after_visibility,
        &[
            b"member",
            b"derived",
            b"composite",
            b"portion",
            b"var",
            b"feature",
            b"step",
            b"bool",
            b"expr",
        ],
    ) || (starts_with_any_keyword(after_visibility, &[b"abstract", b"end", b"const"])
        && (kerml_feature(input).is_ok()
            || malformed_feature_prefix_continuation(after_visibility_input)))
    {
        map(kerml_feature, |n| {
            CalcDefBodyElement::KermlFeature(Box::new(n))
        })
        .parse(input)?
    // `Invariant` is a `FeatureElement`, so the optional `MemberPrefix` visibility belongs to
    // this member just as it does to the feature arm above (KerML BNF 519-527, 913-917). Dispatch
    // must inspect beyond that prefix: otherwise `private inv { ... }` falls through to the
    // expression fallback, which turns `private` into a sibling feature reference.
    } else if starts_with_keyword(after_visibility, b"inv") {
        map(kerml_invariant_member, |n| {
            CalcDefBodyElement::Invariant(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_keyword(input.fragment(), b"assert") {
        map(
            crate::parser::occurrence_body::assert_constraint_member,
            |n| CalcDefBodyElement::AssertConstraint(Box::new(n)),
        )
        .parse(input)?
    } else if starts_with_any_keyword(
        after_visibility,
        &[
            b"struct",
            b"datatype",
            b"metaclass",
            b"assoc",
            b"association",
            // `class` joined this list when `ClassDef` was deleted: it is an ordinary
            // `KermlClassifierDecl` keyword like its siblings, and a nested `class Inner { ... }`
            // inside a type body had been reaching the old bespoke node instead.
            b"class",
            b"classifier",
            b"function",
            b"behavior",
            b"predicate",
            b"interaction",
        ],
    ) {
        // Nested classifier declarations inside a type body (`struct StructuredSurface
        // specializes ... { ... }` inside a struct, `Objects.kerml`).
        map(crate::parser::package::kerml_classifier_structured, |n| {
            CalcDefBodyElement::KermlClassifier(Box::new(n))
        })
        .parse(input)?
    } else if starts_with_keyword(input.fragment(), b"in")
        || starts_with_keyword(input.fragment(), b"out")
        || starts_with_keyword(input.fragment(), b"inout")
    {
        // A directed `in part …` was claimed here before the scope had a `PartUsage` arm at all;
        // the arm above owns every part usage now, directed or not.
        match directed_member_keyword(input) {
            // `in calc scenario : NominalScenario;` (validation `10c-Fuel Economy Analysis`) is a
            // SysML `CalculationUsage = OccurrenceUsagePrefix 'calc' …` (SysML BNF 1355), not a
            // KerML `Feature`. It reached the directed-parameter node only because this arm ran
            // before the `calc` arm below; `CalcUsage` already models direction, `abstract` and
            // `ref` through `RefPrefix`, so the production keeps its own node.
            Some(keyword) if keyword == b"calc" => {
                map(calc_usage, |n| CalcDefBodyElement::CalcUsage(Box::new(n))).parse(input)?
            }
            // `in expr …` / `in bool …` / `in feature …` / `in step …` are `FeaturePrefix`'s
            // direction slot in front of the same four productions the undirected spelling uses
            // (KerML BNF 577/562/863/895/908), so they are the same node.
            Some(_) => map(kerml_feature, |n| {
                CalcDefBodyElement::KermlFeature(Box::new(n))
            })
            .parse(input)?,
            // The keyword-less `Feature` alternative is the remaining production under this
            // KerML TypeBody dispatcher: `in value : T;`, `out result;`, and `in :>> inherited;`
            // are `BasicFeaturePrefix FeatureDeclaration` (KerML BNF 562-569, 577-608), not the
            // SysML action-body parameter shorthand. Re-parse from the original input through
            // the transactional feature parser so every prefix/declaration clause owns its
            // source-backed span. `calculation_body` keeps its separate ActionBody dispatch and
            // therefore retains `InOutDecl` for SysML calculation members.
            None => map(kerml_feature, |n| {
                CalcDefBodyElement::KermlFeature(Box::new(n))
            })
            .parse(input)?,
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
            (input, CalcDefBodyElement::ReturnDecl(Box::new(decl)))
        } else if let Ok((input, expr)) = return_expression_stmt(input) {
            (input, CalcDefBodyElement::Expression(expr))
        } else {
            let (next, node) = other_return_recovery(
                input,
                CALC_DEF_BODY_STARTERS,
                "calc body",
                "recovered_calc_body_element",
            )?;
            (next, CalcDefBodyElement::Error(node))
        }
    // `Message` is a SysML `StructureUsageElement` (SysML BNF 805), not a KerML
    // `FeatureElement`.  A CalculationBody reaches it through ActionBodyItem above, but a
    // KerML TypeBody must reject it before the keyword-less expression fallback can shred its
    // declaration into `message`, name, `of`, and type expressions.  Use the original input so
    // recovery retains a single exact source span, including a leading MemberPrefix.
    } else if starts_with_keyword(after_visibility, b"message") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    } else if let Ok((next, binding)) = calc_named_binding(input) {
        // Keyword-less named member binding (`private instantNum: Natural[1] = ...;`,
        // `private thisClock : Clock :>> self;`) -- tried before the bare-expression arm,
        // which cannot represent the typing/redefinition. `calc_named_binding` requires a
        // value or redefinition, so plain expression statements never land here.
        (
            next,
            CalcDefBodyElement::DefaultReferenceUsage(Box::new(binding)),
        )
    } else {
        map(expression, CalcDefBodyElement::Expression).parse(input)?
    };
    let (input, _) = opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// Whether a reserved `FeaturePrefix` head is immediately followed by a token which makes its
/// ordered production malformed.
///
/// `FeaturePrefix`'s `BasicFeaturePrefix` accepts `abstract`, but its ordered slot sequence does
/// not allow the earlier `derived`/direction slots or the separate `EndFeaturePrefix` afterward.
/// Conversely, `EndFeaturePrefix` spells only optional `const` *before* its required `end`, so a
/// BasicFeaturePrefix slot after `end` is invalid. If either chain is not dispatched to
/// [`kerml_feature`], the terminal expression fallback fabricates a feature reference for its
/// first keyword and parses the rest as a second member. This lookahead is intentionally lexical:
/// it never allocates or accepts syntax; the owning, transactional feature parser still decides
/// whether the complete production is valid.
fn malformed_feature_prefix_continuation(input: Input<'_>) -> bool {
    if let Some((input, _)) = crate::parser::occurrence_prefix::slot_keyword(input, b"abstract") {
        return starts_with_any_keyword(
            input.fragment(),
            &[
                b"member",
                b"in",
                b"out",
                b"inout",
                b"derived",
                b"abstract",
                b"composite",
                b"portion",
                b"var",
                b"const",
                b"end",
                b"feature",
                b"step",
                b"expr",
                b"bool",
            ],
        );
    }
    if let Some((input, _)) = crate::parser::occurrence_prefix::slot_keyword(input, b"end") {
        return starts_with_any_keyword(
            input.fragment(),
            &[
                b"in",
                b"out",
                b"inout",
                b"derived",
                b"abstract",
                b"composite",
                b"portion",
                b"var",
                b"const",
            ],
        );
    }
    false
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

/// A `return` member that parses as neither a [`return_decl`] nor a return expression becomes
/// an explicit recovery node with a diagnostic (previously a diagnostic-silent opaque
/// `Other(preview)` capture).
///
/// The owning scope supplies its own starters, description and diagnostic code, because both
/// scopes that dispatch a `ReturnParameterMember` -- the calculation body and the constraint
/// body it shares `CalculationBody` with -- must report the failure as their own member.
fn other_return_recovery<'a>(
    input: Input<'a>,
    starters: &[&[u8]],
    scope: &str,
    code: &'static str,
) -> IResult<Input<'a>, Node<ParseErrorNode>> {
    let start_unknown = input;
    let (next, _) = skip_statement_or_block(input)?;
    if next.location_offset() == start_unknown.location_offset() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Many0,
        )));
    }
    let recovery = build_recovery_error_node_from_span(start_unknown, next, starters, scope, code);
    Ok((next, node_from_to(start_unknown, next, recovery)))
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
    // Kind keyword: `return attribute verdict : VerdictKind = ...;` / `return feature
    // timeSignal : TimeSignal[1] = ...` (`Observation.kerml`, `Triggers.kerml`).
    let (input, kind_keyword) = opt(alt((
        map(preceded(tag(&b"attribute"[..]), ws1), |_| {
            crate::ast::ReturnKindKeyword::Attribute
        }),
        map(preceded(tag(&b"feature"[..]), ws1), |_| {
            crate::ast::ReturnKindKeyword::Feature
        }),
    )))
    .parse(input)?;
    let (input, short_name) = crate::parser::lex::short_name_prefix(input)?;
    let (input, _) = ws_and_comments(input)?;
    // A leading `return :>> <target> ...` names a semantic redefinition target -- which may be
    // `::`-qualified (`return :>> ISQ::power = ...;`, Apollo 11) -- not a new declaration, so it
    // is parsed as a reference and stored on `redefines`, mirroring `case_return_decl_inner`.
    // `name()` (a single identifier) previously truncated `ISQ::power` to `ISQ` and then failed.
    let redefine_target_start = input;
    let (input, n, leading_redefines) = if is_redefine {
        let (input, target) = qualified_reference(input)?;
        let span = crate::parser::span_from_to(redefine_target_start, input);
        (
            input,
            None,
            Some(crate::parser::usage::subsetting_relationship_node(
                vec![target],
                crate::ast::SubsettingKind::Redefines,
                span,
            )),
        )
    } else if input.fragment().starts_with(b":") {
        // Anonymous typed/subsetted form: `return : Type [= expr];` or `return :> Feature
        // [= expr];` (`return :> ISQ::power = system.subparts->select { ... };`, Apollo 11).
        // `:>>` was already consumed by the `is_redefine` branch above, so a leading `:` here
        // is either `:` or `:>`; both are handled by the `is_subsetting` block below, and
        // neither declares a name -- falling through to `name()` would try to parse `:>` as an
        // identifier and fail.
        (input, None, None)
    } else {
        let (input, n) = name(input)?;
        (input, Some(n), None)
    };
    let (input, _) = ws_and_comments(input)?;
    // The typing is optional on named results: `return result [1..1];`, `return sampling =
    // new SampledFunction(...);` (Domain Libraries `SampledFunctions.sysml`).
    let (input, is_subsetting, type_name) =
        if input.fragment().starts_with(b":>") && !input.fragment().starts_with(b":>>") {
            let (input, _) = tag(&b":>"[..]).parse(input)?;
            let (input, target) = preceded(ws_and_comments, qualified_reference).parse(input)?;
            (input, true, Some(target))
        } else if input.fragment().starts_with(b":") {
            let (input, _) = tag(&b":"[..]).parse(input)?;
            let (input, target) = preceded(ws_and_comments, qualified_reference).parse(input)?;
            (input, false, Some(target))
        } else if n.is_none() && leading_redefines.is_none() {
            // Nothing declared at all -- not a return declaration.
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        } else {
            (input, false, None)
        };
    // Multiplicity and `ordered`/`nonunique` after the type: `return : Real[1] = x;`,
    // `return : Anything[0..*] ordered nonunique;` (Kernel Function Library).
    let (input, multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (input, modifiers) = crate::parser::usage::multiplicity_modifier_slots(input)?;
    // Trailing redefinitions; repeated clauses merge targets (`... redefines result redefines
    // values;`, `FeatureReferencingPerformances.kerml`).
    let mut input = input;
    let mut redefines: Option<Node<crate::ast::SubsettingRelationship>> = leading_redefines;
    while let Ok((rest, clause)) = preceded(ws_and_comments, redefinition).parse(input) {
        match &mut redefines {
            None => redefines = Some(clause),
            Some(existing) => {
                existing.value.target.extend(clause.value.target);
            }
        }
        input = rest;
    }
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
                short_name,
                kind_keyword,
                name: n,
                type_name,
                is_redefine,
                is_subsetting,
                multiplicity,
                multiplicity_modifiers: modifiers,
                redefines,
                value,
                body,
            },
        ),
    ))
}

#[cfg(test)]
mod calc_named_binding_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    /// This keyword-less type-body binding owns an `AttributeBody`; its semicolon must retain the
    /// authored delimiter for serialized provenance validation.
    #[test]
    fn calc_named_binding_keeps_its_semicolon_body_span() {
        let source = "private thisClock : Clock :>> self;";
        let (rest, binding) = calc_named_binding(input(source)).expect("named binding");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let crate::ast::AttributeBody::Semicolon { semicolon_span } = &binding.value.body else {
            panic!("expected a semicolon body");
        };
        assert_eq!(
            &source[semicolon_span.offset..semicolon_span.offset + semicolon_span.len],
            ";"
        );
    }
}

#[cfg(test)]
mod const_prefix_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    /// Spec42 Gap 36: `const end [1] feature a;` attaches `const` to the end prefix and the
    /// `[1]` to the owned cross feature (KerML BNF 573/592/595), rather than to a second node
    /// wrapping a feature that offered `end` and `const` all over again.
    #[test]
    fn const_prefix_is_retained_on_end_members() {
        let src = input("const end [1] feature a;");
        let (rest, node) = kerml_feature(src).expect("const end member");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let crate::ast::FeaturePrefixHead::End { prefix, cross } = &node.value.prefix.head else {
            panic!("expected the end alternative of FeaturePrefix");
        };
        assert!(prefix.constant_span.is_some(), "const span");
        let cross = cross.as_ref().expect("owned cross feature");
        assert!(cross.value.name.is_none(), "the cross feature is unnamed");
        assert!(
            cross.value.multiplicity.is_some(),
            "[1] is the cross feature's"
        );
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(src, n)),
            Some(&b"a"[..])
        );
    }

    /// `const end feature b;` takes `FeaturePrefix`'s `EndFeaturePrefix` alternative (KerML BNF
    /// 573/584), so `const` lands in that alternative's own slot rather than on a second,
    /// independently settable flag beside `end`.
    #[test]
    fn const_prefix_is_retained_on_feature_members() {
        let src = input("const end feature b;");
        let (rest, node) = kerml_feature(src).expect("const end feature");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let end = node
            .value
            .prefix
            .head
            .end()
            .expect("the end alternative of FeaturePrefix");
        assert!(end.constant_span.is_some(), "const span");
        assert!(node.value.prefix.is_end());
        assert!(node.value.prefix.is_constant());
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(src, n)),
            Some(&b"b"[..])
        );
    }

    /// `var const feature b;` spells one slot twice: `BasicFeaturePrefix`'s last group is
    /// `( isVariable ?= 'var' | isConstant ?= 'const' )?` (KerML BNF 577), an alternation. Before
    /// the `FeaturePrefix` seam both keywords were accepted and set two independent booleans.
    #[test]
    fn variability_slot_admits_one_keyword() {
        let parsed = kerml_feature(input("var const feature b;"));
        let Ok((rest, _)) = parsed else {
            return;
        };
        assert!(
            !rest.fragment().is_empty(),
            "`var const` must not be consumed as one prefix"
        );
    }
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
        let src = input("constraint c : C;");
        let (rest, node) = constraint_usage(src).expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(src, n)),
            Some(&b"c"[..])
        );
        assert!(node.value.type_name.is_some());
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn constraint_usage_accepts_typed_braced_form() {
        let src = input("constraint mc : MassConstraint4 {\n    in totalMass : MassValue;\n}");
        let (rest, node) = constraint_usage(src).expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(src, n)),
            Some(&b"mc"[..])
        );
        assert!(node.value.type_name.is_some());
        assert!(matches!(node.value.body, ConstraintDefBody::Brace { .. }));
    }

    #[test]
    fn constraint_usage_accepts_untyped_braced_form() {
        let src = input("constraint hasLegalProfileDepth {profileDepth >= 3.5 [mm]}");
        let (_, node) = constraint_usage(src).expect("constraint usage");
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(src, n)),
            Some(&b"hasLegalProfileDepth"[..])
        );
        assert_eq!(node.value.type_name, None);
    }

    /// Regression: `Systems Library/Constraints.sysml`'s `constraintChecks` -- `abstract` +
    /// typing + trailing multiplicity + `nonunique` modifier + subsetting, all `def`-less. This
    /// exact shape is why `constraint_def` couldn't safely require `def` before this parser
    /// existed (CHANGELOG 0.33.0).
    #[test]
    fn constraint_usage_accepts_the_real_library_constraint_checks_form() {
        let src = input("abstract constraint constraintChecks: ConstraintCheck[0..*] nonunique :> booleanEvaluations {\n}");
        let (rest, node) = constraint_usage(src).expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(src, n)),
            Some(&b"constraintChecks"[..])
        );
        assert!(node.value.type_name.is_some());
    }

    /// Regression: `Systems Library/Constraints.sysml`'s `assertedConstraintChecks` -- `abstract`
    /// + multi-target subsetting only, no typing at all.
    #[test]
    fn constraint_usage_accepts_subsetting_only_multi_target_form() {
        let src = input("abstract constraint assertedConstraintChecks :> constraintChecks, trueEvaluations {\n}");
        let (rest, node) = constraint_usage(src).expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(src, n)),
            Some(&b"assertedConstraintChecks"[..])
        );
        assert_eq!(node.value.type_name, None);
    }

    /// Regression: `Systems Library/Requirements.sysml`'s `constraint assumptions[0..*] :>
    /// constraintChecks, subperformances` -- multiplicity directly after the name with no typing,
    /// then multi-target subsetting.
    #[test]
    fn constraint_usage_accepts_leading_multiplicity_then_subsetting() {
        let src = input("constraint assumptions[0..*] :> constraintChecks, subperformances {\n}");
        let (rest, node) = constraint_usage(src).expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(src, n)),
            Some(&b"assumptions"[..])
        );
        assert_eq!(node.value.type_name, None);
    }

    /// Regression: `Systems Library/Requirements.sysml`'s `constraint assumptions :>>
    /// RequirementConstraintCheck::assumptions;` -- redefinition with a qualified target,
    /// semicolon body.
    #[test]
    fn constraint_usage_accepts_redefinition_with_qualified_target() {
        let src = input("constraint assumptions :>> RequirementConstraintCheck::assumptions;");
        let (rest, node) = constraint_usage(src).expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(src, n)),
            Some(&b"assumptions"[..])
        );
        assert_eq!(node.value.type_name, None);
        assert!(matches!(
            node.value.body,
            ConstraintDefBody::Semicolon { .. }
        ));
    }

    #[test]
    fn constraint_usage_visibility_prefix_is_captured_on_membership() {
        let src = input("public constraint c1;");
        let (_, node) = constraint_usage(src).expect("constraint usage");
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
        let src = input("constraint c1;");
        let (_, node) = constraint_usage(src).expect("constraint usage");
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
