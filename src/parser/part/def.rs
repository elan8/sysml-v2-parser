use super::body::part_def_body;
use super::prelude::*;
use super::usage::part_usage;

/// Part definition: ( 'abstract' | 'variation' )? 'part' 'def' Identification ( (':>' | 'specializes') qualified_name )? body
pub(crate) fn part_def(input: Input<'_>) -> IResult<Input<'_>, Node<PartDef>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, definition_prefix) =
        crate::parser::definition_prefix::parse_basic_definition_prefix(
            input,
            crate::parser::definition_prefix::BasicPrefixSlot::Basic,
        )?;
    let (input, is_individual) = opt(preceded(tag(&b"individual"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (input, _) = tag(&b"part"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"def"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, identification) = identification(input)?;
    let (input, specializes) = parse_optional_definition_specialization(input)?;
    let (input, body) = part_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PartDef {
                definition_prefix,
                is_individual,
                identification,
                specializes,
                body,
                membership: Membership::owning(visibility, visibility_span),
            },
        ),
    ))
}

/// `PartDefinition` or `PartUsage`, whichever the member really is.
///
/// Package scope reaches both through one entry point so `part_def` cannot consume the `part` of
/// `part name …` before the usage parser is tried. `part_def` requires the `def` keyword, so
/// trying it first is safe and each parser reads its own `MemberPrefix` and its own prefix
/// production from the start of the member: `OccurrenceDefinitionPrefix` for the definition
/// (`BasicDefinitionPrefix? ( 'individual' … )? DefinitionExtensionKeyword*`, SysML BNF 541) and
/// `OccurrenceUsagePrefix` for the usage (line 564).
///
/// The two are different productions, which is exactly what this function used to get wrong: it
/// parsed one prefix -- `abstract`/`variation`, `ref`, `individual` -- and shared it between
/// both branches, so at package scope a part usage accepted no direction, no `derived`, no
/// `constant`, no `PortionKind` and no `UsageExtensionKeyword`, while `ref part def P;` skipped
/// the definition branch and was read as a usage *named* `def`.
pub(crate) fn part_def_or_usage(input: Input<'_>) -> IResult<Input<'_>, PartDefOrUsage> {
    if let Ok((rest, def)) = part_def(input) {
        return Ok((rest, PartDefOrUsage::Def(def)));
    }
    let (rest, usage) = part_usage(input)?;
    Ok((rest, PartDefOrUsage::Usage(usage)))
}
