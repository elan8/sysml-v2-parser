//! Lexer and skip helpers: whitespace, comments, names, qualified names, and body-skip utilities.

use crate::ast::{
    Identification, QualifiedDeclarationName, QualifiedReferenceId, ReferenceSegment,
    ReferenceSeparator, Span,
};
use crate::parser::{span_from_to, Input};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while, take_while1};
use nom::combinator::{map, opt, rest, value};
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;
use nom::Parser;

pub(crate) const PART_BODY_STARTERS: &[&[u8]] = &[
    b"#",
    b"@",
    b"abstract",
    b"allocate",
    // `assert` and `not` both begin a `SatisfyRequirementUsage`
    // (`( 'assert' )? ( 'not' )? 'satisfy' ...`), and `assert` also begins an
    // `AssertConstraintUsage`. Recovery must synchronize on the *first* token of the member, not
    // on `satisfy`, or a malformed member before `not satisfy r by p;` scans past the prefix and
    // takes that member's terminator with it.
    b"assert",
    b"not",
    b"action",
    b"attribute",
    b"allocate",
    b"calc",
    b"bind",
    b"calc",
    b"comment",
    b"connection",
    b"connect",
    b"constraint",
    b"dependency",
    b"doc",
    b"enum",
    b"event",
    b"exhibit",
    b"first",
    b"flow",
    b"import",
    b"individual",
    // The rest of FIRST(`OccurrenceUsagePrefix`) -- see
    // `planning/occurrence-usage-prefix-matrix.md` §4. `abstract`, `individual`, `ref`,
    // `snapshot`, `timeslice` and `variation` were already listed; these five were not, so a
    // malformed member before `in individual :>> v : V;` or `derived occurrence o;` scanned past
    // the prefix and consumed the whole usage.
    b"constant",
    b"derived",
    b"in",
    b"inout",
    b"out",
    b"message",
    b"interface",
    b"item",
    b"library",
    b"occurrence",
    b"part",
    b"package",
    b"perform",
    b"port",
    b"private",
    b"protected",
    b"public",
    b"ref",
    b"requirement",
    b"satisfy",
    b"state",
    b"standard",
    b"succession",
    b"snapshot",
    b"timeslice",
    b"variant",
    b"variation",
    // `DefinitionBodyItem = ( SourceSuccessionMember )? OccurrenceUsageMember`, so `then` is the
    // first token of a member here, not part of the one before it.
    b"then",
    b"analysis",
    b"metadata",
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
    b"class",
];

pub(crate) const PORT_DEF_BODY_STARTERS: &[&[u8]] = &[
    b":>>",
    b":>",
    b"doc",
    b"attribute",
    b"port",
    b"in",
    b"out",
    b"inout",
    b"ref",
    b"abstract",
    // The rest of FIRST(`OccurrenceUsagePrefix`) on the port usage this scope dispatches;
    // `abstract`, `in`, `inout`, `out`, `port` and `ref` were already listed. See
    // `planning/port-usage-prefix-matrix.md` §6.
    b"#",
    b"constant",
    b"derived",
    b"individual",
    b"snapshot",
    b"timeslice",
    b"variation",
    b"variant",
    // The remaining members `port_def_body_element` dispatches. A starter table is only worth
    // having if it names where a member *starts*; recovery synchronizes on it, so a missing entry
    // is a valid sibling consumed by the malformed node before it.
    b"item",
    b"enum",
    b"comment",
    b"rep",
    b"@",
    b"private",
    b"protected",
    b"public",
];

/// `PortBody = DefinitionBody`, and this scope dispatches a `PortUsage`, so every token of
/// FIRST(`PortUsage`) is a member starter here. Only four of the thirteen were listed, so a
/// malformed member before `ref port q;` scanned past the prefix and consumed the usage. See
/// `planning/port-usage-prefix-matrix.md` §6.
pub(crate) const PORT_BODY_STARTERS: &[&[u8]] = &[
    b":>>",
    b":>",
    b"doc",
    b"event",
    b"port",
    b"in",
    b"out",
    b"inout",
    b"#",
    b"abstract",
    b"constant",
    b"derived",
    b"individual",
    b"ref",
    b"snapshot",
    b"timeslice",
    b"variation",
    // `DefinitionBodyItem` admits a `VariantUsageMember` here, which `variation port :>> autoPort
    // { variant port autoPort1; }` writes (`Variability Examples/VehicleVariabilityModel.sysml:79`).
    // This scope now owns that typed member, and recovery synchronizes at its grammar starter.
    b"variant",
    b"attribute",
    b"item",
    b"comment",
    b"rep",
    b"@",
    b"private",
    b"protected",
    b"public",
];

pub(crate) const REQUIREMENT_BODY_STARTERS: &[&[u8]] = &[
    b"#",
    b"@",
    // See `PART_BODY_STARTERS`: the two optional prefixes of `SatisfyRequirementUsage` are FIRST
    // tokens of this scope exactly as `satisfy` itself is.
    b"assert",
    b"not",
    b"attribute",
    b"allocate",
    b"calc",
    b"constraint",
    b"doc",
    b"frame",
    b"import",
    b"rep",
    b"require",
    b"requirement",
    b"satisfy",
    // FIRST(`OccurrenceUsagePrefix`), which a `SatisfyRequirementUsage` in this scope now spells
    // ahead of `assert`/`not`/`satisfy` -- see `planning/occurrence-usage-prefix-matrix.md` §4.
    // `#` and `ref` were already listed.
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
    b"subject",
    b"actor",
    b"stakeholder",
    b"purpose",
    b"port",
    b"ref",
    b"verify",
    b"variant",
    // `RequirementBodyItem -> DefinitionBodyItem` (SysML BNF 1407, 237) admits the general usage
    // families as well as the requirement-specific members, so these are FIRST tokens of this
    // scope. Without them a legal `action a;` was classified `unexpected_keyword_in_scope`.
    b"action",
    b"succession",
    b"perform",
    b"state",
    b"item",
    b"part",
    b"connect",
    b"connection",
    b":>>",
    b":>",
];

#[allow(dead_code)]
pub(crate) const STATE_BODY_STARTERS: &[&[u8]] = &[
    b"#",
    b"@",
    b"accept",
    // `StateBodyItem -> NonBehaviorBodyItem -> StructureUsageMember -> PartUsage`, and the
    // sibling `BehaviorUsageMember -> ConstraintUsage`, each begin with the complete
    // OccurrenceUsagePrefix. These entries are recovery FIRST-set membership, not a permissive
    // parser: the owning part/constraint parsers still validate the complete production.
    b"abstract",
    b"constant",
    b"constraint",
    b"derived",
    b"doc",
    b"do",
    b"entry",
    b"exit",
    b"final",
    b"first",
    b"if",
    b"in",
    b"inout",
    b"individual",
    b"out",
    b"part",
    b"ref",
    b"snapshot",
    b"state",
    b"then",
    b"timeslice",
    b"transition",
    b"variation",
];

pub(crate) const USE_CASE_BODY_STARTERS: &[&[u8]] = &[
    b"@",
    b"abstract",
    b"action",
    b"actor",
    b"analysis",
    b"assign",
    b"attribute",
    b"bind",
    b"calc",
    b"case",
    b"doc",
    b"first",
    b"flow",
    b"for",
    b"include",
    b"in",
    b"objective",
    b"out",
    b"part",
    b"perform",
    b"private",
    b"protected",
    b"ref",
    b"requirement",
    b"return",
    b"public",
    b"state",
    b"subject",
    b"then",
    // The rest of FIRST(`OccurrenceUsagePrefix`) on the part usage this scope dispatches;
    // `abstract`, `in`, `out`, `part` and `ref` were already listed. See
    // `planning/part-usage-prefix-matrix.md` §6.
    b"#",
    b"constant",
    b"derived",
    b"individual",
    b"inout",
    b"snapshot",
    b"timeslice",
    b"variation",
];

pub(crate) const CALC_DEF_BODY_STARTERS: &[&[u8]] = &[
    b"@",
    b"doc",
    b"in",
    b"out",
    b"inout",
    b"return",
    // SysML `CalculationBodyItem -> ActionBodyItem -> NonBehaviorBodyItem` owns
    // `ReferenceUsage` (SysML BNF 1366/1367, 901/902, 335). Keep recovery from consuming a
    // valid generic `ref` declaration after malformed calculation-body content.
    b"ref",
    b"calc",
    b"part",
    // KerML `TypeBodyElement` owns `AliasMember` directly (textual BNF 431-438), while a
    // SysML `CalculationBody` reaches it through `ActionBodyItem -> NonBehaviorBodyItem`
    // (SysML textual BNF 1359-1368, 901-917). Keep recovery from swallowing a valid alias
    // after malformed content in either owner.
    b"alias",
    // `TypeBodyElement -> FeatureMember -> OwnedFeatureMember -> FeatureElement -> Flow` (KerML
    // BNF 434, 519, 526, 360/369, 1303): a KerML type body owns a `flow` member, so recovery
    // resynchronizes on the keyword rather than swallowing the flow after a malformed sibling.
    b"flow",
    // `REDEFINES` heads a nameless `Feature` whose `FeatureDeclaration` is a bare
    // `FeatureSpecializationPart` (KerML BNF 562, 601, 632, 663, 666). Only the word spelling is
    // listed here; the `:>>` symbol spelling's absence predates this list's flow/redefines entries.
    b"redefines",
];

/// The action-node keywords a `CalculationBody` owns through `CalculationBodyItem =
/// ActionBodyItem | ReturnParameterMember`, used to route a member to the action dispatcher
/// before the keyword-less-binding fallback can read the keyword itself as a feature name.
pub(crate) const CALCULATION_ACTION_STARTERS: &[&[u8]] = &[
    b"first",
    b"merge",
    b"decide",
    b"join",
    b"fork",
    b"action",
    b"if",
    b"while",
    b"loop",
    b"for",
    b"assign",
    b"terminate",
    b"then",
    b"perform",
    b"send",
    b"accept",
    b"exhibit",
    b"state",
    b"item",
    b"flow",
    // `CalculationBodyItem -> ActionBodyItem -> NonBehaviorBodyItem -> StructureUsageMember ->
    // StructureUsageElement -> Message` (SysML BNF 1366/1367, 901/902, 910/916-917, 262,
    // 355/362/371, 805). `flow` was routed here and `message` was not, even though one parser --
    // `flow::flow_usage_member`, keyed by `FlowUsageKind` -- owns both spellings: `message m of
    // T;` in a calculation body fell through to `calc_def_body_element`'s bare-expression
    // fallback and was shredded into `'message';`, `m;`, `'of';`, `T;` with no diagnostic
    // (spec42 Gap 61). `Message` is a SysML-only production; KerML `FeatureElement` does not
    // reach it, so `calc_def_body_element` deliberately has no `message` arm of its own.
    b"message",
    // Route generic `ReferenceUsage` through the action-body parser before calculation's
    // keyword-less binding fallback can interpret `ref` as a feature name.
    b"ref",
];

pub(crate) const CONSTRAINT_DEF_BODY_STARTERS: &[&[u8]] = &[
    b"doc",
    b"@",
    b"in",
    b"out",
    b"inout",
    b"constraint",
    // `CalculationBodyItem -> ActionBodyItem -> NonBehaviorBodyItem` admits AliasMember.
    // Keep recovery from swallowing a valid alias after malformed constraint-body content.
    b"alias",
    // `CalculationBodyItem = ActionBodyItem | ReturnParameterMember` (SysML BNF 1366, 1370), and
    // a constraint body is a `CalculationBody`, so `return` starts a member here too.
    b"return",
    b":>>",
    b":>",
    // `CalculationBody` reaches `StructureUsageMember -> PartUsage`, so `part` and the rest of
    // FIRST(`OccurrenceUsagePrefix`) are member starters here too. See
    // `planning/part-usage-prefix-matrix.md` §6.
    b"part",
    b"#",
    b"abstract",
    b"constant",
    b"derived",
    b"individual",
    b"ref",
    b"snapshot",
    b"timeslice",
    b"variation",
];

/// Starters for `RelationshipBody`-shaped brace bodies (alias/import/dependency and other
/// annotation-only leaf bodies): BNF `RelationshipBody : Relationship = ';' | '{'
/// (ownedRelationship += OwnedAnnotation)* '}'`.
pub(crate) const RELATIONSHIP_BODY_STARTERS: &[&[u8]] =
    &[b"doc", b"comment", b"rep", b"@", b"feature"];

/// `satisfy`, `assert` and `not` are here because `view_def_body_element` dispatches
/// `SatisfyRequirementUsage`; the list had none of the three, so a malformed member before any
/// satisfy usage in a view definition body consumed it.
pub(crate) const VIEW_DEF_BODY_STARTERS: &[&[u8]] = &[
    b"@",
    b"alias",
    b"assert",
    b"doc",
    b"filter",
    b"not",
    b"render",
    b"rendering",
    b"ref",
    b"satisfy",
    b"abstract",
    // FIRST(`OccurrenceUsagePrefix`) on the satisfy usage this scope dispatches; `abstract`,
    // `ref` and the three satisfy keywords were already listed. See
    // `planning/occurrence-usage-prefix-matrix.md` §4.
    b"#",
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

pub(crate) const VIEW_BODY_STARTERS: &[&[u8]] = &[
    b"alias",
    b"assert",
    b"doc",
    b"expose",
    b"filter",
    b"not",
    b"render",
    b"rendering",
    b"satisfy",
    // FIRST(`OccurrenceUsagePrefix`) on the satisfy usage this scope dispatches. See
    // `planning/occurrence-usage-prefix-matrix.md` §4.
    b"#",
    b"abstract",
    b"constant",
    b"derived",
    b"in",
    b"individual",
    b"inout",
    b"out",
    b"ref",
    b"snapshot",
    b"timeslice",
    b"variation",
];

/// `connect`, `end`, `ref`, `doc` -- plus `part` and the rest of FIRST(`OccurrenceUsagePrefix`),
/// because this scope dispatches `OccurrenceUsage`, `ItemUsage` and `PartUsage`, each of which
/// may be written with the whole shared prefix. See `planning/part-usage-prefix-matrix.md` §6.
pub(crate) const CONNECTION_DEF_BODY_STARTERS: &[&[u8]] = &[
    b"connect",
    b"end",
    b"ref",
    b"doc",
    b"port",
    b"#",
    b"abstract",
    b"constant",
    b"derived",
    b"in",
    b"individual",
    b"inout",
    b"item",
    b"occurrence",
    b"out",
    b"part",
    b"snapshot",
    b"timeslice",
    b"variation",
    // The remaining members `connection_def_body_element` dispatches; see
    // `PORT_DEF_BODY_STARTERS` for why an incomplete table costs a valid sibling.
    b"attribute",
    b"assert",
    b"succession",
    b"comment",
    b"rep",
    b"@",
    b"private",
    b"protected",
    b"public",
];

/// GH-51: mirrors [`CONNECTION_DEF_BODY_STARTERS`] -- `interface_def_body` previously had no
/// starter list at all (its own hand-rolled brace loop swallowed unparseable content silently,
/// with no diagnostic).
///
/// The list named four of the dozen members `interface_def_body_element` dispatches, so a
/// malformed member scanned past every attribute, item and port declaration after it. `port` and
/// the rest of FIRST(`PortUsage`) are added here with the seam that makes a port usage in this
/// scope carry the whole shared prefix; see `planning/port-usage-prefix-matrix.md` §6 and §10.1.
pub(crate) const INTERFACE_DEF_BODY_STARTERS: &[&[u8]] = &[
    b"connect",
    // `InterfaceOccurrenceUsageElement` includes `BehaviorUsageElement`, whose ConstraintUsage
    // alternative owns the full occurrence-prefix and calculation-body grammar.
    b"constraint",
    b"end",
    b"ref",
    b"doc",
    b"attribute",
    b"item",
    b"port",
    b"flow",
    b"#",
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
    b"comment",
    b"rep",
    b"@",
    b"private",
    b"protected",
    b"public",
];

/// Starters for the currently typed members of `InterfaceUsage`'s `InterfaceBody`.
///
/// This deliberately differs from [`INTERFACE_DEF_BODY_STARTERS`]: the two grammar productions
/// share `InterfaceBody`, but their AST owners support different member sets. In particular,
/// `perform` is a `BehaviorUsageElement` here, while `message` and `succession flow` travel
/// through the existing `FlowUsage` owner. Every accepted prefix is listed so recovery resumes
/// before a valid later member instead of absorbing it into the preceding malformed span.
pub(crate) const INTERFACE_USAGE_BODY_STARTERS: &[&[u8]] = &[
    b"ref",
    b"end",
    b"flow",
    b"message",
    b"succession",
    b"perform",
    b"doc",
    b"comment",
    b"rep",
    b"@",
    b"abstract",
    b"variation",
    b"private",
    b"protected",
    b"public",
];

/// Skip optional whitespace (space, tab, newline).
pub(crate) fn ws(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) =
        take_while(|c: u8| c == b' ' || c == b'\t' || c == b'\n' || c == b'\r').parse(input)?;
    Ok((input, ()))
}

/// Skip whitespace and comments (block, single-line). Use between tokens and at body boundaries.
/// Does NOT consume "doc /* ... */" — that is a body element (PackageBodyElement::Doc etc.) and must
/// be parsed explicitly so it appears in the AST. //* ... */ is tried before line_comment so that
/// "//*" starts a block comment, not a line comment.
pub(crate) fn ws_and_comments(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let skipped = trivia_len(input.fragment());
    Ok((crate::parser::span::advance(input, skipped), ()))
}

/// Skip whitespace and *notes* only, leaving a `/* ... */` block comment for the caller.
///
/// The pinned grammar defines three lexical forms and only two of them are notes:
///
/// ```text
/// SINGLE_LINE_NOTE = '//' LINE_TEXT
/// MULTILINE_NOTE   = '//*' COMMENT_TEXT '*/'
/// REGULAR_COMMENT  = '/*'  COMMENT_TEXT '*/'
/// ```
///
/// (KerML BNF 32-39.) A `REGULAR_COMMENT` is the body of a `Comment`, and every group preceding
/// that body in `Comment = ( 'comment' Identification ( 'about' ... )? )? ( 'locale' ... )? body`
/// (KerML BNF 199) is optional -- so a bare `/* ... */` at a member position is an
/// `AnnotatingElement`, i.e. syntax, not trivia.
///
/// Used exactly where a member may begin: the structured brace-member loop and
/// [`crate::parser::body::annotating_member`]. Everywhere else keeps [`ws_and_comments`], because
/// the grammar has no member between the tokens of a declaration for a comment there to be.
pub(crate) fn ws_and_notes(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let skipped = note_trivia_len(input.fragment());
    Ok((crate::parser::span::advance(input, skipped), ()))
}

/// Byte length of the whitespace-and-notes run at the start of `bytes`.
///
/// Mirrors [`trivia_len`] minus its `/* ... */` arm. An unterminated `//*` is an ordinary line
/// note, exactly as there.
fn note_trivia_len(bytes: &[u8]) -> usize {
    let mut pos = 0usize;
    loop {
        while let Some(&byte) = bytes.get(pos) {
            if byte == b' ' || byte == b'\t' || byte == b'\n' || byte == b'\r' {
                pos += 1;
            } else {
                break;
            }
        }

        let rest = &bytes[pos..];
        if rest.starts_with(b"//") {
            if rest.starts_with(b"//*") {
                if let Some(end) = block_comment_end(rest, 3) {
                    pos += end;
                    continue;
                }
            }
            pos += line_comment_len(rest);
        } else {
            return pos;
        }
    }
}

/// Byte length of the trivia run at the start of `bytes`.
///
/// This is the parser's hottest lexical routine: it runs before every token and again for every
/// alternative that backtracks over the same position, so it is written as one explicit scan
/// rather than composed combinators. The recognized forms and their precedence match the grammar
/// documented on [`ws_and_comments`]: whitespace, a terminated `/* ... */`, a terminated
/// `//* ... */`, and `//` to end of line. An unterminated `/*` is not trivia and stops the scan,
/// leaving it for the caller to report; an unterminated `//*` is an ordinary line comment.
fn trivia_len(bytes: &[u8]) -> usize {
    let mut pos = 0usize;
    loop {
        while let Some(&byte) = bytes.get(pos) {
            if byte == b' ' || byte == b'\t' || byte == b'\n' || byte == b'\r' {
                pos += 1;
            } else {
                break;
            }
        }

        let rest = &bytes[pos..];
        if rest.starts_with(b"/*") {
            match block_comment_end(rest, 2) {
                Some(end) => pos += end,
                None => return pos,
            }
        } else if rest.starts_with(b"//") {
            if rest.starts_with(b"//*") {
                if let Some(end) = block_comment_end(rest, 3) {
                    pos += end;
                    continue;
                }
            }
            pos += line_comment_len(rest);
        } else {
            return pos;
        }
    }
}

/// Length of a block comment whose body starts at `body_start`, or `None` when it is unterminated.
fn block_comment_end(bytes: &[u8], body_start: usize) -> Option<usize> {
    let offset = bytes[body_start..]
        .windows(2)
        .position(|pair| pair == b"*/")?;
    Some(body_start + offset + 2)
}

/// Length of a `//` line comment, including the newline run that ends it.
fn line_comment_len(bytes: &[u8]) -> usize {
    let mut pos = match bytes[2..].iter().position(|&b| b == b'\n' || b == b'\r') {
        Some(offset) => 2 + offset,
        None => return bytes.len(),
    };
    while matches!(bytes.get(pos), Some(b'\n') | Some(b'\r')) {
        pos += 1;
    }
    pos
}

/// Parse one or more whitespace characters (consumes at least one).
pub(crate) fn ws1(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let skipped = input
        .fragment()
        .iter()
        .take_while(|c| matches!(c, b' ' | b'\t' | b'\n' | b'\r'))
        .count();
    if skipped == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeWhile1,
        )));
    }
    Ok((crate::parser::span::advance(input, skipped), ()))
}

/// Skip to the next sync point (next line start after newline and ws/comments), or to end of input.
/// Used for error recovery so parsing can continue after a failed top-level element.
pub(crate) fn skip_to_next_sync_point(input: Input<'_>) -> IResult<Input<'_>, ()> {
    alt((
        map(
            (
                take_until(&b"\n"[..]),
                opt(tag(&b"\n"[..])),
                ws_and_comments,
            ),
            |_| (),
        ),
        value((), rest),
    ))
    .parse(input)
}

/// Skip one malformed root fragment without stopping inside a brace-delimited block.
///
/// A newline or semicolon at depth zero is a safe boundary. Braces, quoted strings, and comments
/// are tracked so resilient parsing retains one exact recovery span and can continue with later
/// top-level siblings.
pub(crate) fn skip_to_next_balanced_sync_point(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let fragment = input.fragment();
    let mut pos = 0usize;
    let mut brace_depth = 0usize;
    let mut block_comment_depth = 0usize;
    let mut line_comment = false;
    let mut quote = None;
    let mut escaped = false;

    while pos < fragment.len() {
        let byte = fragment[pos];
        let next = fragment.get(pos + 1).copied();
        if line_comment {
            pos += 1;
            if byte == b'\n' {
                line_comment = false;
                if brace_depth == 0 {
                    break;
                }
            }
            continue;
        }
        if block_comment_depth > 0 {
            if byte == b'/' && next == Some(b'*') {
                block_comment_depth += 1;
                pos += 2;
            } else if byte == b'*' && next == Some(b'/') {
                block_comment_depth -= 1;
                pos += 2;
            } else {
                pos += 1;
            }
            continue;
        }
        if let Some(delimiter) = quote {
            pos += 1;
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == delimiter {
                quote = None;
            }
            continue;
        }

        match (byte, next) {
            (b'/', Some(b'/')) => {
                line_comment = true;
                pos += 2;
            }
            (b'/', Some(b'*')) => {
                block_comment_depth = 1;
                pos += 2;
            }
            (b'\'' | b'"', _) => {
                quote = Some(byte);
                pos += 1;
            }
            (b'{', _) => {
                brace_depth += 1;
                pos += 1;
            }
            (b'}', _) if brace_depth == 0 => break,
            (b'}', _) => {
                brace_depth -= 1;
                pos += 1;
                if brace_depth == 0 {
                    break;
                }
            }
            (b';', _) if brace_depth == 0 => {
                pos += 1;
                break;
            }
            (b'\n' | b'\r', _) if brace_depth == 0 => {
                pos += 1;
                if byte == b'\r' && next == Some(b'\n') {
                    pos += 1;
                }
                break;
            }
            _ => pos += 1,
        }
    }

    let advance = pos.max(1).min(fragment.len());
    let (input, _) = nom::bytes::complete::take(advance).parse(input)?;
    Ok((input, ()))
}

/// Skip to the next root-level package or namespace (next line starting with "package " or "namespace "
/// after ws/comments), or to end of input. Used when recovery from a failure inside a package body.
/// Skip to the next root-level package or namespace, or to end of input.
/// Used when recovering from a failure inside a package body (avoids reporting errors on every line).
#[allow(dead_code)]
pub(crate) fn skip_to_next_root_element(mut input: Input<'_>) -> IResult<Input<'_>, ()> {
    loop {
        if input.fragment().is_empty() {
            return Ok((input, ()));
        }
        let (after_ws, _) = ws_and_comments(input).unwrap_or((input, ()));
        let frag = after_ws.fragment();
        if frag.len() >= 8 && (frag.starts_with(b"package ") || frag.starts_with(b"namespace ")) {
            return Ok((after_ws, ()));
        }
        match skip_to_next_sync_point(input) {
            Ok((rest, _)) => input = rest,
            Err(_) => return Ok((input, ())),
        }
    }
}

/// Reserved keywords of the SysML v2 textual notation (OMG Systems Modeling Language v2.0, Part 1,
/// clause 8.2.2.1.2 "Lexical Structure"). Used to tell a genuinely misused language keyword apart
/// from an arbitrary (non-keyword) identifier when a body-recovery diagnostic reports an
/// "unexpected token" -- an unrecognized identifier like `test` or `distancePerVolume` is an input
/// defect, not a grammar gap, and should not be reported as `unexpected keyword` (GH-18).
const SYSML_RESERVED_KEYWORDS: &[&str] = &[
    "about",
    "abstract",
    "accept",
    "action",
    "actor",
    "after",
    "alias",
    "all",
    "allocate",
    "allocation",
    "analysis",
    "and",
    "as",
    "assert",
    "assign",
    "assume",
    "at",
    "attribute",
    "bind",
    "binding",
    "by",
    "calc",
    "case",
    "comment",
    "concern",
    "connect",
    "connection",
    "constant",
    "constraint",
    "crosses",
    "decide",
    "def",
    "default",
    "defined",
    "dependency",
    "derived",
    "do",
    "doc",
    "else",
    "end",
    "entry",
    "enum",
    "event",
    "exhibit",
    "exit",
    "expose",
    "false",
    "filter",
    "first",
    "flow",
    "for",
    "fork",
    "frame",
    "from",
    "hastype",
    "if",
    "implies",
    "import",
    "in",
    "include",
    "individual",
    "inout",
    "interface",
    "istype",
    "item",
    "join",
    "language",
    "library",
    "locale",
    "loop",
    "merge",
    "message",
    "meta",
    "metadata",
    "nonunique",
    "not",
    "null",
    "objective",
    "occurrence",
    "of",
    "or",
    "ordered",
    "out",
    "package",
    "parallel",
    "part",
    "perform",
    "port",
    "private",
    "protected",
    "public",
    "redefines",
    "ref",
    "references",
    "render",
    "rendering",
    "rep",
    "require",
    "requirement",
    "return",
    "satisfy",
    "send",
    "snapshot",
    "specializes",
    "stakeholder",
    "standard",
    "state",
    "subject",
    "subsets",
    "succession",
    "terminate",
    "then",
    "timeslice",
    "to",
    "transition",
    "true",
    "until",
    "use",
    "variant",
    "variation",
    "verification",
    "verify",
    "via",
    "view",
    "viewpoint",
    "when",
    "while",
    "xor",
];

/// Whether `word` (a bare identifier-like byte slice, no trailing punctuation) is one of the
/// reserved keywords of the SysML v2 textual notation.
pub(crate) fn is_reserved_keyword(word: &[u8]) -> bool {
    std::str::from_utf8(word).is_ok_and(|s| SYSML_RESERVED_KEYWORDS.contains(&s))
}

pub(crate) fn starts_with_keyword(fragment: &[u8], keyword: &[u8]) -> bool {
    // Candidates are tested against the same position in long starter lists (52 entries for a
    // package body), so the cheap discriminating test comes first: nearly every candidate fails on
    // its first byte, and only a match pays for classifying the keyword's shape.
    if !fragment.starts_with(keyword) {
        return false;
    }
    // A punctuation starter such as `:>>` is not identifier-shaped and needs no token boundary
    // after it; an identifier-shaped keyword must not be the prefix of a longer name.
    if keyword
        .iter()
        .any(|b| !b.is_ascii_alphanumeric() && *b != b'_')
    {
        return true;
    }
    fragment
        .get(keyword.len())
        .is_none_or(|b| b.is_ascii_whitespace() || matches!(*b, b'{' | b':' | b';' | b'['))
}

pub(crate) fn starts_with_any_keyword(fragment: &[u8], keywords: &[&[u8]]) -> bool {
    keywords
        .iter()
        .any(|keyword| starts_with_keyword(fragment, keyword))
}

/// Whether `keyword` occurs anywhere in `fragment` at a word boundary (not as a substring of a
/// longer identifier, e.g. `connect` does not match inside `Connection` or `reconnect`). Used to
/// detect a keyword swallowed into text a caller is about to discard, as opposed to
/// [`starts_with_keyword`]'s anchored-at-the-front check.
pub(crate) fn contains_keyword(fragment: &[u8], keyword: &[u8]) -> bool {
    fn is_ident_byte(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_'
    }
    if keyword.is_empty() || fragment.len() < keyword.len() {
        return false;
    }
    (0..=fragment.len() - keyword.len()).any(|i| {
        &fragment[i..i + keyword.len()] == keyword
            && (i == 0 || !is_ident_byte(fragment[i - 1]))
            && fragment
                .get(i + keyword.len())
                .is_none_or(|b| !is_ident_byte(*b))
    })
}

fn local_recovery_line_boundary<'a>(input: Input<'a>, starters: &[&[u8]]) -> Option<Input<'a>> {
    let (input, _) = ws_and_comments(input).ok()?;
    let fragment = input.fragment();
    if fragment.is_empty() {
        return Some(input);
    }

    let mut pos = 0usize;
    let mut brace_depth = 0usize;
    let mut block_comment_depth = 0usize;
    let mut line_comment = false;
    let mut quote = None;
    let mut escaped = false;
    while pos < fragment.len() {
        let byte = fragment[pos];
        let next = fragment.get(pos + 1).copied();
        if line_comment {
            if matches!(byte, b'\n' | b'\r') {
                line_comment = false;
            } else {
                pos += 1;
                continue;
            }
        }
        if block_comment_depth > 0 {
            if byte == b'/' && next == Some(b'*') {
                block_comment_depth += 1;
                pos += 2;
            } else if byte == b'*' && next == Some(b'/') {
                block_comment_depth -= 1;
                pos += 2;
            } else {
                pos += 1;
            }
            continue;
        }
        if let Some(delimiter) = quote {
            pos += 1;
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == delimiter {
                quote = None;
            }
            continue;
        }

        match (byte, next) {
            (b'/', Some(b'/')) => {
                line_comment = true;
                pos += 2;
                continue;
            }
            (b'/', Some(b'*')) => {
                block_comment_depth = 1;
                pos += 2;
                continue;
            }
            (b'\'' | b'"', _) => {
                quote = Some(byte);
                pos += 1;
                continue;
            }
            _ => {}
        }

        if pos < fragment.len()
            && (fragment[pos] == b'\n' || fragment[pos] == b'\r')
            && brace_depth == 0
        {
            let newline_start = pos;
            while pos < fragment.len() && (fragment[pos] == b'\n' || fragment[pos] == b'\r') {
                pos += 1;
            }
            let (candidate, _) =
                nom::bytes::complete::take::<_, _, nom::error::Error<Input<'a>>>(pos)
                    .parse(input)
                    .ok()?;
            let (candidate, _) = ws_and_comments(candidate).unwrap_or((candidate, ()));
            if (candidate.fragment().is_empty()
                || candidate.fragment().starts_with(b"}")
                || starts_with_any_keyword(candidate.fragment(), starters))
                && newline_start > 0
            {
                return Some(candidate);
            }
            continue;
        }

        match byte {
            b'{' => brace_depth += 1,
            b'}' if brace_depth == 0 => break,
            b'}' => brace_depth -= 1,
            _ => {}
        }
        pos += 1;
    }

    None
}

/// Skip to the next likely body element starter for the current grammar scope, or to the closing `}` / EOF.
pub(crate) fn skip_to_next_body_element_or_end<'a>(
    mut input: Input<'a>,
    starters: &[&[u8]],
) -> IResult<Input<'a>, ()> {
    loop {
        let (after_ws, _) = ws_and_comments(input).unwrap_or((input, ()));
        input = after_ws;
        if input.fragment().is_empty()
            || input.fragment().starts_with(b"}")
            || starts_with_any_keyword(input.fragment(), starters)
        {
            return Ok((input, ()));
        }
        match skip_to_next_sync_point(input) {
            Ok((rest, _)) if rest.location_offset() != input.location_offset() => input = rest,
            _ => return Ok((input, ())),
        }
    }
}

/// Recover from a failed body element parse by first skipping the current statement or block,
/// then syncing to the next likely body element starter or `}`.
pub(crate) fn recover_body_element<'a>(
    input: Input<'a>,
    starters: &[&[u8]],
) -> IResult<Input<'a>, ()> {
    if let Some(next) = local_recovery_line_boundary(input, starters) {
        if next.location_offset() != input.location_offset() {
            return Ok((next, ()));
        }
    }
    let (input, _) = skip_statement_or_block(input)?;
    skip_to_next_body_element_or_end(input, starters)
}

/// NAME: BASIC_NAME (identifier) or UNRESTRICTED_NAME (single-quoted string).
pub(crate) fn name(input: Input<'_>) -> IResult<Input<'_>, String> {
    alt((quoted_name, basic_name)).parse(input)
}

/// Unquoted identifier: letter or underscore, then alphanumeric or underscore.
fn basic_name(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, raw) = take_while1(|c: u8| c.is_ascii_alphanumeric() || c == b'_').parse(input)?;
    let s = String::from_utf8_lossy(raw.fragment()).into_owned();
    Ok((input, s))
}

/// Quoted name: '...' (content between single quotes; \' for escape).
fn quoted_name(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = tag(&b"'"[..]).parse(input)?;
    let frag = input.fragment();
    let mut count = 0usize;
    let mut bytes = Vec::new();
    while count < frag.len() {
        if frag[count] == b'\\' && count + 1 < frag.len() && frag[count + 1] == b'\'' {
            bytes.push(b'\'');
            count += 2;
        } else if frag[count] == b'\'' {
            count += 1;
            break;
        } else {
            bytes.push(frag[count]);
            count += 1;
        }
    }
    let s = String::from_utf8_lossy(&bytes).into_owned();
    let (input, _) = nom::bytes::complete::take(count).parse(input)?;
    Ok((input, s))
}

/// Parse one authored name token without decoding or allocating its spelling.
///
/// The returned span includes quotes for unrestricted names. Unlike the legacy `quoted_name`
/// decoder, this source-backed path requires a closing quote and therefore cannot manufacture a
/// successful reference from an unterminated token.
fn reference_name_span(input: Input<'_>) -> IResult<Input<'_>, Span> {
    let start = input;
    let fragment = input.fragment();
    let Some(first) = fragment.first().copied() else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )));
    };

    let consumed = if first == b'\'' {
        let mut index = 1usize;
        let mut closing_quote = None;
        while index < fragment.len() {
            if fragment[index] == b'\\'
                && fragment.get(index + 1).is_some_and(|next| *next == b'\'')
            {
                index += 2;
            } else if fragment[index] == b'\'' {
                closing_quote = index.checked_add(1);
                break;
            } else {
                index += 1;
            }
        }
        closing_quote.ok_or_else(|| {
            nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Char))
        })?
    } else {
        if !(first.is_ascii_alphabetic() || first == b'_') {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Alpha,
            )));
        }
        fragment
            .iter()
            .take_while(|byte| byte.is_ascii_alphanumeric() || **byte == b'_')
            .count()
    };

    let (rest, _) = nom::bytes::complete::take(consumed).parse(input)?;
    Ok((rest, span_from_to(start, rest)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReferencePathKind {
    Qualified,
    Dotted,
}

/// Whether the segment starting here is an *unquoted* reserved keyword.
///
/// A `QualifiedName`'s segments are `NAME`s, and a reserved keyword is never a `NAME`. A quoted
/// name is not a keyword however it is spelled, so `#'part'` is a legitimate reference and `#part`
/// is not. Used only where a reference sits directly in front of another production's keyword --
/// see [`qualified_reference_without_reserved_names`].
fn segment_is_reserved_keyword(input: Input<'_>) -> bool {
    let fragment = input.fragment();
    if fragment.first().is_some_and(|byte| *byte == b'\'') {
        return false;
    }
    let length = fragment
        .iter()
        .take_while(|byte| byte.is_ascii_alphanumeric() || **byte == b'_')
        .count();
    length > 0 && is_reserved_keyword(&fragment[..length])
}

fn source_backed_reference(
    input: Input<'_>,
    allow_dot: bool,
    require_qualification: bool,
    reject_reserved_segments: bool,
) -> IResult<Input<'_>, (QualifiedReferenceId, ReferencePathKind)> {
    let (input, _) = ws_and_comments(input)?;
    let reference_start = input;
    let (mut rest, absolute) = if input.fragment().starts_with(b"$::") {
        let (input, _) = tag(&b"$::"[..]).parse(input)?;
        let (input, _) = ws_and_comments(input)?;
        (input, true)
    } else {
        (input, false)
    };

    let segments_start = rest;
    if reject_reserved_segments && segment_is_reserved_keyword(rest) {
        return Err(nom::Err::Error(nom::error::Error::new(
            rest,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (next, _) = reference_name_span(rest)?;
    rest = next;
    let mut path_kind = ReferencePathKind::Qualified;
    let mut is_qualified = absolute;

    loop {
        let (after_ws, _) = ws_and_comments(rest)?;
        let separator = if after_ws.fragment().starts_with(b"::") {
            Some((ReferenceSeparator::ColonColon, &b"::"[..]))
        } else if allow_dot && after_ws.fragment().starts_with(b".") {
            Some((ReferenceSeparator::Dot, &b"."[..]))
        } else {
            None
        };
        let Some((separator_before, token)) = separator else {
            break;
        };

        let (after_separator, _) = tag(token).parse(after_ws)?;
        let (after_separator_ws, _) = ws_and_comments(after_separator)?;
        // A `::*`/`::**` import suffix and malformed/trailing separators belong to the caller.
        // Only commit the separator after proving that another authored name follows it.
        if reject_reserved_segments && segment_is_reserved_keyword(after_separator_ws) {
            break;
        }
        let Ok((after_name, source_span)) = reference_name_span(after_separator_ws) else {
            break;
        };
        if separator_before == ReferenceSeparator::Dot {
            path_kind = ReferencePathKind::Dotted;
        }
        is_qualified = true;
        let _ = (source_span, separator_before);
        rest = after_name;
    }

    // Declaration labels use ordinary `String` storage unless their authored grammar is actually
    // qualified. Reject before arena mutation so the simple-name alternative can parse without a
    // speculative orphan entry.
    if require_qualification && !is_qualified {
        return Err(nom::Err::Error(nom::error::Error::new(
            reference_start,
            nom::error::ErrorKind::Tag,
        )));
    }

    let span = span_from_to(reference_start, rest);
    // Walk the already-validated token range a second time and stream segments straight into the
    // packed arena. This avoids a mandatory per-reference `Vec` allocation while keeping parser
    // failure atomic: no arena mutation happens until the whole reference is known to be valid.
    let end_offset = rest.location_offset();
    let mut cursor = segments_start;
    let mut first = true;
    let segments = std::iter::from_fn(move || {
        if cursor.location_offset() >= end_offset {
            return None;
        }
        let separator_before = if first {
            first = false;
            None
        } else {
            let (after_ws, _) = ws_and_comments(cursor).ok()?;
            let (after_separator, separator) = if after_ws.fragment().starts_with(b"::") {
                let (next, _) = tag::<_, _, nom::error::Error<Input<'_>>>(&b"::"[..])
                    .parse(after_ws)
                    .ok()?;
                (next, ReferenceSeparator::ColonColon)
            } else {
                let (next, _) = tag::<_, _, nom::error::Error<Input<'_>>>(&b"."[..])
                    .parse(after_ws)
                    .ok()?;
                (next, ReferenceSeparator::Dot)
            };
            let (after_ws, _) = ws_and_comments(after_separator).ok()?;
            cursor = after_ws;
            Some(separator)
        };
        let (after_name, source_span) = reference_name_span(cursor).ok()?;
        cursor = after_name;
        Some(ReferenceSegment {
            source_span,
            separator_before,
        })
    });
    let id = input
        .extra
        .add_reference(absolute, span, segments)
        .ok_or_else(|| {
            nom::Err::Error(nom::error::Error::new(
                reference_start,
                nom::error::ErrorKind::Fail,
            ))
        })?;
    Ok((rest, (id, path_kind)))
}

/// Parse a source-backed `::`-qualified semantic reference into the document arena.
pub(crate) fn qualified_reference(input: Input<'_>) -> IResult<Input<'_>, QualifiedReferenceId> {
    let (input, (reference, _)) = source_backed_reference(input, false, false, false)?;
    Ok((input, reference))
}

/// [`qualified_reference`], refusing any unquoted reserved keyword as a segment.
///
/// For the one position where a reference sits immediately in front of another production's kind
/// keyword: `PrefixMetadataUsage`'s `OwnedFeatureTyping`. An incomplete extension keyword
/// otherwise swallows the member behind it -- `# part p;` became a metadata reference *named*
/// `part` plus a separate declaration, and `#Tag:: part p;` a reference `Tag::part`, both with no
/// diagnostic about the `#` that was never completed. Rejection happens during the validation
/// walk, before any arena mutation, so nothing speculative is allocated.
pub(crate) fn qualified_reference_without_reserved_names(
    input: Input<'_>,
) -> IResult<Input<'_>, QualifiedReferenceId> {
    let (input, (reference, _)) = source_backed_reference(input, false, false, true)?;
    Ok((input, reference))
}

/// Parse a genuinely qualified declaration identity (`A::B` or `$::A`) into arena-backed storage.
/// A simple `A` is rejected before allocation so the declaration parser can retain it as a label.
pub(crate) fn qualified_declaration_name(
    input: Input<'_>,
) -> IResult<Input<'_>, QualifiedDeclarationName> {
    let (input, (reference, _)) = source_backed_reference(input, false, true, false)?;
    Ok((input, QualifiedDeclarationName::new(reference)))
}

/// Parse a source-backed semantic path with authored `::` and `.` separators.
pub(crate) fn reference_path(input: Input<'_>) -> IResult<Input<'_>, QualifiedReferenceId> {
    let (input, (reference, _)) = source_backed_reference(input, true, false, false)?;
    Ok((input, reference))
}

/// Parse a semantic path and retain whether its accepted grammar contained a dotted feature-chain
/// separator. This prevents expression parsing from rediscovering that syntax by inspecting text.
pub(crate) fn classified_reference_path(
    input: Input<'_>,
) -> IResult<Input<'_>, (QualifiedReferenceId, ReferencePathKind)> {
    source_backed_reference(input, true, false, false)
}

/// Skip any content until we see `}` at the same brace level.
///
/// Nested braces are ignored inside line/block comments and quoted values so recovery cannot
/// mistake authored text for structure and truncate the enclosing body.
pub(crate) fn skip_until_brace_end(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let frag = input.fragment();
    let mut depth = 1u32;
    let mut pos = 0usize;
    let mut block_comment_depth = 0usize;
    let mut line_comment = false;
    let mut quote = None;
    let mut escaped = false;
    while depth > 0 && pos < frag.len() {
        let byte = frag[pos];
        let next = frag.get(pos + 1).copied();
        if line_comment {
            pos += 1;
            if matches!(byte, b'\n' | b'\r') {
                line_comment = false;
            }
            continue;
        }
        if block_comment_depth > 0 {
            if byte == b'/' && next == Some(b'*') {
                block_comment_depth += 1;
                pos += 2;
            } else if byte == b'*' && next == Some(b'/') {
                block_comment_depth -= 1;
                pos += 2;
            } else {
                pos += 1;
            }
            continue;
        }
        if let Some(delimiter) = quote {
            pos += 1;
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == delimiter {
                quote = None;
            }
            continue;
        }

        match (byte, next) {
            (b'/', Some(b'/')) => {
                line_comment = true;
                pos += 2;
            }
            (b'/', Some(b'*')) => {
                block_comment_depth = 1;
                pos += 2;
            }
            (b'\'' | b'"', _) => {
                quote = Some(byte);
                pos += 1;
            }
            (b'{', _) => {
                depth += 1;
                pos += 1;
            }
            (b'}', _) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
                pos += 1;
            }
            _ => pos += 1,
        }
    }
    let (input, _) = nom::bytes::complete::take(pos).parse(input)?;
    Ok((input, ()))
}

const DECLARATION_BOUNDARY_STARTERS: &[&[u8]] = &[
    b"package",
    b"namespace",
    b"import",
    b"part",
    b"attribute",
    b"action",
    b"requirement",
    b"state",
    b"transition",
    b"view",
    b"viewpoint",
    b"rendering",
    b"constraint",
    b"calc",
    b"ref",
    b"port",
    b"perform",
    b"bind",
    b"flow",
    b"first",
    b"merge",
    b"then",
    b"in",
    b"out",
    b"inout",
    b"return",
    b"actor",
    b"subject",
    b"objective",
    b"require",
    b"satisfy",
    b"expose",
    b"doc",
];

fn trim_ascii_end_bytes(mut fragment: &[u8]) -> &[u8] {
    while let Some(last) = fragment.last() {
        if last.is_ascii_whitespace() {
            fragment = &fragment[..fragment.len() - 1];
        } else {
            break;
        }
    }
    fragment
}

fn starts_new_declaration_after_newline(fragment: &[u8], newline_end: usize) -> bool {
    let mut pos = newline_end;
    while pos < fragment.len() && matches!(fragment[pos], b' ' | b'\t' | b'\n' | b'\r') {
        pos += 1;
    }
    let candidate = &fragment[pos..];
    candidate.is_empty()
        || candidate.starts_with(b"}")
        || starts_with_any_keyword(candidate, DECLARATION_BOUNDARY_STARTERS)
}

/// Identification: ( '<' ShortName '>' )? ( Name )?
pub(crate) fn identification(input: Input<'_>) -> IResult<Input<'_>, Identification> {
    let (input, short_name) = short_name_prefix(input)?;
    let (input, decl_name) = opt(preceded(ws_and_comments, name)).parse(input)?;
    Ok((
        input,
        Identification {
            short_name,
            name: decl_name,
        },
    ))
}

/// The `( '<' ShortName '>' )?` half of `Identification` (BNF §8.2.2.2) in isolation, for usage
/// parsers (`attribute_usage`, `part_usage`, `item_usage`, `port_usage`, ...) whose own
/// name-dispatch logic (anonymous colon form vs. named vs. prefix-redefines) can't reuse
/// `identification` wholesale since the name half isn't a plain `opt(name)` for them.
pub(crate) fn short_name_prefix(input: Input<'_>) -> IResult<Input<'_>, Option<String>> {
    opt(delimited(
        preceded(ws_and_comments, tag(&b"<"[..])),
        preceded(ws_and_comments, name),
        preceded(ws_and_comments, tag(&b">"[..])),
    ))
    .parse(input)
}

/// Optional `private` / `protected` / `public` visibility prefix, shared by every `*Def`/`*Usage`
/// parser that needs to feed a [`crate::ast::Membership`] (parser work item 4b, post-PAR-006).
/// Returns the span of the whole prefix (zero-width, positioned at `input`, when no prefix is
/// written) alongside the matched [`crate::ast::Visibility`], so callers can build a `Membership`
/// without re-deriving the span themselves. This consolidates what several parsers (`attribute_def`,
/// `attribute_usage`, `filter_member`, `import`, ...) previously matched ad hoc with their own
/// inline `alt((tag("private"), ...))`; matching and discarding the value is what left every
/// `*Def`/`*Usage` struct with no visibility field before this item.
pub(crate) fn visibility_prefix(
    input: Input<'_>,
) -> IResult<Input<'_>, (crate::ast::Span, Option<crate::ast::Visibility>)> {
    let start = input;
    // Every member start runs this; nearly all of them begin with some other keyword, so refuse
    // on the first byte before the three `tag` trials.
    if !input.fragment().starts_with(b"p") {
        return Ok((input, (crate::parser::span_from_to(start, input), None)));
    }
    let (input, visibility) = opt(alt((
        map(preceded(tag(&b"private"[..]), ws1), |_| {
            crate::ast::Visibility::Private
        }),
        map(preceded(tag(&b"protected"[..]), ws1), |_| {
            crate::ast::Visibility::Protected
        }),
        map(preceded(tag(&b"public"[..]), ws1), |_| {
            crate::ast::Visibility::Public
        }),
    )))
    .parse(input)?;
    let span = crate::parser::span_from_to(start, input);
    Ok((input, (span, visibility)))
}

/// Take input until we hit one of the terminator bytes (e.g. '{' or ';'), return as string (trimmed).
pub(crate) fn take_until_terminator<'a>(
    input: Input<'a>,
    terminators: &'a [u8],
) -> IResult<Input<'a>, String> {
    let frag = input.fragment();
    let mut i = 0;
    let mut quote = None;
    let mut escaped = false;
    while i < frag.len() {
        let byte = frag[i];
        if let Some(delimiter) = quote {
            i += 1;
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == delimiter {
                quote = None;
            }
            continue;
        }
        if matches!(byte, b'\'' | b'"') {
            quote = Some(byte);
            i += 1;
            continue;
        }
        if terminators.contains(&frag[i]) {
            let s = String::from_utf8_lossy(&frag[..i]).trim().to_string();
            let (input, _) = nom::bytes::complete::take(i).parse(input)?;
            return Ok((input, s));
        }
        if terminators.contains(&b';') && matches!(frag[i], b'\n' | b'\r') {
            let mut newline_end = i;
            while newline_end < frag.len() && matches!(frag[newline_end], b'\n' | b'\r') {
                newline_end += 1;
            }
            let consumed = trim_ascii_end_bytes(&frag[..i]);
            let consumed_ends_incomplete = consumed.last().is_some_and(|b| {
                matches!(
                    *b,
                    b':' | b'=' | b',' | b'.' | b'+' | b'-' | b'*' | b'/' | b'>' | b'<' | b'|'
                )
            });
            if !consumed.is_empty()
                && !consumed_ends_incomplete
                && starts_new_declaration_after_newline(frag, newline_end)
            {
                let s = String::from_utf8_lossy(&frag[..i]).trim().to_string();
                let (input, _) = nom::bytes::complete::take(i).parse(input)?;
                return Ok((input, s));
            }
        }
        if frag[i] == b'/' && i + 1 < frag.len() && (frag[i + 1] == b'*' || frag[i + 1] == b'/') {
            break;
        }
        i += 1;
    }
    let s = String::from_utf8_lossy(&frag[..i]).trim().to_string();
    let (input, _) = nom::bytes::complete::take(i).parse(input)?;
    Ok((input, s))
}

/// Skip one unknown statement or balanced block.
///
/// This is used as a recovery mechanism inside body parsers so we can continue
/// parsing later known elements instead of aborting the entire enclosing body.
pub(crate) fn skip_statement_or_block(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) = ws_and_comments(input)?;
    let frag = input.fragment();
    if frag.is_empty() {
        return Ok((input, ()));
    }
    if frag[0] == b'}' {
        return Ok((input, ()));
    }
    if frag[0] == b'{' {
        let (input, _) = tag(&b"{"[..]).parse(input)?;
        let (input, _) = skip_until_brace_end(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b"}"[..])).parse(input)?;
        return Ok((input, ()));
    }

    let mut depth = 0usize;
    let mut pos = 0usize;
    let mut block_comment_depth = 0usize;
    let mut line_comment = false;
    let mut quote = None;
    let mut escaped = false;
    while pos < frag.len() {
        let byte = frag[pos];
        let next = frag.get(pos + 1).copied();
        if line_comment {
            if matches!(byte, b'\n' | b'\r') {
                if depth == 0 {
                    pos += 1;
                    break;
                }
                line_comment = false;
            }
            pos += 1;
            continue;
        }
        if block_comment_depth > 0 {
            if byte == b'/' && next == Some(b'*') {
                block_comment_depth += 1;
                pos += 2;
            } else if byte == b'*' && next == Some(b'/') {
                block_comment_depth -= 1;
                pos += 2;
            } else {
                pos += 1;
            }
            continue;
        }
        if let Some(delimiter) = quote {
            pos += 1;
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == delimiter {
                quote = None;
            }
            continue;
        }

        match (byte, next) {
            (b'/', Some(b'/')) => {
                line_comment = true;
                pos += 2;
            }
            (b'/', Some(b'*')) => {
                block_comment_depth = 1;
                pos += 2;
            }
            (b'\'' | b'"', _) => {
                quote = Some(byte);
                pos += 1;
            }
            (b'{', _) => {
                depth += 1;
                pos += 1;
            }
            (b'}', _) => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
                pos += 1;
                if depth == 0 {
                    break;
                }
            }
            (b';', _) if depth == 0 => {
                pos += 1;
                break;
            }
            _ => pos += 1,
        }
    }
    let advance = pos.max(1).min(frag.len());
    let (input, _) = nom::bytes::complete::take(advance).parse(input)?;
    Ok((input, ()))
}

/// Parse specialization marker in SysML concrete syntax:
/// either symbolic `:>` or keyword `specializes`.
pub(crate) fn specialization_operator(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::TypingSpelling> {
    alt((
        value(crate::ast::TypingSpelling::Operator, tag(&b":>"[..])),
        value(
            crate::ast::TypingSpelling::Specializes,
            terminated(tag(&b"specializes"[..]), ws1),
        ),
    ))
    .parse(input)
}

/// Parse subsetting marker in SysML concrete syntax:
/// either symbolic `:>` or keyword `subsets`.
pub(crate) fn subset_operator(input: Input<'_>) -> IResult<Input<'_>, ()> {
    value((), spelled_subset_operator).parse(input)
}

/// [`subset_operator`], reporting which of the two interchangeable spellings was authored.
pub(crate) fn spelled_subset_operator(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::SubsettingSpelling> {
    use crate::ast::SubsettingSpelling as S;
    alt((
        value(S::Operator, tag(&b":>"[..])),
        value(S::Keyword, terminated(tag(&b"subsets"[..]), ws1)),
    ))
    .parse(input)
}

/// Parse redefinition marker in SysML concrete syntax:
/// either symbolic `:>>` or keyword `redefines`.
pub(crate) fn redefine_operator(input: Input<'_>) -> IResult<Input<'_>, ()> {
    value((), spelled_redefine_operator).parse(input)
}

/// [`redefine_operator`], reporting which of the two interchangeable spellings was authored.
pub(crate) fn spelled_redefine_operator(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::SubsettingSpelling> {
    use crate::ast::SubsettingSpelling as S;
    alt((
        value(S::Operator, tag(&b":>>"[..])),
        value(S::Keyword, terminated(tag(&b"redefines"[..]), ws1)),
    ))
    .parse(input)
}

/// Parse typing marker in SysML concrete syntax:
/// symbolic `:`, or keyword pairs `defined by` / `typed by`.
pub(crate) fn typed_by_operator(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::TypingSpelling> {
    alt((
        value(crate::ast::TypingSpelling::Operator, tag(&b":"[..])),
        value(
            crate::ast::TypingSpelling::DefinedBy,
            (tag(&b"defined"[..]), ws1, tag(&b"by"[..]), ws1),
        ),
        value(
            crate::ast::TypingSpelling::TypedBy,
            (tag(&b"typed"[..]), ws1, tag(&b"by"[..]), ws1),
        ),
    ))
    .parse(input)
}

/// Reference subsetting: `::>` or keyword `references`.
#[allow(dead_code)] // BNF lexical conformance surface; the spelled variant is what parsers use.
pub(crate) fn references_operator(input: Input<'_>) -> IResult<Input<'_>, ()> {
    value((), spelled_references_operator).parse(input)
}

/// [`references_operator`], reporting which of the two interchangeable spellings was authored.
pub(crate) fn spelled_references_operator(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::SubsettingSpelling> {
    use crate::ast::SubsettingSpelling as S;
    alt((
        value(S::Operator, tag(&b"::>"[..])),
        value(S::Keyword, (tag(&b"references"[..]), ws1)),
    ))
    .parse(input)
}

/// Cross subsetting: `=>` or keyword `crosses`.
#[allow(dead_code)] // BNF lexical conformance surface; the spelled variant is what parsers use.
pub(crate) fn crosses_operator(input: Input<'_>) -> IResult<Input<'_>, ()> {
    value((), spelled_crosses_operator).parse(input)
}

/// [`crosses_operator`], reporting which of the two interchangeable spellings was authored.
pub(crate) fn spelled_crosses_operator(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::SubsettingSpelling> {
    use crate::ast::SubsettingSpelling as S;
    alt((
        value(S::Operator, tag(&b"=>"[..])),
        value(S::Keyword, (tag(&b"crosses"[..]), ws1)),
    ))
    .parse(input)
}

/// Conjugation: `~` prefix on types or `conjugates` keyword form.
#[allow(dead_code)]
pub(crate) fn conjugates_operator(input: Input<'_>) -> IResult<Input<'_>, ()> {
    alt((
        value((), tag(&b"~"[..])),
        value((), (tag(&b"conjugates"[..]), ws1)),
    ))
    .parse(input)
}

/// DECIMAL_VALUE: integer or real literal text (for BNF DECIMAL_VALUE / EXPONENTIAL_VALUE).
#[allow(dead_code)] // used by BNF lexical conformance tests in `bnf_surface`
pub(crate) fn decimal_value_text(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    let (input, raw) = take_while1(|c: u8| {
        c.is_ascii_digit() || c == b'.' || c == b'e' || c == b'E' || c == b'+' || c == b'-'
    })
    .parse(input)?;
    Ok((input, String::from_utf8_lossy(raw.fragment()).into_owned()))
}

/// STRING_VALUE: single-quoted unrestricted name content.
#[allow(dead_code)] // used by BNF lexical conformance tests in `bnf_surface`
pub(crate) fn string_value(input: Input<'_>) -> IResult<Input<'_>, String> {
    quoted_name(input)
}

#[cfg(test)]
mod lexical_bnf_tests {
    use super::*;
    use crate::ast::SourceStorage;
    use crate::parser::span::ParseContext;

    fn span_input(text: &str) -> Input<'_> {
        // Convenience for legacy lexer tests that do not inspect the arena. Semantic-reference
        // tests retain and finish their context explicitly below.
        let context = Box::leak(Box::new(ParseContext::new()));
        context.input(text.as_bytes())
    }

    #[test]
    fn name_parses_basic_name() {
        let (_, n) = name(span_input("myPart")).expect("NAME");
        assert_eq!(n, "myPart");
    }

    #[test]
    fn name_parses_unrestricted_name() {
        let (_, n) = name(span_input("'a name'")).expect("UNRESTRICTED_NAME");
        assert_eq!(n, "a name");
    }

    #[test]
    fn name_parses_unrestricted_name_with_degree_symbol() {
        let (_, n) = name(span_input("'\u{00b0}F'")).expect("UNRESTRICTED_NAME");
        assert_eq!(n, "\u{00b0}F");
    }

    #[test]
    fn qualified_name_parses_scoped_name() {
        let source_text = "SI::kg";
        let context = ParseContext::new();
        let (rest, id) =
            qualified_reference(context.input(source_text.as_bytes())).expect("QualifiedName");
        assert!(rest.fragment().is_empty());
        let arena = context.finish();
        let source = SourceStorage::from(source_text);
        assert_eq!(
            arena
                .get(&source, id)
                .expect("reference view")
                .authored_text(),
            source_text
        );
    }

    #[test]
    fn qualified_declaration_name_requires_qualification_before_allocating() {
        let simple_context = ParseContext::new();
        assert!(qualified_declaration_name(simple_context.input(b"Simple")).is_err());
        assert!(simple_context.finish().is_empty());

        let source_text = "AstronomyReference::Domain";
        let context = ParseContext::new();
        let (rest, declaration) = qualified_declaration_name(context.input(source_text.as_bytes()))
            .expect("qualified declaration name");
        assert!(rest.fragment().is_empty());
        let arena = context.finish();
        let source = SourceStorage::from(source_text);
        let view = arena
            .get(&source, declaration.storage_id())
            .expect("qualified declaration view");
        assert_eq!(view.authored_text(), source_text);
        assert_eq!(view.segments.len(), 2);
        assert_eq!(
            view.segments[1].separator_before,
            Some(ReferenceSeparator::ColonColon)
        );
    }

    #[test]
    fn qualified_name_requires_atomic_absolute_prefix() {
        let source_text = "$::SI::kg";
        let context = ParseContext::new();
        let (rest, id) = qualified_reference(context.input(source_text.as_bytes()))
            .expect("absolute QualifiedName");
        assert!(rest.fragment().is_empty());
        let arena = context.finish();
        let source = SourceStorage::from(source_text);
        let view = arena.get(&source, id).expect("reference view");
        assert!(view.metadata.is_absolute);
        assert_eq!(view.authored_text(), source_text);
        assert!(qualified_reference(span_input("$SI::kg")).is_err());
        assert!(qualified_reference(span_input("::SI::kg")).is_err());
    }

    #[test]
    fn source_backed_reference_captures_mixed_separators_and_exact_spans() {
        let source_text = "Vehicle::'mass value'.amount";
        let context = ParseContext::new();
        let (rest, id) = reference_path(context.input(source_text.as_bytes()))
            .expect("source-backed reference path");
        assert!(rest.fragment().is_empty());
        let arena = context.finish();
        let source = SourceStorage::from(source_text);
        arena.validate(&source).expect("valid arena");
        let view = arena.get(&source, id).expect("reference view");

        assert!(!view.metadata.is_absolute);
        assert_eq!(view.authored_text(), source_text);
        assert_eq!(view.segments.len(), 3);
        assert_eq!(view.segment_authored_text(0), Some("Vehicle"));
        assert_eq!(view.segment_authored_text(1), Some("'mass value'"));
        assert_eq!(view.segment_decoded_text(1).as_deref(), Some("mass value"));
        assert_eq!(view.segment_authored_text(2), Some("amount"));
        assert_eq!(
            view.segments[1].separator_before,
            Some(ReferenceSeparator::ColonColon)
        );
        assert_eq!(
            view.segments[2].separator_before,
            Some(ReferenceSeparator::Dot)
        );
        assert_eq!(
            view.segments[1].source_span,
            Span {
                offset: 9,
                line: 1,
                column: 10,
                len: 12,
            }
        );
    }

    #[test]
    fn source_backed_absolute_reference_uses_metadata_not_segment() {
        let source_text = "$::Library::Thing";
        let context = ParseContext::new();
        let (rest, id) = qualified_reference(context.input(source_text.as_bytes()))
            .expect("absolute qualified reference");
        assert!(rest.fragment().is_empty());
        let arena = context.finish();
        let source = SourceStorage::from(source_text);
        let view = arena.get(&source, id).expect("reference view");

        assert!(view.metadata.is_absolute);
        assert_eq!(view.segments.len(), 2);
        assert_eq!(view.segment_authored_text(0), Some("Library"));
        assert_eq!(view.segment_authored_text(1), Some("Thing"));
        assert!(view
            .segments
            .iter()
            .all(|segment| source.slice(&segment.source_span) != Some("$")));
    }

    #[test]
    fn source_backed_reference_rejects_malformed_absolute_and_quoted_names() {
        let context = ParseContext::new();
        assert!(qualified_reference(context.input(b"$Library::Thing")).is_err());
        assert!(qualified_reference(context.input(b"::Library::Thing")).is_err());
        assert!(qualified_reference(context.input(b"'unterminated")).is_err());
    }

    #[test]
    fn qualified_reference_leaves_import_wildcard_suffix_for_shape_parser() {
        let context = ParseContext::new();
        let (rest, _) = qualified_reference(context.input(b"Library::*::**"))
            .expect("qualified reference before suffix");
        assert_eq!(*rest.fragment(), &b"::*::**"[..]);
    }

    #[test]
    fn recovery_skip_balances_braces_outside_quotes_and_nested_comments() {
        let input = span_input(
            "broken { text = \"}\"; /* outer { /* inner } */ still } */ nested { x; } } part ok;",
        );
        let (rest, ()) = skip_statement_or_block(input).expect("balanced recovery skip");
        assert_eq!(*rest.fragment(), &b" part ok;"[..]);
    }

    #[test]
    fn local_recovery_does_not_sync_to_a_keyword_inside_a_quoted_name() {
        let input = span_input("broken 'line one\npart fake'\npart good;");
        let (rest, ()) = recover_body_element(input, &[b"part"]).expect("body recovery");
        assert_eq!(*rest.fragment(), &b"part good;"[..]);
    }

    #[test]
    fn terminator_scan_ignores_punctuation_inside_quotes() {
        let input = span_input(r#"name "};"; next"#);
        let (rest, captured) = take_until_terminator(input, b";{").expect("quoted terminator scan");
        assert_eq!(captured, r#"name "};""#);
        assert_eq!(*rest.fragment(), &b"; next"[..]);
    }

    #[test]
    fn string_value_parses_quoted() {
        let (_, s) = string_value(span_input("'x'")).expect("STRING_VALUE");
        assert_eq!(s, "x");
    }

    #[test]
    fn decimal_value_parses_real() {
        let (_, v) = decimal_value_text(span_input("1.5e-3")).expect("DECIMAL_VALUE");
        assert_eq!(v, "1.5e-3");
    }

    #[test]
    fn ws_and_comments_skip_line_and_block() {
        let input = span_input("  // line\n  /* block */  part");
        let (rest, _) = ws_and_comments(input).expect("WHITE_SPACE");
        assert!(rest.fragment().starts_with(b"part"));
    }

    #[test]
    fn ws_and_comments_starts_a_block_comment_at_slash_slash_star() {
        // `//*` is a block comment, so the newline inside it does not end it.
        let input = span_input("//* still\n comment */ part");
        let (rest, _) = ws_and_comments(input).expect("WHITE_SPACE");
        assert_eq!(*rest.fragment(), &b"part"[..]);
    }

    #[test]
    fn ws_and_comments_treats_unterminated_slash_slash_star_as_a_line_comment() {
        let input = span_input("//* never closed\npart");
        let (rest, _) = ws_and_comments(input).expect("WHITE_SPACE");
        assert_eq!(*rest.fragment(), &b"part"[..]);
    }

    #[test]
    fn ws_and_comments_leaves_an_unterminated_block_comment_for_the_caller() {
        // An unterminated `/*` is not trivia: the scan stops in front of it so the caller can
        // report it rather than silently consuming the rest of the document.
        let input = span_input("  /* never closed\npart");
        let (rest, _) = ws_and_comments(input).expect("WHITE_SPACE");
        assert!(rest.fragment().starts_with(b"/* never closed"));
    }

    #[test]
    fn ws_and_comments_consumes_runs_of_mixed_trivia() {
        let input = span_input("\r\n\t // a\r// b\n/* c */\n//* d */\t part");
        let (rest, _) = ws_and_comments(input).expect("WHITE_SPACE");
        assert_eq!(*rest.fragment(), &b"part"[..]);
    }

    #[test]
    fn ws_and_comments_is_a_no_op_on_a_token() {
        let input = span_input("part p;");
        let (rest, _) = ws_and_comments(input).expect("WHITE_SPACE");
        assert_eq!(rest.location_offset(), 0);
    }

    #[test]
    fn references_operator_accepts_symbol_and_keyword() {
        let (_, _) = references_operator(span_input("::>")).expect("REFERENCES");
        let (_, _) = references_operator(span_input("references ")).expect("REFERENCES");
    }

    #[test]
    fn crosses_operator_accepts_symbol_and_keyword() {
        let (_, _) = crosses_operator(span_input("=>")).expect("CROSSES");
        let (_, _) = crosses_operator(span_input("crosses ")).expect("CROSSES");
    }
}
