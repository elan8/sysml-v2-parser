use crate::ast::core::{Expression, Node, Span};
use crate::ast::membership::Membership;

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
/// Identification: optional short name in `< >`, optional name.
/// BNF: ( '<' declaredShortName = NAME '>' )? ( declaredName = NAME )?
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identification {
    /// Short name inside `< ... >`, if present.
    pub short_name: Option<String>,
    /// Main declared name (may be quoted, e.g. '1a-Parts Tree').
    pub name: Option<String>,
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
    pub expression: Node<Expression>,
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
    /// Whether this is a namespace import (QualifiedName::* or FilterPackage) or membership import (single QualifiedName).
    pub is_import_all: bool,
    /// Import target, e.g. "SI::kg" or "Definitions::*".
    pub target: String,
    /// Source span of the qualified name in `target` (excludes `::*` / `::**` suffix).
    /// Used by semantic-token providers to highlight only the name portion.
    pub target_span: Span,
    /// KerML: optional recursive import after :: (e.g. QualifiedName::** or QualifiedName::*::**).
    pub is_recursive: bool,
    /// KerML FilterPackage form: one or more `[ expr ]` members. When present, this is a namespace import of a filter package.
    pub filter_members: Option<Vec<Node<FilterPackageMember>>>,
}
/// KerML Documentation: 'doc' Identification? ( 'locale' STRING_VALUE )? body = REGULAR_COMMENT.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DocComment {
    /// Optional identification after 'doc'.
    pub identification: Option<Identification>,
    /// Optional locale string (e.g. "en").
    pub locale: Option<String>,
    /// Body text (content between /* and */).
    pub text: String,
}

/// KerML Comment: ( 'comment' Identification? )? ( 'locale' STRING_VALUE )? body = REGULAR_COMMENT.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommentAnnotation {
    pub identification: Option<Identification>,
    pub locale: Option<String>,
    pub text: String,
}

/// KerML TextualRepresentation: ( 'rep' Identification )? 'language' STRING_VALUE body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextualRepresentation {
    pub rep_identification: Option<Identification>,
    pub language: String,
    pub language_span: Option<Span>,
    pub text: String,
}
/// Body of a connect statement: `;` or `{` ... `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConnectBody {
    Semicolon,
    Brace,
}
