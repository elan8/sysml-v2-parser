use super::behavior::InOutDecl;
use super::common::FilterMember;
use super::common::{ConnectBody, DocComment, Identification, ParseErrorNode};
use super::requirement::RequirementDefBody;
use super::structure::MetadataAnnotation;
use crate::ast::core::{Expression, Node, TypingRelationship};

/// Constraint definition: `constraint def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstraintDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: ConstraintDefBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConstraintDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ConstraintDefBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConstraintDefBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    InOutDecl(Node<InOutDecl>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    Expression(Node<Expression>), // e.g. totalThrust >= totalWeight * margin
    /// Unmodeled constraint-body element captured as raw text (used for library parsing).
    Other(String),
}

/// constraint body {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConstraintBody {
    Semicolon,
    Brace, // Often contains docs or block of expressions
}

/// Calc definition: `calc def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CalcDef {
    pub identification: Identification,
    pub body: CalcDefBody,
}

/// Calculation usage: `calc` Identification (`:` type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CalcUsage {
    pub identification: Identification,
    pub type_name: Option<String>,
    pub body: CalcDefBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CalcDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<CalcDefBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CalcDefBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    InOutDecl(Node<InOutDecl>),
    ReturnDecl(Node<ReturnDecl>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    Expression(Node<Expression>), // formula
    /// Unmodeled calc-body element captured as raw text (used for library parsing).
    Other(String),
}

/// Return declaration: `return` name `:` type `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReturnDecl {
    pub name: String,
    pub type_name: String,
}

// ---------------------------------------------------------------------------
// Views and Viewpoints (SysML v2 Clause 8.2.2.26)
// ---------------------------------------------------------------------------

/// View definition: `view def` Identification ViewDefinitionBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: ViewDefBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ViewDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ViewDefBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ViewDefBodyElement {
    Error(Node<ParseErrorNode>),
    /// Unmodeled view-definition body element captured as raw text (used for library parsing).
    Other(String),
    Doc(Node<DocComment>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    Filter(Node<FilterMember>),
    ViewRendering(Node<ViewRenderingUsage>),
}

/// View rendering usage: `render` name `:` type (`;` or body).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewRenderingUsage {
    pub name: String,
    pub type_name: Option<String>,
    pub body: ConnectBody,
}

/// Viewpoint definition: `viewpoint def` Identification RequirementBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewpointDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: RequirementDefBody,
}

/// Rendering definition: `rendering def` Definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderingDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: RenderingDefBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenderingDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<RenderingDefBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenderingDefBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    Filter(Node<FilterMember>),
    ViewRendering(Node<ViewRenderingUsage>),
    Other(String),
}

/// View usage: `view` name `:` type? ViewBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewUsage {
    pub name: String,
    pub type_name: Option<String>,
    pub body: ViewBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ViewBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ViewBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ViewBodyElement {
    Error(Node<ParseErrorNode>),
    /// Unmodeled view body element captured as raw text (used for library parsing).
    Other(String),
    Doc(Node<DocComment>),
    Filter(Node<FilterMember>),
    ViewRendering(Node<ViewRenderingUsage>),
    Expose(Node<ExposeMember>),
    Satisfy(Node<SatisfyViewMember>),
}

/// Expose in view body: `expose` (MembershipImport | NamespaceImport) RelationshipBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExposeMember {
    /// Full target path (e.g. vehicle, vehicle::*, vehicle::*::**, SystemModel::vehicle::**).
    pub target: String,
    pub body: ConnectBody,
}

/// Satisfy in view body: `satisfy` QualifiedName RelationshipBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SatisfyViewMember {
    pub viewpoint_ref: String,
    pub body: ConnectBody,
}

/// Viewpoint usage: `viewpoint` ConstraintUsageDeclaration RequirementBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewpointUsage {
    pub name: String,
    pub type_name: String,
    pub body: RequirementDefBody,
}

/// Rendering usage: `rendering` Usage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderingUsage {
    pub name: String,
    pub type_name: Option<String>,
    pub body: ConnectBody,
}
