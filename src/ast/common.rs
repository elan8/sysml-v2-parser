use crate::ast::core::{Expression, Node, Span};
use crate::ast::membership::Membership;
use crate::ast::QualifiedReferenceId;

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

/// Typed suffix form of an import or expose target.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImportShape {
    /// A single membership, optionally including recursively imported memberships (`::**`).
    Membership { recursive: bool },
    /// All memberships of a namespace (`::*`), optionally recursively (`::*::**`).
    Namespace { recursive: bool },
    /// A filter package, with its filter expressions retained as typed AST nodes.
    Filter {
        recursive: bool,
        members: Vec<Node<FilterPackageMember>>,
    },
}

/// Source-backed reference plus the import/expose suffix that applies to it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportTarget {
    pub reference: QualifiedReferenceId,
    pub shape: ImportShape,
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
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextualRepresentation {
    pub rep_identification: Option<Identification>,
    pub language: String,
    pub language_span: Option<Span>,
    pub text: String,
}

impl PartialEq for TextualRepresentation {
    fn eq(&self, other: &Self) -> bool {
        self.rep_identification == other.rep_identification
            && self.language == other.language
            && self.text == other.text
    }
}
/// Body of a connect statement: `;` or `{` ... `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConnectBody {
    Semicolon,
    Brace,
}
