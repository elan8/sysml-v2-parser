use crate::ast::core::{Expression, Node, Span, StringLiteral};
use crate::ast::membership::Membership;
use crate::ast::QualifiedReferenceId;
#[cfg(feature = "serde")]
use crate::ast::{QualifiedReferenceArena, SourceStorage};
#[cfg(feature = "serde")]
use std::fmt;

/// KerML ElementFilterMember: MemberPrefix? 'filter' condition ';'
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FilterMember {
    pub visibility: Option<Visibility>,
    pub condition: Node<Expression>,
}

/// Placeholder node inserted when resilient parsing skips malformed input.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParseErrorNode {
    pub message: String,
    pub code: String,
    pub expected: Option<String>,
    pub found: Option<String>,
    pub suggestion: Option<String>,
    pub category: Option<crate::error::DiagnosticCategory>,
}

/// A spec-valid grammar production that is not implemented in the containing parser scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnsupportedProduction {
    BindingConnectorAsUsage,
    Message,
    SuccessionAsUsage,
    PerformActionUsage,
    ExhibitStateUsage,
    IncludeUseCaseUsage,
    ReferenceConnectionUsage,
    ConnectionUsageInPartDefinition,
    ActionBodyMember,
    /// A spec-valid member of a definition or usage body that this scope does not model yet.
    /// The body it appeared in is identified by the diagnostic's span and message.
    UnmodelledBodyMember,
}

/// Explicit AST state for valid grammar that this parser cannot yet model in the current scope.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnsupportedGrammarNode {
    pub production: UnsupportedProduction,
    pub diagnostic: ParseErrorNode,
}
/// An authored declaration name or short name (`NAME`: `BASIC_NAME` or `UNRESTRICTED_NAME`).
///
/// The document source is authoritative for the spelling, so this is a typed span into it rather
/// than a copy of the token: the span covers the exact authored token, including the quotes and
/// escapes of an unrestricted name. Resolve it through the owning document with
/// [`crate::ast::ParsedDocument::declaration_name`] (authored spelling) or
/// [`crate::ast::ParsedDocument::decoded_declaration_name`] (decoded `NAME` value).
///
/// It is a declaration label, not a reference: it never resolves to a
/// [`crate::ast::QualifiedReferenceId`], and equality compares provenance, not spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeclarationName {
    pub(crate) span: Span,
}

impl DeclarationName {
    pub(crate) fn new(span: Span) -> Self {
        Self { span }
    }

    /// The exact source span of the authored token.
    pub fn span(&self) -> &Span {
        &self.span
    }
}

/// Identification: optional short name in `< >`, optional name.
/// BNF: ( '<' declaredShortName = NAME '>' )? ( declaredName = NAME )?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identification {
    /// Short name inside `< ... >`, if present.
    pub short_name: Option<DeclarationName>,
    /// Main declared name (may be quoted, e.g. `'1a-Parts Tree'`).
    pub name: Option<DeclarationName>,
}

/// Visibility for imports and members.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

/// KerML FilterPackageMember: `[` OwnedExpression `]`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FilterPackageMember {
    /// Exact `[` token introducing this member.
    pub open_bracket_span: Span,
    pub expression: Node<Expression>,
    /// Exact `]` token terminating this member.
    pub close_bracket_span: Span,
}

/// Exact source provenance for one import/expose suffix such as `::*` or `::**`.
///
/// `span` covers the whole suffix, including trivia between the separator and marker. The token
/// spans retain the exact grammatical pieces so consumers never need to scan that aggregate text.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportSuffixSpans {
    pub span: Span,
    pub separator_span: Span,
    pub marker_span: Span,
}

/// Typed suffix form of an import or expose target.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportShape {
    /// A single membership, optionally including recursively imported memberships (`::**`).
    Membership {
        recursive_suffix: Option<ImportSuffixSpans>,
    },
    /// All memberships of a namespace (`::*`), optionally recursively (`::*::**`).
    Namespace {
        wildcard_suffix: ImportSuffixSpans,
        recursive_suffix: Option<ImportSuffixSpans>,
        /// Aggregate span of the complete `::*::**` form, including trivia between suffixes.
        /// Present exactly when `recursive_suffix` is present.
        combined_recursive_suffix_span: Option<Span>,
    },
    /// A filter package, with its filter expressions retained as typed AST nodes.
    Filter {
        recursive_suffix: Option<ImportSuffixSpans>,
        members: Vec<Node<FilterPackageMember>>,
    },
}

/// Source-backed reference plus the import/expose suffix that applies to it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportTarget {
    /// Exact span from the first qualified-name token through the final suffix or filter member.
    pub span: Span,
    /// Exact authored `all` token on an import. Always `None` for expose targets.
    pub all_span: Option<Span>,
    pub reference: QualifiedReferenceId,
    pub shape: ImportShape,
}

/// Failure while validating source provenance retained by an [`ImportTarget`].
#[cfg(feature = "serde")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportTargetValidationError {
    DanglingReference {
        id: QualifiedReferenceId,
    },
    InvalidSpan {
        role: &'static str,
    },
    UnexpectedToken {
        role: &'static str,
        expected: &'static str,
    },
    InvalidOrdering {
        before: &'static str,
        after: &'static str,
    },
    NonTriviaGap {
        before: &'static str,
        after: &'static str,
    },
    ShapeInvariant {
        message: &'static str,
    },
}

#[cfg(feature = "serde")]
impl fmt::Display for ImportTargetValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DanglingReference { id } => {
                write!(
                    formatter,
                    "import target contains dangling reference {id:?}"
                )
            }
            Self::InvalidSpan { role } => write!(formatter, "invalid {role} span"),
            Self::UnexpectedToken { role, expected } => {
                write!(formatter, "{role} span does not contain {expected:?}")
            }
            Self::InvalidOrdering { before, after } => {
                write!(formatter, "{before} must end before {after} begins")
            }
            Self::NonTriviaGap { before, after } => {
                write!(
                    formatter,
                    "non-trivia source occurs between {before} and {after}"
                )
            }
            Self::ShapeInvariant { message } => formatter.write_str(message),
        }
    }
}

#[cfg(feature = "serde")]
impl std::error::Error for ImportTargetValidationError {}

#[cfg(feature = "serde")]
impl ImportTarget {
    pub(crate) fn validate_provenance(
        &self,
        source: &SourceStorage,
        arena: &QualifiedReferenceArena,
    ) -> Result<(), ImportTargetValidationError> {
        let reference = arena
            .get(source, self.reference)
            .ok_or(ImportTargetValidationError::DanglingReference { id: self.reference })?;
        let reference_end = span_end(&reference.metadata.span, "reference")?;

        validate_span(source, &self.span, "import target")?;
        let expected_start = self
            .all_span
            .as_ref()
            .map_or(reference.metadata.span.offset, |span| span.offset);
        if self.span.offset != expected_start {
            return Err(ImportTargetValidationError::ShapeInvariant {
                message: "import target span must begin with 'all' when present, otherwise with its qualified reference",
            });
        }
        if let Some(all_span) = &self.all_span {
            validate_token(source, all_span, "all", "import all")?;
            validate_trivia_order(
                source,
                span_end(all_span, "import all")?,
                reference.metadata.span.offset,
                "import all",
                "reference",
            )?;
        }

        let target_end = match &self.shape {
            ImportShape::Membership { recursive_suffix } => {
                if let Some(suffix) = recursive_suffix {
                    validate_suffix(source, suffix, "**", "recursive suffix")?;
                    validate_trivia_order(
                        source,
                        reference_end,
                        suffix.span.offset,
                        "reference",
                        "recursive suffix",
                    )?;
                    span_end(&suffix.span, "recursive suffix")?
                } else {
                    reference_end
                }
            }
            ImportShape::Namespace {
                wildcard_suffix,
                recursive_suffix,
                combined_recursive_suffix_span,
            } => {
                validate_suffix(source, wildcard_suffix, "*", "wildcard suffix")?;
                validate_trivia_order(
                    source,
                    reference_end,
                    wildcard_suffix.span.offset,
                    "reference",
                    "wildcard suffix",
                )?;
                match (recursive_suffix, combined_recursive_suffix_span) {
                    (Some(recursive), Some(combined)) => {
                        validate_suffix(source, recursive, "**", "recursive suffix")?;
                        let wildcard_end = span_end(&wildcard_suffix.span, "wildcard suffix")?;
                        validate_trivia_order(
                            source,
                            wildcard_end,
                            recursive.span.offset,
                            "wildcard suffix",
                            "recursive suffix",
                        )?;
                        validate_span(source, combined, "combined recursive suffix")?;
                        let recursive_end = span_end(&recursive.span, "recursive suffix")?;
                        if combined.offset != wildcard_suffix.span.offset
                            || span_end(combined, "combined recursive suffix")? != recursive_end
                        {
                            return Err(ImportTargetValidationError::ShapeInvariant {
                                message: "combined recursive suffix span must cover wildcard and recursive suffixes exactly",
                            });
                        }
                        recursive_end
                    }
                    (None, None) => span_end(&wildcard_suffix.span, "wildcard suffix")?,
                    (Some(_), None) | (None, Some(_)) => {
                        return Err(ImportTargetValidationError::ShapeInvariant {
                            message: "combined recursive suffix span must be present exactly when the recursive suffix is present",
                        });
                    }
                }
            }
            ImportShape::Filter {
                recursive_suffix,
                members,
            } => {
                if members.is_empty() {
                    return Err(ImportTargetValidationError::ShapeInvariant {
                        message: "filter import must contain at least one member",
                    });
                }
                let mut previous_end = reference_end;
                let mut previous_role = "reference";
                if let Some(suffix) = recursive_suffix {
                    validate_suffix(source, suffix, "**", "recursive suffix")?;
                    validate_trivia_order(
                        source,
                        reference_end,
                        suffix.span.offset,
                        "reference",
                        "recursive suffix",
                    )?;
                    previous_end = span_end(&suffix.span, "recursive suffix")?;
                    previous_role = "recursive suffix";
                }
                for member in members {
                    validate_filter_member(source, member, previous_end, previous_role)?;
                    previous_end = span_end(&member.span, "filter member")?;
                    previous_role = "filter member";
                }
                previous_end
            }
        };
        if span_end(&self.span, "import target")? != target_end {
            return Err(ImportTargetValidationError::ShapeInvariant {
                message: "import target span must end with its final suffix or filter member",
            });
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
fn validate_suffix(
    source: &SourceStorage,
    suffix: &ImportSuffixSpans,
    marker: &'static str,
    role: &'static str,
) -> Result<(), ImportTargetValidationError> {
    validate_span(source, &suffix.span, role)?;
    validate_token(source, &suffix.separator_span, "::", "suffix separator")?;
    validate_token(source, &suffix.marker_span, marker, "suffix marker")?;
    let separator_end = span_end(&suffix.separator_span, "suffix separator")?;
    let marker_end = span_end(&suffix.marker_span, "suffix marker")?;
    if suffix.span.offset != suffix.separator_span.offset
        || span_end(&suffix.span, role)? != marker_end
    {
        return Err(ImportTargetValidationError::ShapeInvariant {
            message: "suffix aggregate span must cover its separator and marker exactly",
        });
    }
    validate_trivia_order(
        source,
        separator_end,
        suffix.marker_span.offset,
        "suffix separator",
        "suffix marker",
    )
}

#[cfg(feature = "serde")]
fn validate_filter_member(
    source: &SourceStorage,
    member: &Node<FilterPackageMember>,
    previous_end: usize,
    previous_role: &'static str,
) -> Result<(), ImportTargetValidationError> {
    validate_span(source, &member.span, "filter member")?;
    validate_token(
        source,
        &member.value.open_bracket_span,
        "[",
        "filter open bracket",
    )?;
    validate_span(source, &member.value.expression.span, "filter expression")?;
    validate_token(
        source,
        &member.value.close_bracket_span,
        "]",
        "filter close bracket",
    )?;
    let open_end = span_end(&member.value.open_bracket_span, "filter open bracket")?;
    let expression_end = span_end(&member.value.expression.span, "filter expression")?;
    let close_end = span_end(&member.value.close_bracket_span, "filter close bracket")?;
    if member.span.offset != member.value.open_bracket_span.offset
        || span_end(&member.span, "filter member")? != close_end
    {
        return Err(ImportTargetValidationError::ShapeInvariant {
            message: "filter member span must cover its delimiters exactly",
        });
    }
    validate_trivia_order(
        source,
        previous_end,
        member.value.open_bracket_span.offset,
        previous_role,
        "filter open bracket",
    )?;
    validate_trivia_order(
        source,
        open_end,
        member.value.expression.span.offset,
        "filter open bracket",
        "filter expression",
    )?;
    validate_trivia_order(
        source,
        expression_end,
        member.value.close_bracket_span.offset,
        "filter expression",
        "filter close bracket",
    )
}

#[cfg(feature = "serde")]
fn validate_span(
    source: &SourceStorage,
    span: &Span,
    role: &'static str,
) -> Result<(), ImportTargetValidationError> {
    if source.validates_span(span) {
        Ok(())
    } else {
        Err(ImportTargetValidationError::InvalidSpan { role })
    }
}

#[cfg(feature = "serde")]
fn validate_token(
    source: &SourceStorage,
    span: &Span,
    expected: &'static str,
    role: &'static str,
) -> Result<(), ImportTargetValidationError> {
    validate_span(source, span, role)?;
    if source.slice(span) == Some(expected) {
        Ok(())
    } else {
        Err(ImportTargetValidationError::UnexpectedToken { role, expected })
    }
}

#[cfg(feature = "serde")]
fn validate_trivia_order(
    source: &SourceStorage,
    before_end: usize,
    after_start: usize,
    before: &'static str,
    after: &'static str,
) -> Result<(), ImportTargetValidationError> {
    if before_end > after_start {
        return Err(ImportTargetValidationError::InvalidOrdering { before, after });
    }
    if !source.trivia_between(before_end, after_start) {
        return Err(ImportTargetValidationError::NonTriviaGap { before, after });
    }
    Ok(())
}

#[cfg(feature = "serde")]
fn span_end(span: &Span, role: &'static str) -> Result<usize, ImportTargetValidationError> {
    span.offset
        .checked_add(span.len)
        .ok_or(ImportTargetValidationError::InvalidSpan { role })
}

/// Import: `private`? `import` `all`? QualifiedName (`::` `*`)? or FilterPackage form.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Import {
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006 continuation), `kind`
    /// always [`crate::ast::MembershipKind::Import`] -- the variant reserved for this struct since
    /// `Membership`'s introduction, previously unconstructed.
    ///
    /// **Design decision**: this *replaces* the pre-existing `visibility: Option<Visibility>`
    /// field (rather than adding `membership` alongside it, the other option the prior
    /// `AttributeDef`/`AttributeUsage` increment's scope-boundary note left open) -- `Import`'s old
    /// `visibility` field already captured exactly the same information
    /// `Membership::visibility` does (an optional `private`/`protected`/`public` prefix, with no
    /// separate ownership/kind data to preserve alongside it), and grepping the whole crate
    /// (`src/`, `tests/`) found no in-crate consumer reading `Import.visibility` other than its own
    /// constructor in `import.rs` -- so a dual-field design would only have added a redundant,
    /// confusing field with no compatibility benefit. This is a breaking `PARSE_AST_VERSION` change
    /// either way, matching every other struct this rollout has touched.
    pub membership: Membership,
    pub target: ImportTarget,
    /// Real annotation content from a braced body (BNF `RelationshipBody`: doc/comment/metadata
    /// only). `None` when the body is a semicolon terminator.
    pub body_elements: Option<Vec<Node<crate::ast::structure::RelationshipBodyElement>>>,
}
/// The body of a `REGULAR_COMMENT`: the authored bytes between `/*` and `*/`, as a span.
///
/// The source is authoritative for the text, so nothing is copied. Resolve through
/// [`crate::ast::ParsedDocument::comment_body`] (raw, what the formatter reproduces) or
/// [`crate::ast::ParsedDocument::normalized_comment_body`] (with the pinned processing rules
/// applied; see [`normalize_comment_body`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommentBody {
    pub(crate) span: Span,
}

impl CommentBody {
    pub(crate) fn new(span: Span) -> Self {
        Self { span }
    }

    /// Source span of the body, i.e. the `REGULAR_COMMENT` without its delimiters.
    pub fn span(&self) -> &Span {
        &self.span
    }
}

/// KerML Documentation: 'doc' Identification? ( 'locale' STRING_VALUE )? body = REGULAR_COMMENT.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DocComment {
    /// Optional identification after 'doc'.
    pub identification: Option<Identification>,
    /// The authored `locale` string literal, e.g. `"en"`, if present.
    pub locale: Option<StringLiteral>,
    /// The `REGULAR_COMMENT` body.
    pub body: CommentBody,
}

/// KerML Comment: `( 'comment' Identification ( 'about' Annotation ( ',' Annotation )* )? )?
/// ( 'locale' STRING_VALUE )? body = REGULAR_COMMENT`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommentAnnotation {
    /// Span of the authored `comment` keyword, or `None` for the keyword-less spelling.
    ///
    /// The grammar makes the keyword optional, so whether it was written is a grammatical fact
    /// and not a formatting choice: without it the member cannot be re-emitted as a member at
    /// all, because a bare `/* ... */` reparses as trivia. Presence is what emission depends on;
    /// the span keeps the provenance rather than reducing the fact to a boolean.
    pub keyword_span: Option<Span>,
    pub identification: Option<Identification>,
    /// Elements this comment annotates: `comment about a, b /* ... */`.
    ///
    /// Each entry is one `Annotation = annotatedElement = [QualifiedName]`, in authored order.
    /// Empty when no `about` clause was written -- the clause is optional, and an empty list is
    /// what "absent" means for a repeated production, so no separate `Option` is warranted.
    /// [`MetadataAnnotation::about_targets`](crate::ast::MetadataAnnotation::about_targets)
    /// records the same clause of the same shape.
    pub about_targets: Vec<QualifiedReferenceId>,
    /// The authored `locale` string literal, if present.
    pub locale: Option<StringLiteral>,
    /// The `REGULAR_COMMENT` body.
    pub body: CommentBody,
}

/// Apply the pinned `REGULAR_COMMENT` body processing (KerML BNF 214 note 1, which BNF 231 note
/// 1 extends to `TextualRepresentation`) to an authored comment body.
///
/// The delimiters are already stripped by the parser, so the remaining rules are:
///
/// - remove any white space immediately after the initial `/*`, up to and including the first
///   line terminator, if any;
/// - on each subsequent line, strip leading white space other than line terminators, then one
///   leading `*` if present, then one following space if present.
///
/// This lives here, beside the three types that own such a body, because the alternative is every
/// consumer normalizing it slightly differently and then disagreeing about what a document says.
/// It is deliberately *not* applied at parse time: the [`CommentBody`] span keeps the authored
/// bytes in the source so the formatter can reproduce them, and normalization is a view over
/// that. A body the rules leave untouched is returned borrowed.
pub fn normalize_comment_body(raw: &str) -> std::borrow::Cow<'_, str> {
    let normalized = normalize_comment_body_owned(raw);
    if normalized == raw {
        std::borrow::Cow::Borrowed(raw)
    } else {
        std::borrow::Cow::Owned(normalized)
    }
}

fn normalize_comment_body_owned(raw: &str) -> String {
    let mut lines = raw.split_inclusive('\n');
    let mut out = String::with_capacity(raw.len());

    // The first line is special: only the run of white space after `/*` is removed, and only
    // through the first line terminator. A body written entirely on one line therefore keeps its
    // interior spacing.
    if let Some(first) = lines.next() {
        let trimmed = first.trim_start_matches([' ', '\t', '\r', '\x0c']);
        if trimmed.starts_with('\n') {
            // The white space ran to the end of the line: drop the line terminator too.
        } else {
            out.push_str(trimmed);
            if !trimmed.ends_with('\n') {
                return out;
            }
        }
    }

    for line in lines {
        let (body, terminator) = match line.strip_suffix('\n') {
            Some(body) => (body.strip_suffix('\r').unwrap_or(body), true),
            None => (line, false),
        };
        let body = body.trim_start_matches([' ', '\t', '\x0c']);
        let body = body.strip_prefix('*').unwrap_or(body);
        let body = body.strip_prefix(' ').unwrap_or(body);
        out.push_str(body);
        if terminator {
            out.push('\n');
        }
    }
    out
}

/// KerML TextualRepresentation: ( 'rep' Identification )? 'language' STRING_VALUE body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextualRepresentation {
    pub rep_identification: Option<Identification>,
    /// The authored `language` string literal. The grammar requires it; the parser is resilient
    /// and records `None` when it is missing, which the diagnostic collector reports.
    pub language: Option<StringLiteral>,
    /// The body, processed with the same rules as a `Comment` (KerML BNF 231 note 1).
    pub body: CommentBody,
}
/// A member that annotates its owner rather than declaring structure or behavior.
///
/// This is the grammar's own `AnnotatingElement` production, which both layers define
/// identically -- KerML 8.2.3.3.1 and SysML 8.2.2.4.1 -- as
/// `Comment | Documentation | TextualRepresentation | MetadataFeature`. A scope opts into the
/// whole production or none of it: the family is not a place to collect members that merely look
/// alike, and a scope that accepts only some of these members keeps them as its own variants
/// until the parser supports the rest, so the type never claims coverage the parser lacks.
///
/// Two related forms are deliberately *not* here:
///
/// - `#Name` prefix metadata is `PrefixMetadataMember` (SysML 8.2.2.20.2), a prefix on a
///   declaration rather than a member of a body, so it stays a scope-specific variant; and
/// - `metadata name : Type;`, the other spelling of `MetadataUsage`, is modelled by a separate
///   AST type whose body and ownership differ, and folding the two spellings together is a
///   grammar question for the metadata audit rather than something this family should decide.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnnotatingMember {
    /// `doc /* ... */`, optionally named.
    Doc(Node<DocComment>),
    /// `comment [about ...] /* ... */`, optionally named and locale-tagged.
    Comment(Node<CommentAnnotation>),
    /// `rep [name] language "..." /* ... */`.
    TextualRep(Node<TextualRepresentation>),
    /// The `@Name [about ...]` spelling, or AttributeBody's literal `metadata Name [about ...]`
    /// spelling, of a metadata feature.
    MetadataAnnotation(Node<crate::ast::MetadataAnnotation>),
}
