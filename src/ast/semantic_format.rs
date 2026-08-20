//! Semantic S-expression formatting for parsed documents.
//!
//! This module owns the projection policy used by semantic AST snapshots. Keeping the traversal
//! beside the AST makes changes to AST enums fail at compile time: every enum match below is
//! exhaustive and intentionally contains no catch-all arm.
//! The projection is a review/debugging format, not a versioned serialization format.

use std::collections::HashMap;
use std::io;
use std::io::Write as _;

use super::{
    ActionDefBodyElement, Argument, CaseReturnFeatureKind, CollectionOperator, ConnectionDefBody,
    ConnectionDefBodyElement, DerivationConnectionRole, DerivationEndRole, EndIdentity, Expression,
    FeatureValue, FeatureValueKind, FirstMergeBody, FirstMergeBodyElement, ImportShape,
    ImportSuffixSpans, ImportTarget, InOut, InterfaceDefBody, InterfaceDefBodyElement, Node,
    PackageBody, PackageBodyElement, ParsedDocument, PartDefBody, PartDefBodyElement, PerformBody,
    PerformBodyElement, PortDefBody, PortDefBodyElement, QualifiedReferenceId, ReferenceSeparator,
    RequirementDefBody, RequirementDefBodyElement, RootElement, Span, StateDefBody,
    StateDefBodyElement, SubsettingKind, SubsettingRelationship, TypeCheckKind, TypingKind,
    TypingRelationship, UseCaseDefBody, UseCaseDefBodyElement, ViewBody, ViewBodyElement,
    Visibility,
};

/// Stream a semantic AST projection to an [`io::Write`] sink.
///
/// The projection is intentionally about language-level roles rather than Rust field paths. A
/// caller can write directly to a file or use a `Vec<u8>`/`Cursor<Vec<u8>>` memory destination.
/// Qualified references are assigned document-local `rN` labels in semantic traversal order and
/// described structurally by scope, ordered tokens, decoded names, separators, and spans.
pub trait WriteSemanticAst {
    /// Write the canonical semantic S-expression.
    ///
    /// Returns an I/O error if the destination rejects a write or if the AST contains a dangling
    /// qualified-reference identity.
    fn write_semantic_ast<W: io::Write + ?Sized>(&self, writer: &mut W) -> io::Result<()>;
}

impl WriteSemanticAst for ParsedDocument {
    fn write_semantic_ast<W: io::Write + ?Sized>(&self, writer: &mut W) -> io::Result<()> {
        self.write_semantic_ast_with_span_policy(writer, true)
    }
}

impl ParsedDocument {
    /// Write the exhaustive semantic projection with source coordinates normalized away.
    ///
    /// This is the cross-document comparison boundary for equivalent parses of different source
    /// text. Reference IDs are resolved through each document's arena, so callers never compare
    /// document-local IDs.
    pub fn write_semantic_ast_for_comparison<W: io::Write + ?Sized>(
        &self,
        writer: &mut W,
    ) -> io::Result<()> {
        self.write_semantic_ast_with_span_policy(writer, false)
    }

    fn write_semantic_ast_with_span_policy<W: io::Write + ?Sized>(
        &self,
        writer: &mut W,
        include_source_spans: bool,
    ) -> io::Result<()> {
        let mut output = SemanticOutput {
            writer,
            include_source_spans,
        };
        // References precede the root in the wire view. Discover their stable semantic order with
        // the same streaming traversal, using a sink rather than allocating an intermediate tree.
        let mut labels = ReferenceLabels::default();
        {
            let mut sink = Sink;
            let mut sink_output = SemanticOutput {
                writer: &mut sink,
                include_source_spans,
            };
            SemanticWriter::new(self, &mut sink_output, &mut labels).write_root()?;
        }

        output.write_str("(parsed-document\n  (references")?;
        for (index, id) in labels.in_order.iter().copied().enumerate() {
            write_reference_definition(self, &mut output, index, id)?;
        }
        output.write_str("\n  )\n  ")?;
        SemanticWriter::new(self, &mut output, &mut labels).write_root()?;
        output.write_str("\n)")
    }
}

struct SemanticOutput<'writer, W: io::Write + ?Sized> {
    writer: &'writer mut W,
    include_source_spans: bool,
}

impl<W: io::Write + ?Sized> io::Write for SemanticOutput<'_, W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.writer.write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }

    fn write_all(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.writer.write_all(bytes)
    }
}

struct Sink;

impl io::Write for Sink {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

trait WriteText: io::Write {
    fn write_str(&mut self, text: &str) -> io::Result<()> {
        self.write_all(text.as_bytes())
    }

    fn write_char(&mut self, character: char) -> io::Result<()> {
        let mut bytes = [0; 4];
        self.write_all(character.encode_utf8(&mut bytes).as_bytes())
    }
}

impl<W: io::Write + ?Sized> WriteText for W {}

#[derive(Default)]
struct ReferenceLabels {
    by_id: HashMap<QualifiedReferenceId, usize>,
    in_order: Vec<QualifiedReferenceId>,
}

impl ReferenceLabels {
    fn label(&mut self, reference: QualifiedReferenceId) -> usize {
        if let Some(index) = self.by_id.get(&reference) {
            return *index;
        }
        let index = self.in_order.len();
        self.by_id.insert(reference, index);
        self.in_order.push(reference);
        index
    }
}

struct SemanticWriter<'document, 'labels, 'output, 'writer, W: io::Write + ?Sized> {
    document: &'document ParsedDocument,
    labels: &'labels mut ReferenceLabels,
    writer: &'output mut SemanticOutput<'writer, W>,
}

impl<'document, 'labels, 'output, 'writer, W: io::Write + ?Sized>
    SemanticWriter<'document, 'labels, 'output, 'writer, W>
{
    fn new(
        document: &'document ParsedDocument,
        writer: &'output mut SemanticOutput<'writer, W>,
        labels: &'labels mut ReferenceLabels,
    ) -> Self {
        Self {
            document,
            labels,
            writer,
        }
    }

    fn write_reference(&mut self, reference: QualifiedReferenceId) -> io::Result<()> {
        let index = self.labels.label(reference);
        write!(self.writer, "(ref r{index})")
    }

    /// Opens a brace body's projection and returns the `first` state its members expect.
    ///
    /// `(body brace)` with no members is an element whose body was authored `{}`; `(body
    /// semicolon)` is an element that owns no body at all. They are different authored syntax, so
    /// the projection names the form rather than leaving the difference to a trailing space --
    /// an empty brace body used to read `(body )`, which is indistinguishable from a typo.
    fn open_brace_body(&mut self) -> io::Result<bool> {
        self.writer.write_str("(body brace")?;
        // Members are appended through `write_item_prefix`, which emits a separator for every
        // item after the first. `false` makes it emit one for the first too, because the keyword
        // above ends without a space.
        Ok(false)
    }

    fn write_item_prefix(&mut self, first: &mut bool) -> io::Result<()> {
        if *first {
            *first = false;
            Ok(())
        } else {
            self.writer.write_char(' ')
        }
    }

    /// `ItemUsage`, projected with the shared `OccurrenceUsagePrefix` it now carries.
    ///
    /// Was a contentless `(item-usage)` marker in every scope, so a snapshot could not tell
    /// `ref individual item i;` from a bare `item i;`.
    fn write_item_usage(&mut self, usage: &super::ItemUsage) -> io::Result<()> {
        self.writer.write_str("(item-usage ")?;
        self.write_occurrence_usage_prefix(&usage.prefix)?;
        self.writer.write_str(" (declaration ")?;
        write_quoted(self.writer, &usage.name)?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, usage.short_name.as_deref())?;
        self.writer.write_str(") (type ")?;
        match usage.type_name {
            Some(reference) => self.write_reference(reference)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (multiplicity ")?;
        self.write_multiplicity_clause(usage.multiplicity.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_multiplicity_modifiers(&usage.multiplicity_modifiers)?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("subsets", usage.subsets.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("redefines", usage.redefines.as_ref())?;
        self.writer.write_str(" (value ")?;
        match &usage.value {
            Some(value) => self.write_feature_value(&value.value)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") ")?;
        self.write_attribute_body(&usage.body)?;
        self.writer.write_char(')')
    }

    fn write_item_usage_member(
        &mut self,
        first: &mut bool,
        usage: &super::ItemUsage,
    ) -> io::Result<()> {
        self.write_item_prefix(first)?;
        self.write_item_usage(usage)
    }

    fn write_marker(&mut self, first: &mut bool, kind: &str) -> io::Result<()> {
        self.write_item_prefix(first)?;
        write!(self.writer, "({kind})")
    }

    fn write_expression(&mut self, expression: &Node<Expression>) -> io::Result<()> {
        self.writer.write_str("(expression ")?;
        write_span(self.writer, &expression.span)?;
        self.writer.write_char(' ')?;
        match &expression.value {
            Expression::LiteralInteger(value) => write!(self.writer, "(integer {value})"),
            Expression::LiteralReal(value) => {
                self.writer.write_str("(real ")?;
                write_quoted(self.writer, value)?;
                self.writer.write_char(')')
            }
            Expression::LiteralString(value) => {
                self.writer.write_str("(string ")?;
                write_quoted(self.writer, value)?;
                self.writer.write_char(')')
            }
            Expression::LiteralBoolean(value) => write!(self.writer, "(boolean {value})"),
            Expression::FeatureRef(reference) | Expression::FeatureChainRef(reference) => {
                self.write_reference(*reference)
            }
            Expression::MemberAccess {
                base,
                member,
                separator,
            } => {
                self.writer.write_str("(member-access (base ")?;
                self.write_expression(base)?;
                self.writer.write_str(") (separator ")?;
                self.writer.write_str(separator_name(*separator))?;
                self.writer.write_str(") (member ")?;
                self.write_reference(*member)?;
                self.writer.write_str("))")
            }
            Expression::Index { base, operands, .. } => {
                self.writer.write_str("(index (base ")?;
                self.write_expression(base)?;
                self.writer.write_str(") (operands ")?;
                self.write_sequence_expression_list(operands)?;
                self.writer.write_str("))")
            }
            Expression::Bracket { base, operands, .. } => {
                self.writer.write_str("(bracket (base ")?;
                self.write_expression(base)?;
                self.writer.write_str(") (operands ")?;
                self.write_sequence_expression_list(operands)?;
                self.writer.write_str("))")
            }
            Expression::BinaryOp { op, left, right } => {
                self.writer.write_str("(binary (operator ")?;
                write_quoted(self.writer, op.as_str())?;
                self.writer.write_str(") (left ")?;
                self.write_expression(left)?;
                self.writer.write_str(") (right ")?;
                self.write_expression(right)?;
                self.writer.write_str("))")
            }
            Expression::UnaryOp { op, operand } => {
                self.writer.write_str("(unary (operator ")?;
                write_quoted(self.writer, op.as_str())?;
                self.writer.write_str(") (operand ")?;
                self.write_expression(operand)?;
                self.writer.write_str("))")
            }
            Expression::Invocation { callee, args } => {
                self.writer.write_str("(invocation (callee ")?;
                self.write_expression(callee)?;
                self.writer.write_str(") (arguments")?;
                self.write_arguments(args)?;
                self.writer.write_str("))")
            }
            Expression::Sequence { operands, .. } => {
                self.writer.write_str("(sequence ")?;
                self.write_sequence_expression_list(operands)?;
                self.writer.write_char(')')
            }
            Expression::Classification { metaclass } => {
                self.writer.write_str("(classification (metaclass ")?;
                self.write_reference(*metaclass)?;
                self.writer.write_str("))")
            }
            Expression::MetaCast { base, metaclass } => {
                self.writer.write_str("(meta-cast (base ")?;
                self.write_expression(base)?;
                self.writer.write_str(") (metaclass ")?;
                self.write_reference(*metaclass)?;
                self.writer.write_str("))")
            }
            Expression::TypeCheck {
                kind,
                operand,
                type_name,
            } => {
                self.writer.write_str("(type-check (kind ")?;
                self.writer.write_str(type_check_name(kind))?;
                self.writer.write_str(") (operand ")?;
                if let Some(operand) = operand {
                    self.write_expression(operand)?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str(") (type ")?;
                self.write_reference(*type_name)?;
                self.writer.write_str("))")
            }
            Expression::Select { base, selector } => {
                self.writer.write_str("(select (base ")?;
                self.write_expression(base)?;
                self.writer.write_str(") (selector ")?;
                self.write_reference(*selector)?;
                self.writer.write_str("))")
            }
            Expression::Collect { base, selector } => {
                self.writer.write_str("(collect (base ")?;
                self.write_expression(base)?;
                self.writer.write_str(") (selector ")?;
                self.write_reference(*selector)?;
                self.writer.write_str("))")
            }
            Expression::Null => self.writer.write_str("(null)"),
            Expression::Constructor { type_name, args } => {
                self.writer.write_str("(constructor (type ")?;
                self.write_reference(*type_name)?;
                self.writer.write_str(") (arguments")?;
                self.write_arguments(args)?;
                self.writer.write_str("))")
            }
            Expression::CollectionOp {
                op,
                base,
                args,
                brace_body,
                dot_shorthand: _,
            } => {
                self.writer.write_str("(collection-op (operator ")?;
                write_quoted(self.writer, collection_operator_name(op))?;
                self.writer.write_str(") (base ")?;
                self.write_expression(base)?;
                self.writer.write_str(") (arguments")?;
                self.write_arguments(args)?;
                self.writer.write_str(") (brace-body ")?;
                if let Some(body) = brace_body {
                    self.write_collection_operator_body(body)?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str("))")
            }
            Expression::MetadataAccess(base) => {
                self.writer.write_str("(metadata-access ")?;
                self.write_expression(base)?;
                self.writer.write_char(')')
            }
            Expression::Conditional {
                test,
                then_expr,
                else_expr,
            } => {
                self.writer.write_str("(conditional (test ")?;
                self.write_expression(test)?;
                self.writer.write_str(") (then ")?;
                self.write_expression(then_expr)?;
                self.writer.write_str(") (else ")?;
                self.write_expression(else_expr)?;
                self.writer.write_str("))")
            }
            Expression::Extent { target } => {
                self.writer.write_str("(extent (target ")?;
                self.write_reference(*target)?;
                self.writer.write_str("))")
            }
            Expression::BodyExpr(body) => {
                self.writer.write_str("(body-expr ")?;
                self.write_collection_operator_body(body)?;
                self.writer.write_char(')')
            }
        }?;
        self.writer.write_char(')')
    }

    fn write_collection_operator_body(
        &mut self,
        body: &Node<super::CollectionOperatorBody>,
    ) -> io::Result<()> {
        self.writer.write_str("(body ")?;
        write_span(self.writer, &body.span)?;
        self.writer.write_str(" (open-brace ")?;
        write_span(self.writer, &body.value.open_brace_span)?;
        self.writer.write_str(") (parameters")?;
        for parameter in &body.value.parameters {
            self.writer.write_str(" (parameter ")?;
            write_span(self.writer, &parameter.span)?;
            self.writer.write_str(" (direction ")?;
            if let Some(direction) = &parameter.value.direction {
                self.writer.write_str(match direction.value {
                    InOut::In => "in",
                    InOut::Out => "out",
                    InOut::InOut => "inout",
                })?;
                self.writer.write_char(' ')?;
                write_span(self.writer, &direction.span)?;
            } else {
                self.writer.write_str("none")?;
            }
            self.writer.write_str(") (reference-keyword ")?;
            if let Some(span) = &parameter.value.reference_keyword_span {
                write_span(self.writer, span)?;
            } else {
                self.writer.write_str("none")?;
            }
            self.writer.write_str(") (name ")?;
            write_quoted(self.writer, &parameter.value.name)?;
            self.writer.write_char(' ')?;
            write_span(self.writer, &parameter.value.name_span)?;
            self.writer.write_str(") (typing ")?;
            if let Some(typing) = &parameter.value.typing {
                self.writer.write_str("(typed (separator ")?;
                write_span(self.writer, &typing.separator_span)?;
                self.writer.write_str(") (target ")?;
                self.write_reference(typing.target)?;
                self.writer.write_str("))")?;
            } else {
                self.writer.write_str("none")?;
            }
            self.writer.write_str(") (terminator ")?;
            match &parameter.value.terminator {
                crate::ast::CollectionOperatorParameterTerminator::Semicolon { span } => {
                    self.writer.write_str("(semicolon ")?;
                    write_span(self.writer, span)?;
                    self.writer.write_str(")")?;
                }
                crate::ast::CollectionOperatorParameterTerminator::Body {
                    open_brace_span,
                    doc,
                    close_brace_span,
                } => {
                    self.writer.write_str("(body (open-brace ")?;
                    write_span(self.writer, open_brace_span)?;
                    self.writer.write_str(") (doc ")?;
                    self.writer
                        .write_str(if doc.is_some() { "present" } else { "none" })?;
                    self.writer.write_str(") (close-brace ")?;
                    write_span(self.writer, close_brace_span)?;
                    self.writer.write_str("))")?;
                }
            }
            self.writer.write_str("))")?;
        }
        self.writer.write_str(") (result ")?;
        if let Some(result) = &body.value.result {
            self.write_expression(result)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") (close-brace ")?;
        write_span(self.writer, &body.value.close_brace_span)?;
        self.writer.write_str("))")
    }

    fn write_sequence_expression_list(
        &mut self,
        operands: &Node<crate::ast::SequenceExpressionList>,
    ) -> io::Result<()> {
        self.writer.write_str("(sequence-list")?;
        for element in &operands.value.elements {
            self.writer.write_str(" (element ")?;
            if element.comma_before.is_some() {
                self.writer.write_str("comma ")?;
            } else {
                self.writer.write_str("first ")?;
            }
            self.write_expression(&element.expression)?;
            self.writer.write_char(')')?;
        }
        if operands.value.trailing_comma_span.is_some() {
            self.writer.write_str(" (trailing-comma)")?;
        }
        self.writer.write_char(')')
    }

    fn write_arguments(&mut self, args: &[Argument]) -> io::Result<()> {
        for argument in args {
            self.writer.write_str(" (argument (parameter ")?;
            if let Some(reference) = argument.parameter {
                self.write_reference(reference)?;
            } else {
                self.writer.write_str("none")?;
            }
            self.writer.write_str(") (value ")?;
            self.write_expression(&argument.value)?;
            self.writer.write_str("))")?;
        }
        Ok(())
    }

    fn write_import_target(&mut self, target: &ImportTarget) -> io::Result<()> {
        self.writer.write_str("(target (span ")?;
        write_span(self.writer, &target.span)?;
        self.writer.write_str(") (all ")?;
        if let Some(span) = &target.all_span {
            write_span(self.writer, span)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_reference(target.reference)?;
        self.writer.write_str(" (shape ")?;
        match &target.shape {
            ImportShape::Membership { recursive_suffix } => {
                self.writer.write_str("(membership ")?;
                self.write_optional_import_suffix("recursive-suffix", recursive_suffix.as_ref())?;
                self.writer.write_char(')')?;
            }
            ImportShape::Namespace {
                wildcard_suffix,
                recursive_suffix,
                combined_recursive_suffix_span,
            } => {
                self.writer.write_str("(namespace ")?;
                self.write_import_suffix("wildcard-suffix", wildcard_suffix)?;
                self.writer.write_char(' ')?;
                self.write_optional_import_suffix("recursive-suffix", recursive_suffix.as_ref())?;
                self.writer.write_str(" (combined-recursive-suffix-span ")?;
                if let Some(span) = combined_recursive_suffix_span {
                    write_span(self.writer, span)?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str("))")?;
            }
            ImportShape::Filter {
                recursive_suffix,
                members,
            } => {
                self.writer.write_str("(filter ")?;
                self.write_optional_import_suffix("recursive-suffix", recursive_suffix.as_ref())?;
                self.writer.write_str(" (members")?;
                for member in members {
                    self.writer.write_str(" (filter-member (span ")?;
                    write_span(self.writer, &member.span)?;
                    self.writer.write_str(") (open ")?;
                    write_span(self.writer, &member.value.open_bracket_span)?;
                    self.writer.write_str(") (expression ")?;
                    self.write_expression(&member.value.expression)?;
                    self.writer.write_str(") (close ")?;
                    write_span(self.writer, &member.value.close_bracket_span)?;
                    self.writer.write_str("))")?;
                }
                self.writer.write_str("))")?;
            }
        }
        self.writer.write_str("))")
    }

    fn write_import_suffix(&mut self, role: &str, suffix: &ImportSuffixSpans) -> io::Result<()> {
        write!(self.writer, "({role} (span ")?;
        write_span(self.writer, &suffix.span)?;
        self.writer.write_str(") (separator ")?;
        write_span(self.writer, &suffix.separator_span)?;
        self.writer.write_str(") (marker ")?;
        write_span(self.writer, &suffix.marker_span)?;
        self.writer.write_str("))")
    }

    fn write_optional_import_suffix(
        &mut self,
        role: &str,
        suffix: Option<&ImportSuffixSpans>,
    ) -> io::Result<()> {
        if let Some(suffix) = suffix {
            self.write_import_suffix(role, suffix)
        } else {
            write!(self.writer, "({role} none)")
        }
    }

    fn write_malformed(&mut self, error: &super::ParseErrorNode, span: &Span) -> io::Result<()> {
        self.writer.write_str("(malformed (code ")?;
        write_quoted(self.writer, &error.code)?;
        self.writer.write_str(") (found ")?;
        write_optional_quoted(self.writer, error.found.as_deref())?;
        self.writer.write_str(") ")?;
        write_span(self.writer, span)?;
        self.writer.write_char(')')
    }

    fn write_unsupported(
        &mut self,
        unsupported: &super::UnsupportedGrammarNode,
        span: &Span,
    ) -> io::Result<()> {
        self.writer.write_str("(unsupported (production ")?;
        self.writer.write_str(match unsupported.production {
            super::UnsupportedProduction::BindingConnectorAsUsage => "binding-connector-as-usage",
            super::UnsupportedProduction::Message => "message",
            super::UnsupportedProduction::SuccessionAsUsage => "succession-as-usage",
            super::UnsupportedProduction::PerformActionUsage => "perform-action-usage",
            super::UnsupportedProduction::ExhibitStateUsage => "exhibit-state-usage",
            super::UnsupportedProduction::IncludeUseCaseUsage => "include-use-case-usage",
            super::UnsupportedProduction::ReferenceConnectionUsage => "reference-connection-usage",
            super::UnsupportedProduction::ConnectionUsageInPartDefinition => {
                "connection-usage-in-part-definition"
            }
            super::UnsupportedProduction::ActionBodyMember => "action-body-member",
            super::UnsupportedProduction::UnmodelledBodyMember => "unmodelled-body-member",
        })?;
        self.writer.write_str(") (code ")?;
        write_quoted(self.writer, &unsupported.diagnostic.code)?;
        self.writer.write_str(") (found ")?;
        write_optional_quoted(self.writer, unsupported.diagnostic.found.as_deref())?;
        self.writer.write_str(") ")?;
        write_span(self.writer, span)?;
        self.writer.write_char(')')
    }

    /// One projection for `SatisfyRequirementUsage`, shared by every scope that owns one.
    ///
    /// This was a contentless `(satisfy)` marker in five scopes and a viewpoint-only
    /// `(satisfy (viewpoint …) (body …))` in view bodies, so a snapshot could show neither the
    /// `assert`/`not` prefixes, which of the two requirement alternatives was authored, whether a
    /// `by` clause existed, nor a single member of the requirement body.
    fn write_satisfy_requirement_usage(
        &mut self,
        usage: &super::SatisfyRequirementUsage,
    ) -> io::Result<()> {
        self.writer.write_str("(satisfy ")?;
        self.write_occurrence_usage_prefix(&usage.prefix)?;
        write!(
            self.writer,
            " (visibility {}) (assert {}) (negated {}) (requirement ",
            visibility_name(usage.membership.visibility),
            usage.assert_span.is_some(),
            usage.not_span.is_some()
        )?;
        match &usage.requirement {
            super::SatisfiedRequirement::Reference { reference } => {
                self.writer.write_str("(reference ")?;
                self.write_reference(*reference)?;
                self.writer.write_char(')')?;
            }
            super::SatisfiedRequirement::Declaration(declaration) => {
                self.writer.write_str("(declaration (name ")?;
                write_optional_quoted(
                    self.writer,
                    declaration.value.identification.name.as_deref(),
                )?;
                self.writer.write_str(") (short-name ")?;
                write_optional_quoted(
                    self.writer,
                    declaration.value.identification.short_name.as_deref(),
                )?;
                self.writer.write_str("))")?;
            }
        }
        self.writer.write_str(") (typing ")?;
        match &usage.typing {
            Some(typing) => self.write_typing(&typing.value)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (multiplicity ")?;
        self.write_multiplicity_clause(usage.multiplicity.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_multiplicity_modifiers(&usage.multiplicity_modifiers)?;
        for (role, clause) in [
            ("subsets", usage.subsets.as_ref()),
            ("references", usage.references.as_ref()),
            ("redefines", usage.redefines.as_ref()),
            ("crosses", usage.crosses.as_ref()),
        ] {
            write!(self.writer, " ({role} ")?;
            match clause {
                Some(clause) => self.write_subsetting(&clause.value)?,
                None => self.writer.write_str("none")?,
            }
            self.writer.write_char(')')?;
        }
        self.writer.write_str(" (value ")?;
        match &usage.value {
            Some(value) => self.write_feature_value(&value.value)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (by ")?;
        match &usage.subject {
            Some(subject) => self.write_reference(subject.value.reference)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") ")?;
        self.write_requirement_body(&usage.body)?;
        self.writer.write_char(')')
    }

    /// `MultiplicityPart`'s `OwnedMultiplicity`, or `none` when the clause was not authored.
    fn write_multiplicity_clause(
        &mut self,
        multiplicity: Option<&Node<super::Multiplicity>>,
    ) -> io::Result<()> {
        let Some(multiplicity) = multiplicity else {
            return self.writer.write_str("none");
        };
        self.writer.write_str("(lower ")?;
        match &multiplicity.value.lower {
            Some(lower) => self.write_expression(lower)?,
            None => self.writer.write_str("unbounded")?,
        }
        self.writer.write_str(") (upper ")?;
        match &multiplicity.value.upper {
            Some(upper) => self.write_expression(upper)?,
            None => self.writer.write_str("unbounded")?,
        }
        self.writer.write_char(')')
    }

    /// `MultiplicityPart`'s two keyword slots, each shown as the authored spelling or `none`.
    ///
    /// Distinguishing `none` from an authored default -- `unique`, `nonordered` -- is the whole
    /// point of the slots, so the projection names the spelling rather than the boolean the
    /// spelling implies.
    fn write_multiplicity_modifiers(
        &mut self,
        modifiers: &super::MultiplicityModifiers,
    ) -> io::Result<()> {
        self.writer
            .write_str("(multiplicity-modifiers (ordering ")?;
        match &modifiers.ordering {
            Some(ordering) => self.writer.write_str(ordering.value.keyword())?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (uniqueness ")?;
        match &modifiers.uniqueness {
            Some(uniqueness) => self.writer.write_str(uniqueness.value.keyword())?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str("))")
    }

    fn write_named_multiplicity(
        &mut self,
        kind: &str,
        name: &str,
        multiplicity: Option<&Node<super::Multiplicity>>,
    ) -> io::Result<()> {
        self.writer.write_char('(')?;
        self.writer.write_str(kind)?;
        self.writer.write_str(" (name ")?;
        if name.is_empty() {
            self.writer.write_str("none")?;
        } else {
            write_quoted(self.writer, name)?;
        }
        self.writer.write_str(") (multiplicity ")?;
        self.write_multiplicity_clause(multiplicity)?;
        self.writer.write_str("))")
    }

    fn write_requirement_body(&mut self, body: &RequirementDefBody) -> io::Result<()> {
        match body {
            RequirementDefBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            RequirementDefBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        RequirementDefBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        RequirementDefBodyElement::Satisfy(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_satisfy_requirement_usage(&usage.value)?;
                        }
                        RequirementDefBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        RequirementDefBodyElement::MetadataKeywordUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&usage.value)?;
                        }
                        RequirementDefBodyElement::Dependency(dependency) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_dependency(&dependency.value)?;
                        }
                        RequirementDefBodyElement::Import(import) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(import ")?;
                            self.write_import_target(&import.value.target)?;
                            self.writer.write_char(')')?;
                        }
                        RequirementDefBodyElement::SubjectDecl(subject) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_subject_decl(&subject.value)?;
                        }
                        RequirementDefBodyElement::SubjectRef(_subject) => {
                            self.write_marker(&mut first, "subject-ref")?;
                        }
                        RequirementDefBodyElement::RequirementActorDecl(actor) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(actor (name ")?;
                            write_quoted(self.writer, &actor.value.name)?;
                            self.writer.write_str(") (short-name ")?;
                            write_optional_quoted(self.writer, actor.value.short_name.as_deref())?;
                            self.writer.write_str(") (type ")?;
                            self.write_reference(actor.value.type_name)?;
                            self.writer.write_str(") (multiplicity ")?;
                            self.write_multiplicity_clause(actor.value.multiplicity.as_ref())?;
                            self.writer.write_str("))")?;
                        }
                        RequirementDefBodyElement::RequirementUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_named_multiplicity(
                                "requirement-usage",
                                &usage.value.name,
                                usage.value.multiplicity.as_ref(),
                            )?;
                        }
                        RequirementDefBodyElement::Stakeholder(stakeholder) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(stakeholder (declaration ")?;
                            write_quoted(self.writer, &stakeholder.value.declaration_name)?;
                            self.writer.write_str(") (target ")?;
                            if let Some(reference) = stakeholder.value.target {
                                self.write_reference(reference)?;
                            } else {
                                self.writer.write_str("none")?;
                            }
                            self.writer.write_str(") (type ")?;
                            if let Some(reference) = stakeholder.value.type_name {
                                self.write_reference(reference)?;
                            } else {
                                self.writer.write_str("none")?;
                            }
                            write!(
                                self.writer,
                                ") (redefinition {}))",
                                stakeholder.value.is_redefinition
                            )?;
                        }
                        RequirementDefBodyElement::Purpose(purpose) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(purpose (target ")?;
                            self.write_reference(purpose.value.target)?;
                            self.writer.write_str("))")?;
                        }
                        RequirementDefBodyElement::AttributeDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_definition(&definition.value)?;
                        }
                        RequirementDefBodyElement::AttributeUsage(_usage) => {
                            self.write_marker(&mut first, "attribute-usage")?;
                        }
                        RequirementDefBodyElement::VariantUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_variant_usage(&usage.value)?;
                        }
                        RequirementDefBodyElement::VerifyRequirement(verify) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(verify (target ")?;
                            if let Some(reference) = verify.value.target {
                                self.write_reference(reference)?;
                            } else {
                                self.writer.write_str("none")?;
                            }
                            self.writer.write_str(") (redefines ")?;
                            if let Some(reference) = verify.value.redefines {
                                self.write_reference(reference)?;
                            } else {
                                self.writer.write_str("none")?;
                            }
                            self.writer.write_str("))")?;
                        }
                        RequirementDefBodyElement::RequireConstraint(_constraint) => {
                            self.write_marker(&mut first, "require-constraint")?;
                        }
                        RequirementDefBodyElement::Constraint(_constraint) => {
                            self.write_marker(&mut first, "constraint")?;
                        }
                        RequirementDefBodyElement::Frame(frame) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(frame (concern-keyword ")?;
                            write!(self.writer, "{}", frame.value.has_concern_keyword)?;
                            self.writer.write_str(") (name ")?;
                            write_quoted(self.writer, &frame.value.name)?;
                            self.writer.write_str(") (short-name ")?;
                            write_optional_quoted(self.writer, frame.value.short_name.as_deref())?;
                            self.writer.write_str(") (type ")?;
                            if let Some(type_name) = frame.value.type_name {
                                self.write_reference(type_name)?;
                            } else {
                                self.writer.write_str("none")?;
                            }
                            self.writer.write_str(") ")?;
                            self.write_requirement_body(&frame.value.body)?;
                            self.writer.write_char(')')?;
                        }
                        RequirementDefBodyElement::RequirementDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_requirement_definition(&definition.value)?;
                        }
                        RequirementDefBodyElement::PortUsage(usage) => {
                            self.write_port_usage_member(&mut first, &usage.value)?;
                        }
                        RequirementDefBodyElement::AllocationUsage(_usage) => {
                            self.write_marker(&mut first, "allocation-usage")?;
                        }
                        RequirementDefBodyElement::ConcernUsage(_usage) => {
                            self.write_marker(&mut first, "concern-usage")?;
                        }
                        RequirementDefBodyElement::CalcUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_named_multiplicity(
                                "calc-usage",
                                usage.value.identification.name.as_deref().unwrap_or(""),
                                usage.value.multiplicity.as_ref(),
                            )?;
                        }
                        RequirementDefBodyElement::RefDecl(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                        // The general usage families `RequirementBodyItem` inherits from
                        // `DefinitionBodyItem`. Each is projected exactly as the sibling scopes
                        // that already own it project it, so a member reads the same wherever it
                        // is written.
                        RequirementDefBodyElement::ActionUsage(_usage) => {
                            self.write_marker(&mut first, "action-usage")?;
                        }
                        RequirementDefBodyElement::SuccessionUsage(_usage) => {
                            self.write_marker(&mut first, "succession-usage")?;
                        }
                        RequirementDefBodyElement::Perform(_perform) => {
                            self.write_marker(&mut first, "perform")?;
                        }
                        RequirementDefBodyElement::StateUsage(_usage) => {
                            self.write_marker(&mut first, "state-usage")?;
                        }
                        RequirementDefBodyElement::ItemUsage(usage) => {
                            self.write_item_usage_member(&mut first, &usage.value)?;
                        }
                        RequirementDefBodyElement::PartUsage(usage) => {
                            self.write_part_usage_member(&mut first, &usage.value)?;
                        }
                        RequirementDefBodyElement::Connect(_connect) => {
                            self.write_marker(&mut first, "connect")?;
                        }
                        RequirementDefBodyElement::ConnectionUsage(_usage) => {
                            self.write_marker(&mut first, "connection")?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_view_body(&mut self, body: &ViewBody) -> io::Result<()> {
        match body {
            ViewBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            ViewBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        ViewBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        ViewBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        ViewBodyElement::AliasDef(alias) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_alias_definition(&alias.value)?;
                        }
                        ViewBodyElement::RefDecl(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                        ViewBodyElement::Filter(_filter) => {
                            self.write_marker(&mut first, "filter")?;
                        }
                        ViewBodyElement::ViewRendering(_rendering) => {
                            self.write_marker(&mut first, "view-rendering")?;
                        }
                        ViewBodyElement::RenderingUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_rendering_usage(&usage.value)?;
                        }
                        ViewBodyElement::Expose(expose) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(expose ")?;
                            self.write_import_target(&expose.value.target)?;
                            self.writer.write_char(' ')?;
                            self.write_relationship_body(&expose.value.body)?;
                            self.writer.write_char(')')?;
                        }
                        ViewBodyElement::Satisfy(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_satisfy_requirement_usage(&usage.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_use_case_body(&mut self, body: &UseCaseDefBody) -> io::Result<()> {
        match body {
            UseCaseDefBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            UseCaseDefBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        UseCaseDefBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        UseCaseDefBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        UseCaseDefBodyElement::MetadataKeywordUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&usage.value)?;
                        }
                        UseCaseDefBodyElement::AttributeDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_definition(&definition.value)?;
                        }
                        UseCaseDefBodyElement::SubjectDecl(subject) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_subject_decl(&subject.value)?;
                        }
                        UseCaseDefBodyElement::SubjectRef(_subject) => {
                            self.write_marker(&mut first, "subject-ref")?;
                        }
                        UseCaseDefBodyElement::ActorUsage(actor) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(actor (name ")?;
                            write_quoted(self.writer, &actor.value.name)?;
                            self.writer.write_str(") (short-name ")?;
                            write_optional_quoted(self.writer, actor.value.short_name.as_deref())?;
                            self.writer.write_str(") (type ")?;
                            match actor.value.type_name {
                                Some(reference) => self.write_reference(reference)?,
                                None => self.writer.write_str("none")?,
                            }
                            self.writer.write_str(") (multiplicity ")?;
                            self.write_multiplicity_clause(actor.value.multiplicity.as_ref())?;
                            self.writer.write_str("))")?;
                        }
                        UseCaseDefBodyElement::ActorRedefinitionAssignment(actor) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(actor-redefinition (target ")?;
                            self.write_reference(actor.value.target)?;
                            self.writer.write_str(") (value ")?;
                            self.write_expression(&actor.value.value)?;
                            self.writer.write_str("))")?;
                        }
                        UseCaseDefBodyElement::Objective(_objective) => {
                            self.write_marker(&mut first, "objective")?;
                        }
                        UseCaseDefBodyElement::FirstSuccession(first_succession) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(first (target ")?;
                            self.write_reference(first_succession.value.target)?;
                            self.writer.write_str("))")?;
                        }
                        UseCaseDefBodyElement::ThenIncludeUseCase(include) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(then-include (target ")?;
                            self.write_reference(include.value.include.value.target)?;
                            self.writer.write_str("))")?;
                        }
                        UseCaseDefBodyElement::ThenUseCaseUsage(_usage) => {
                            self.write_marker(&mut first, "then-use-case")?;
                        }
                        UseCaseDefBodyElement::UseCaseUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_case_like_usage(
                                "use-case-usage",
                                &usage.value.name,
                                usage.value.is_abstract,
                                usage.value.type_name,
                                usage.value.subsets.as_ref(),
                            )?;
                        }
                        UseCaseDefBodyElement::CaseUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_case_like_usage(
                                "case-usage",
                                &usage.value.name,
                                usage.value.is_abstract,
                                usage.value.type_name,
                                usage.value.subsets.as_ref(),
                            )?;
                        }
                        UseCaseDefBodyElement::VerificationCaseUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_case_like_usage(
                                "verification-case-usage",
                                &usage.value.name,
                                usage.value.is_abstract,
                                usage.value.type_name,
                                usage.value.subsets.as_ref(),
                            )?;
                        }
                        UseCaseDefBodyElement::ThenDone(_done) => {
                            self.write_marker(&mut first, "then-done")?;
                        }
                        UseCaseDefBodyElement::IncludeUseCase(include) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(include (target ")?;
                            self.write_reference(include.value.target)?;
                            self.writer.write_str("))")?;
                        }
                        UseCaseDefBodyElement::Ref(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                        UseCaseDefBodyElement::InOutDecl(_declaration) => {
                            self.write_marker(&mut first, "in-out")?;
                        }
                        UseCaseDefBodyElement::RefRedefinition(reference) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(ref-redefinition (target ")?;
                            self.write_reference(reference.value.target)?;
                            self.writer.write_str(") (body-span ")?;
                            write_span(self.writer, &reference.value.body.span)?;
                            self.writer.write_str(") ")?;
                            self.write_use_case_body(&reference.value.body.value)?;
                            self.writer.write_char(')')?;
                        }
                        UseCaseDefBodyElement::AssertConstraint(_constraint) => {
                            self.write_marker(&mut first, "assert-constraint")?;
                        }
                        UseCaseDefBodyElement::ReturnRef(return_ref) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(return-ref (name ")?;
                            write_quoted(self.writer, &return_ref.value.name)?;
                            self.writer.write_str(") (body-span ")?;
                            write_span(self.writer, &return_ref.value.body.span)?;
                            self.writer.write_str(") ")?;
                            match &return_ref.value.body.value {
                                super::ReturnRefBody::Semicolon { .. } => {
                                    self.writer.write_str("(body semicolon)")?;
                                }
                                super::ReturnRefBody::Brace { elements, .. } => {
                                    self.writer.write_str("(body brace")?;
                                    for element in elements {
                                        self.writer.write_char(' ')?;
                                        match &element.value {
                                            super::ReturnRefBodyElement::Annotating(member) => {
                                                self.write_annotating_member(member)?;
                                            }
                                            super::ReturnRefBodyElement::Result(expression) => {
                                                self.writer.write_str("(result ")?;
                                                self.write_expression(expression)?;
                                                self.writer.write_char(')')?;
                                            }
                                            super::ReturnRefBodyElement::Error(error) => {
                                                self.write_malformed(&error.value, &element.span)?;
                                            }
                                        }
                                    }
                                    self.writer.write_char(')')?;
                                }
                            }
                            self.writer.write_char(')')?;
                        }
                        UseCaseDefBodyElement::CaseReturnDecl(return_decl) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_case_return(&return_decl.value)?;
                        }
                        UseCaseDefBodyElement::Assign(_assign) => {
                            self.write_marker(&mut first, "assign")?;
                        }
                        UseCaseDefBodyElement::ForLoop(for_loop) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_for_loop(&for_loop.value)?;
                        }
                        UseCaseDefBodyElement::ThenAction(action) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_then_action(&action.value)?;
                        }
                        UseCaseDefBodyElement::ActionUsage(_usage) => {
                            self.write_marker(&mut first, "action-usage")?;
                        }
                        UseCaseDefBodyElement::AnalysisCaseUsage(_usage) => {
                            self.write_marker(&mut first, "analysis-case-usage")?;
                        }
                        UseCaseDefBodyElement::CalcUsage(_usage) => {
                            self.write_marker(&mut first, "calc-usage")?;
                        }
                        UseCaseDefBodyElement::AttributeUsage(_usage) => {
                            self.write_marker(&mut first, "attribute-usage")?;
                        }
                        UseCaseDefBodyElement::RequirementUsage(_usage) => {
                            self.write_marker(&mut first, "requirement-usage")?;
                        }
                        UseCaseDefBodyElement::PartUsage(usage) => {
                            self.write_part_usage_member(&mut first, &usage.value)?;
                        }
                        UseCaseDefBodyElement::Expression(expression) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_expression(expression)?;
                        }
                        UseCaseDefBodyElement::FlowUsage(_usage) => {
                            self.write_marker(&mut first, "flow-usage")?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_state_body(&mut self, body: &StateDefBody) -> io::Result<()> {
        match body {
            StateDefBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            StateDefBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        StateDefBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        StateDefBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        StateDefBodyElement::MetadataKeywordUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&usage.value)?;
                        }
                        StateDefBodyElement::InOutDecl(_declaration) => {
                            self.write_marker(&mut first, "inout-declaration")?;
                        }
                        StateDefBodyElement::Entry(entry) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_state_behavior_action(
                                "entry",
                                entry.value.has_action_keyword,
                                entry.value.action_reference,
                                entry.value.declared_name.as_deref(),
                                entry.value.type_name,
                                entry.value.redefines.as_ref().map(|n| &n.value),
                                entry.value.effect.is_some(),
                                &entry.value.body,
                            )?;
                        }
                        StateDefBodyElement::Do(action) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_state_behavior_action(
                                "do",
                                action.value.has_action_keyword,
                                action.value.action_reference,
                                action.value.declared_name.as_deref(),
                                action.value.type_name,
                                action.value.redefines.as_ref().map(|n| &n.value),
                                action.value.effect.is_some(),
                                &action.value.body,
                            )?;
                        }
                        StateDefBodyElement::Exit(exit) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_state_behavior_action(
                                "exit",
                                exit.value.has_action_keyword,
                                exit.value.action_reference,
                                exit.value.declared_name.as_deref(),
                                exit.value.type_name,
                                exit.value.redefines.as_ref().map(|n| &n.value),
                                exit.value.effect.is_some(),
                                &exit.value.body,
                            )?;
                        }
                        StateDefBodyElement::Then(then) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(then (state ")?;
                            self.write_reference(then.value.state_reference)?;
                            self.writer.write_str("))")?;
                        }
                        StateDefBodyElement::FinalState(_state) => {
                            self.write_marker(&mut first, "final-state")?;
                        }
                        StateDefBodyElement::Ref(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                        StateDefBodyElement::RequirementUsage(_usage) => {
                            self.write_marker(&mut first, "requirement-usage")?;
                        }
                        StateDefBodyElement::StateUsage(_usage) => {
                            self.write_marker(&mut first, "state-usage")?;
                        }
                        StateDefBodyElement::Transition(transition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_transition(&transition.value)?;
                        }
                        StateDefBodyElement::AttributeUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_usage(&usage.value)?;
                        }
                        StateDefBodyElement::ActionUsage(_usage) => {
                            self.write_marker(&mut first, "action-usage")?;
                        }
                        StateDefBodyElement::SuccessionUsage(_usage) => {
                            self.write_marker(&mut first, "succession-usage")?;
                        }
                        StateDefBodyElement::AssertConstraint(_member) => {
                            self.write_marker(&mut first, "assert-constraint")?;
                        }
                        StateDefBodyElement::PartUsage(usage) => {
                            self.write_part_usage_member(&mut first, &usage.value)?;
                        }
                        StateDefBodyElement::ConstraintUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_constraint_usage(&usage.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn write_state_behavior_action(
        &mut self,
        kind: &str,
        has_action_keyword: bool,
        action_reference: Option<QualifiedReferenceId>,
        declared_name: Option<&str>,
        type_name: Option<QualifiedReferenceId>,
        redefines: Option<&SubsettingRelationship>,
        has_effect: bool,
        body: &StateDefBody,
    ) -> io::Result<()> {
        write!(
            self.writer,
            "({kind} (action-keyword {has_action_keyword}) (target "
        )?;
        if let Some(reference) = action_reference {
            self.write_reference(reference)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") (declared-name ")?;
        write_optional_quoted(self.writer, declared_name)?;
        self.writer.write_str(") (type ")?;
        match type_name {
            Some(reference) => self.write_reference(reference)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (redefines ")?;
        match redefines {
            Some(redefines) => self.write_subsetting(redefines)?,
            None => self.writer.write_str("none")?,
        }
        write!(self.writer, ") (effect {has_effect}) ")?;
        self.write_state_body(body)?;
        self.writer.write_char(')')
    }

    fn write_transition(&mut self, transition: &super::Transition) -> io::Result<()> {
        self.writer.write_str("(transition (name ")?;
        write_optional_quoted(self.writer, transition.name.as_deref())?;
        self.writer.write_str(") (source ")?;
        if let Some(source) = &transition.source {
            self.write_expression(source)?;
        } else {
            self.writer.write_str("none")?;
        }
        write!(
            self.writer,
            ") (initial {}) (accept ",
            transition.is_initial
        )?;
        match &transition.accept {
            Some(accept) => self.write_transition_accept(accept)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (guard ")?;
        if let Some(guard) = &transition.guard {
            self.write_expression(guard)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") (effect ")?;
        match &transition.effect {
            Some(effect) => self.write_transition_effect(effect)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (target ")?;
        self.write_expression(&transition.target)?;
        self.writer.write_str(") ")?;
        self.write_action_body(&transition.body)?;
        self.writer.write_char(')')
    }

    fn write_transition_accept(&mut self, accept: &super::TransitionAccept) -> io::Result<()> {
        match accept {
            super::TransitionAccept::Payload(payload, via) => {
                self.writer.write_str("(payload (name ")?;
                write_quoted(self.writer, &payload.name)?;
                self.writer.write_str(") (type ")?;
                match payload.type_name {
                    Some(reference) => self.write_reference(reference)?,
                    None => self.writer.write_str("none")?,
                }
                self.writer.write_str(") (via ")?;
                if let Some(via) = via {
                    self.write_expression(via)?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str("))")
            }
            super::TransitionAccept::Shorthand(expression, via) => {
                self.writer.write_str("(shorthand ")?;
                self.write_expression(expression)?;
                self.writer.write_str(" (via ")?;
                if let Some(via) = via {
                    self.write_expression(via)?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str("))")
            }
            super::TransitionAccept::TimeTrigger(kind, expression) => {
                self.writer.write_str("(time-trigger ")?;
                self.writer.write_str(match kind {
                    super::TriggerKind::At => "at",
                    super::TriggerKind::When => "when",
                    super::TriggerKind::After => "after",
                })?;
                self.writer.write_char(' ')?;
                self.write_expression(expression)?;
                self.writer.write_char(')')
            }
        }
    }

    fn write_transition_effect(&mut self, effect: &super::TransitionEffect) -> io::Result<()> {
        match effect {
            super::TransitionEffect::Perform {
                name,
                type_name,
                body,
            } => {
                self.writer.write_str("(perform (name ")?;
                write_optional_quoted(self.writer, name.as_deref())?;
                self.writer.write_str(") (type ")?;
                match type_name {
                    Some(reference) => self.write_reference(*reference)?,
                    None => self.writer.write_str("none")?,
                }
                self.writer.write_str(") (body ")?;
                self.write_optional_transition_effect_body(body)?;
                self.writer.write_str("))")
            }
            super::TransitionEffect::Accept {
                payload,
                type_name,
                via,
                body,
            } => {
                self.writer.write_str("(accept (payload ")?;
                self.write_expression(payload)?;
                self.writer.write_str(") (type ")?;
                match type_name {
                    Some(reference) => self.write_reference(*reference)?,
                    None => self.writer.write_str("none")?,
                }
                self.writer.write_str(") (via ")?;
                if let Some(via) = via {
                    self.write_expression(via)?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str(") (body ")?;
                self.write_optional_transition_effect_body(body)?;
                self.writer.write_str("))")
            }
            super::TransitionEffect::Send {
                payload,
                type_name,
                via,
                to,
                body,
            } => {
                self.writer.write_str("(send (payload ")?;
                self.write_expression(payload)?;
                self.writer.write_str(") (type ")?;
                match type_name {
                    Some(reference) => self.write_reference(*reference)?,
                    None => self.writer.write_str("none")?,
                }
                self.writer.write_str(") (via ")?;
                if let Some(via) = via {
                    self.write_expression(via)?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str(") (to ")?;
                if let Some(to) = to {
                    self.write_expression(to)?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str(") (body ")?;
                self.write_optional_transition_effect_body(body)?;
                self.writer.write_str("))")
            }
            super::TransitionEffect::Assign { lhs, rhs, body } => {
                self.writer.write_str("(assign (lhs ")?;
                self.write_expression(lhs)?;
                self.writer.write_str(") (rhs ")?;
                self.write_expression(rhs)?;
                self.writer.write_str(") (body ")?;
                self.write_optional_transition_effect_body(body)?;
                self.writer.write_str("))")
            }
            super::TransitionEffect::Expression(expression) => {
                self.writer.write_str("(expression ")?;
                self.write_expression(expression)?;
                self.writer.write_char(')')
            }
        }
    }

    fn write_optional_transition_effect_body(
        &mut self,
        body: &Option<super::ActionDefBody>,
    ) -> io::Result<()> {
        match body {
            Some(body) => self.write_action_body(body),
            None => self.writer.write_str("none"),
        }
    }

    fn write_part_body(&mut self, body: &PartDefBody) -> io::Result<()> {
        match body {
            PartDefBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            PartDefBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        PartDefBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        PartDefBodyElement::KermlClassifier(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_kerml_classifier(&declaration.value)?;
                        }
                        PartDefBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        PartDefBodyElement::Package(package) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_package(&package.value)?;
                        }
                        PartDefBodyElement::LibraryPackage(package) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_library_package(&package.value)?;
                        }
                        PartDefBodyElement::MetadataKeywordUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&usage.value)?;
                        }
                        PartDefBodyElement::Dependency(dependency) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_dependency(&dependency.value)?;
                        }
                        PartDefBodyElement::AttributeDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_definition(&definition.value)?;
                        }
                        PartDefBodyElement::AttributeUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_usage(&usage.value)?;
                        }
                        PartDefBodyElement::DefaultReferenceUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_default_reference_usage(&usage.value)?;
                        }
                        PartDefBodyElement::RequirementUsage(_usage) => {
                            self.write_marker(&mut first, "requirement-usage")?;
                        }
                        PartDefBodyElement::ItemDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_item_definition(&definition.value)?;
                        }
                        PartDefBodyElement::ItemUsage(usage) => {
                            self.write_item_usage_member(&mut first, &usage.value)?;
                        }
                        PartDefBodyElement::Ref(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                        PartDefBodyElement::PortUsage(usage) => {
                            self.write_port_usage_member(&mut first, &usage.value)?;
                        }
                        PartDefBodyElement::PartUsage(usage) => {
                            self.write_part_usage_member(&mut first, &usage.value)?;
                        }
                        PartDefBodyElement::PartDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_part_definition(&definition.value)?;
                        }
                        PartDefBodyElement::OccurrenceUsage(occurrence) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_occurrence(&occurrence.value)?;
                        }
                        PartDefBodyElement::InterfaceDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_interface_definition(&definition.value)?;
                        }
                        PartDefBodyElement::InterfaceUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_interface_usage(&usage.value)?;
                        }
                        PartDefBodyElement::Connect(connect) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_connect(&connect.value)?;
                        }
                        PartDefBodyElement::FlowUsage(_usage) => {
                            self.write_marker(&mut first, "flow-usage")?;
                        }
                        PartDefBodyElement::Connection(_connection) => {
                            self.write_marker(&mut first, "connection")?;
                        }
                        PartDefBodyElement::Perform(perform) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_perform(&perform.value)?;
                        }
                        PartDefBodyElement::Allocate(_allocate) => {
                            self.write_marker(&mut first, "allocate")?;
                        }
                        PartDefBodyElement::UnsupportedMember(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_unsupported(&member.value, &member.span)?;
                        }
                        PartDefBodyElement::ExhibitState(exhibit) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(exhibit (declaration ")?;
                            write_quoted(self.writer, &exhibit.value.name)?;
                            self.writer.write_str(") (state ")?;
                            if let Some(reference) = exhibit.value.state_reference {
                                self.write_reference(reference)?;
                            } else {
                                self.writer.write_str("none")?;
                            }
                            self.writer.write_str("))")?;
                        }
                        PartDefBodyElement::CalcUsage(_usage) => {
                            self.write_marker(&mut first, "calc-usage")?;
                        }
                        PartDefBodyElement::ConstraintDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_constraint_definition(&definition.value)?;
                        }
                        PartDefBodyElement::ConstraintUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_constraint_usage(&usage.value)?;
                        }
                        PartDefBodyElement::Import(import) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(import ")?;
                            self.write_import_target(&import.value.target)?;
                            self.writer.write_char(')')?;
                        }
                        PartDefBodyElement::ActionUsage(_usage) => {
                            self.write_marker(&mut first, "action-usage")?;
                        }
                        PartDefBodyElement::ActionDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_action_definition(&definition.value)?;
                        }
                        PartDefBodyElement::StateUsage(_usage) => {
                            self.write_marker(&mut first, "state-usage")?;
                        }
                        PartDefBodyElement::EnumerationUsage(_usage) => {
                            self.write_marker(&mut first, "enumeration-usage")?;
                        }
                        PartDefBodyElement::AssertConstraint(_constraint) => {
                            self.write_marker(&mut first, "assert-constraint")?;
                        }
                        PartDefBodyElement::Satisfy(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_satisfy_requirement_usage(&usage.value)?;
                        }
                        PartDefBodyElement::VariantUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_variant_usage(&usage.value)?;
                        }
                        PartDefBodyElement::StateDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_state_definition(&definition.value)?;
                        }
                        PartDefBodyElement::MetadataDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_definition(&definition.value)?;
                        }
                        PartDefBodyElement::MetadataUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_usage(&usage.value)?;
                        }
                        PartDefBodyElement::FlowDef(_definition) => {
                            self.write_marker(&mut first, "flow-def")?;
                        }
                        PartDefBodyElement::RequirementDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_requirement_definition(&definition.value)?;
                        }
                        PartDefBodyElement::OccurrenceDef(definition) => {
                            self.write_definition_prefix_marker(
                                &mut first,
                                "occurrence-def",
                                definition.value.definition_prefix.as_ref(),
                            )?;
                        }
                        PartDefBodyElement::ConnectionDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_connection_definition(&definition.value)?;
                        }
                        PartDefBodyElement::PortDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_port_definition(&definition.value)?;
                        }
                        PartDefBodyElement::CalcDef(_definition) => {
                            self.write_marker(&mut first, "calc-def")?;
                        }
                        PartDefBodyElement::EnumDef(_definition) => {
                            self.write_marker(&mut first, "enum-def")?;
                        }
                        PartDefBodyElement::AllocationDef(_definition) => {
                            self.write_marker(&mut first, "allocation-def")?;
                        }
                        PartDefBodyElement::AllocationUsage(_usage) => {
                            self.write_marker(&mut first, "allocation-usage")?;
                        }
                        PartDefBodyElement::ViewDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_view_definition(&definition.value)?;
                        }
                        PartDefBodyElement::ViewUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_view_usage(&usage.value)?;
                        }
                        PartDefBodyElement::ViewpointDef(definition) => {
                            self.write_definition_prefix_marker(
                                &mut first,
                                "viewpoint-def",
                                definition.value.definition_prefix.as_ref(),
                            )?;
                        }
                        PartDefBodyElement::ViewpointUsage(_usage) => {
                            self.write_marker(&mut first, "viewpoint-usage")?;
                        }
                        PartDefBodyElement::RenderingDef(definition) => {
                            self.write_definition_prefix_marker(
                                &mut first,
                                "rendering-def",
                                definition.value.definition_prefix.as_ref(),
                            )?;
                        }
                        PartDefBodyElement::RenderingUsage(_usage) => {
                            self.write_marker(&mut first, "rendering-usage")?;
                        }
                        PartDefBodyElement::CaseDef(definition) => {
                            self.write_definition_prefix_marker(
                                &mut first,
                                "case-def",
                                definition.value.definition_prefix.as_ref(),
                            )?;
                        }
                        PartDefBodyElement::CaseUsage(_usage) => {
                            self.write_marker(&mut first, "case-usage")?;
                        }
                        PartDefBodyElement::UseCaseDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_use_case_definition(&definition.value)?;
                        }
                        PartDefBodyElement::UseCaseUsage(_usage) => {
                            self.write_marker(&mut first, "use-case-usage")?;
                        }
                        PartDefBodyElement::AnalysisCaseDef(definition) => {
                            self.write_definition_prefix_marker(
                                &mut first,
                                "analysis-case-def",
                                definition.value.definition_prefix.as_ref(),
                            )?;
                        }
                        PartDefBodyElement::AnalysisCaseUsage(_usage) => {
                            self.write_marker(&mut first, "analysis-case-usage")?;
                        }
                        PartDefBodyElement::VerificationCaseDef(_definition) => {
                            self.write_marker(&mut first, "verification-case-def")?;
                        }
                        PartDefBodyElement::VerificationCaseUsage(_usage) => {
                            self.write_marker(&mut first, "verification-case-usage")?;
                        }
                        PartDefBodyElement::FirstStmt(statement) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_first_statement(&statement.value)?;
                        }
                        PartDefBodyElement::Bind(_bind) => {
                            self.write_marker(&mut first, "bind")?;
                        }
                        PartDefBodyElement::AliasDef(alias) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_alias_definition(&alias.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_dependency(&mut self, dependency: &super::Dependency) -> io::Result<()> {
        self.writer.write_str("(dependency (clients")?;
        for reference in &dependency.clients {
            self.writer.write_char(' ')?;
            self.write_reference(*reference)?;
        }
        self.writer.write_str(") (suppliers")?;
        for reference in &dependency.suppliers {
            self.writer.write_char(' ')?;
            self.write_reference(*reference)?;
        }
        self.writer.write_str(") ")?;
        self.write_relationship_body(&dependency.body)?;
        self.writer.write_char(')')
    }

    /// A `RelationshipBody`'s members.
    ///
    /// The body owns the whole annotating production, so its members are named rather than
    /// counted: that is the only thing a relationship body can get wrong, and `expose` and the
    /// view-body `satisfy` used to discard theirs entirely.
    fn write_relationship_body(
        &mut self,
        body: &super::Body<super::RelationshipBodyElement>,
    ) -> io::Result<()> {
        match body {
            super::Body::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            super::Body::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    self.write_item_prefix(&mut first)?;
                    match &element.value {
                        super::RelationshipBodyElement::Annotating(member) => {
                            self.write_annotating_member(member)?
                        }
                        super::RelationshipBodyElement::Error(error) => {
                            self.write_malformed(&error.value, &element.span)?
                        }
                        super::RelationshipBodyElement::KermlFeature(_) => {
                            self.writer.write_str("(kerml-feature)")?
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_first_statement(&mut self, statement: &super::FirstStmt) -> io::Result<()> {
        self.writer.write_str("(first (source ")?;
        self.write_expression(&statement.first)?;
        self.writer.write_str(") (target ")?;
        if let Some(target) = &statement.then {
            self.write_expression(target)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_first_merge_body(&statement.body)?;
        self.writer.write_char(')')
    }

    fn write_control_node_declaration(
        &mut self,
        declaration: &super::ControlNodeDeclaration,
    ) -> io::Result<()> {
        match declaration {
            super::ControlNodeDeclaration::Anonymous => self.writer.write_str("anonymous"),
            super::ControlNodeDeclaration::Named(name) => {
                self.writer.write_str("(named ")?;
                self.write_expression(name)?;
                self.writer.write_char(')')
            }
        }
    }

    fn write_control_node(
        &mut self,
        keyword: &str,
        declaration: &super::ControlNodeDeclaration,
        body: &FirstMergeBody,
    ) -> io::Result<()> {
        self.writer.write_char('(')?;
        self.writer.write_str(keyword)?;
        self.writer.write_str(" (declaration ")?;
        self.write_control_node_declaration(declaration)?;
        self.writer.write_str(") ")?;
        self.write_first_merge_body(body)?;
        self.writer.write_char(')')
    }

    fn write_then_action(&mut self, action: &super::ThenAction) -> io::Result<()> {
        match &action.target {
            super::ThenTarget::Merge(merge) => {
                self.writer.write_str("(then-control ")?;
                self.write_control_node("merge", &merge.value.declaration, &merge.value.body)?;
                self.writer.write_char(')')
            }
            super::ThenTarget::Fork(fork) => {
                self.writer.write_str("(then-control ")?;
                self.write_control_node("fork", &fork.value.declaration, &fork.value.body)?;
                self.writer.write_char(')')
            }
            super::ThenTarget::Decide(decision) => {
                self.writer.write_str("(then-control ")?;
                self.write_control_node(
                    "decide",
                    &decision.value.declaration,
                    &decision.value.body,
                )?;
                self.writer.write_char(')')
            }
            super::ThenTarget::Join(join) => {
                self.writer.write_str("(then-control ")?;
                self.write_control_node("join", &join.value.declaration, &join.value.body)?;
                self.writer.write_char(')')
            }
            // Retain the established compact projection for unrelated alternatives while matching
            // them explicitly: a new target variant must decide whether it is a control node.
            super::ThenTarget::Action(_)
            | super::ThenTarget::Perform(_)
            | super::ThenTarget::Accept(_)
            | super::ThenTarget::Send(_)
            | super::ThenTarget::Feature(_) => self.writer.write_str("(then-action)"),
        }
    }

    fn write_for_loop(&mut self, for_loop: &super::ForLoop) -> io::Result<()> {
        self.writer.write_str("(for-loop (variable ")?;
        write_quoted(self.writer, &for_loop.var)?;
        self.writer.write_str(") (range ")?;
        self.write_expression(&for_loop.range)?;
        self.writer.write_str(") ")?;
        self.write_action_body(&for_loop.body)?;
        self.writer.write_char(')')
    }

    fn write_extended_definition(
        &mut self,
        definition: &super::ExtendedDefinition,
    ) -> io::Result<()> {
        self.writer.write_str("(extended-def (prefix-keywords (")?;
        let mut first_keyword = true;
        for keyword in &definition.prefix_keywords {
            if !first_keyword {
                self.writer.write_char(' ')?;
            }
            first_keyword = false;
            self.write_reference(keyword.value.reference)?;
        }
        self.writer.write_str(")) (definition-prefix ")?;
        match definition
            .definition_prefix
            .as_ref()
            .map(|prefix| prefix.value)
        {
            Some(super::DefinitionPrefix::Abstract) => self.writer.write_str("abstract")?,
            Some(super::DefinitionPrefix::Variation) => self.writer.write_str("variation")?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (def ")?;
        self.writer.write_str(if definition.has_def_keyword {
            "true"
        } else {
            "false"
        })?;
        self.writer.write_str(") (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.writer.write_str(") (specializes ")?;
        if let Some(specializes) = &definition.specializes {
            self.write_typing(&specializes.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_package_body(&definition.body)?;
        self.writer.write_char(')')
    }

    fn write_action_definition(&mut self, definition: &super::ActionDef) -> io::Result<()> {
        self.writer.write_str("(action-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_definition_modifiers(definition.definition_prefix.as_ref())?;
        self.writer.write_str(") (specializes ")?;
        if let Some(specializes) = &definition.specializes {
            self.write_typing(&specializes.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_action_body(&definition.body)?;
        self.writer.write_char(')')
    }

    fn write_action_body(&mut self, body: &super::ActionDefBody) -> io::Result<()> {
        match body {
            super::ActionDefBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            super::ActionDefBody::Brace { elements, .. } => {
                self.writer.write_str("(body brace")?;
                for element in elements {
                    self.writer.write_char(' ')?;
                    self.write_first_merge_member(&element.value, &element.span)?;
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_first_merge_body(&mut self, body: &FirstMergeBody) -> io::Result<()> {
        match body {
            FirstMergeBody::Semicolon { semicolon_span } => {
                // The semicolon spelling had no span at all while this scope owned its own body
                // enum, so this projected a bare `(body semicolon)` and a consumer could not
                // locate the terminator. The shared `Body` keeps the token.
                self.writer.write_str("(body semicolon (span ")?;
                write_span(self.writer, semicolon_span)?;
                self.writer.write_str("))")
            }
            FirstMergeBody::Brace {
                open_span,
                elements,
                close_span,
            } => {
                // No `(span ...)` for the body as a whole: it was a second representation of the
                // extent the two delimiter spans already give, and it is gone with the `Node`
                // wrapper that carried it.
                self.writer.write_str("(body brace (open-brace ")?;
                write_span(self.writer, open_span)?;
                self.writer.write_str(") (members")?;
                for element in elements {
                    self.writer.write_char(' ')?;
                    match &element.value {
                        FirstMergeBodyElement::Member(member) => {
                            self.write_first_merge_member(&member.value, &member.span)?;
                        }
                        FirstMergeBodyElement::Unsupported(unsupported) => {
                            self.write_unsupported(&unsupported.value, &unsupported.span)?;
                        }
                        FirstMergeBodyElement::Error(error) => {
                            self.write_malformed(&error.value, &element.span)?;
                        }
                    }
                }
                self.writer.write_str(") (close-brace ")?;
                write_span(self.writer, close_span)?;
                self.writer.write_str("))")
            }
        }
    }

    fn write_calc_definition(&mut self, definition: &super::CalcDef) -> io::Result<()> {
        self.writer.write_str("(calc-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_definition_modifiers(definition.definition_prefix.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_calc_def_body(&definition.body)?;
        self.writer.write_char(')')
    }

    /// A calculation body's members.
    ///
    /// `CalculationBodyItem = ActionBodyItem | ReturnParameterMember`, and the action half used to
    /// be shredded into invented keyword-named features, so a marker for the definition as a whole
    /// could not show whether a member survived at all. Action members reuse the exhaustive
    /// `ActionDefBodyElement` writer rather than restating it.
    /// A KerML classifier declaration and its body.
    ///
    /// Spelled twice, in two different shapes, neither of which projected the body -- so every
    /// member of a `struct`/`classifier`/`datatype`/... was invisible, including a part usage,
    /// which this scope silently shredded into two bare expressions until it gained an arm.
    fn write_kerml_classifier(
        &mut self,
        declaration: &super::KermlClassifierDecl,
    ) -> io::Result<()> {
        self.writer.write_str("(kerml-classifier (keyword ")?;
        self.writer.write_str(declaration.keyword.as_str())?;
        write!(
            self.writer,
            ") (abstract {}) (name ",
            declaration.is_abstract
        )?;
        write_optional_quoted(self.writer, declaration.identification.name.as_deref())?;
        self.writer.write_str(") (specializes ")?;
        match &declaration.specializes {
            Some(typing) => self.write_typing(&typing.value)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") ")?;
        self.write_calc_def_body(&declaration.body)?;
        self.writer.write_char(')')
    }

    /// `ConstraintDefBody`, with its members.
    ///
    /// `ConstraintDefinition` and `ConstraintUsage` own a `CalculationBody`, and no scope that
    /// owns one was projected here at all -- so a `constraint def` was a leaf however much it
    /// declared. Exhaustive, so a new member of this scope is a compile error here.
    fn write_constraint_def_body(&mut self, body: &super::ConstraintDefBody) -> io::Result<()> {
        match body {
            super::ConstraintDefBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            super::ConstraintDefBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        super::ConstraintDefBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        super::ConstraintDefBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        super::ConstraintDefBodyElement::MetadataKeywordUsage(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&member.value)?;
                        }
                        super::ConstraintDefBodyElement::AttributeUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_usage(&usage.value)?;
                        }
                        super::ConstraintDefBodyElement::PartUsage(member) => {
                            self.write_part_usage_member(&mut first, &member.value)?;
                        }
                        super::ConstraintDefBodyElement::Expression(expression) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_expression(expression)?;
                        }
                        super::ConstraintDefBodyElement::InOutDecl(_member) => {
                            self.write_marker(&mut first, "in-out-declaration")?;
                        }
                        super::ConstraintDefBodyElement::Constraint(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_constraint_usage(&usage.value)?;
                        }
                        super::ConstraintDefBodyElement::FeatureDecl(_member) => {
                            self.write_marker(&mut first, "default-reference-usage")?;
                        }
                        super::ConstraintDefBodyElement::RequireConstraint(_member) => {
                            self.write_marker(&mut first, "require-constraint")?;
                        }
                        // Projected exactly as the calculation scope projects its own
                        // `ReturnParameterMember`; the two scopes share the node.
                        super::ConstraintDefBodyElement::ReturnDecl(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_return_declaration(&member.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    /// `ConstraintDefinition = … DefinitionDeclaration CalculationBody` (SysML BNF 1379).
    fn write_constraint_definition(&mut self, definition: &super::ConstraintDef) -> io::Result<()> {
        self.writer.write_str("(constraint-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_definition_modifiers(definition.definition_prefix.as_ref())?;
        self.writer.write_str(") (specializes ")?;
        match &definition.specializes {
            Some(typing) => self.write_typing(&typing.value)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") ")?;
        self.write_constraint_def_body(&definition.body)?;
        self.writer.write_char(')')
    }

    /// `ConstraintUsage = OccurrenceUsagePrefix 'constraint' ConstraintUsageDeclaration
    /// CalculationBody` (SysML BNF 1382).
    fn write_constraint_usage(&mut self, usage: &super::ConstraintUsage) -> io::Result<()> {
        self.writer.write_str("(constraint-usage ")?;
        self.write_occurrence_usage_prefix(&usage.prefix)?;
        self.writer.write_str(" (declaration-name ")?;
        self.write_usage_declaration_name(&usage.name)?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, usage.short_name.as_deref())?;
        self.writer.write_str(") (type ")?;
        match usage.type_name {
            Some(reference) => self.write_reference(reference)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (multiplicity ")?;
        self.write_multiplicity_clause(usage.multiplicity.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_optional_subsetting("subsets", usage.subsets.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("redefines", usage.redefines.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_constraint_def_body(&usage.body)?;
        self.writer.write_char(')')
    }

    /// `CalculationUsage = OccurrenceUsagePrefix 'calc' ActionUsageDeclaration CalculationBody`
    /// (SysML BNF 1354).
    ///
    /// `CalcUsage` has not migrated onto the shared `OccurrenceUsagePrefix` component yet, so the
    /// prefix slots it does carry are shown individually rather than through
    /// `write_occurrence_usage_prefix`.
    fn write_calculation_usage(&mut self, usage: &super::CalcUsage) -> io::Result<()> {
        self.writer.write_str("(calc-usage (name ")?;
        write_optional_quoted(self.writer, usage.identification.name.as_deref())?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, usage.identification.short_name.as_deref())?;
        self.writer.write_str(") (direction ")?;
        match usage.direction {
            Some(InOut::In) => self.writer.write_str("in")?,
            Some(InOut::Out) => self.writer.write_str("out")?,
            Some(InOut::InOut) => self.writer.write_str("inout")?,
            None => self.writer.write_str("none")?,
        }
        write!(
            self.writer,
            ") (abstract {}) (reference {}) (type ",
            usage.is_abstract, usage.is_reference
        )?;
        match usage.type_name {
            Some(reference) => self.write_reference(reference)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (multiplicity ")?;
        self.write_multiplicity_clause(usage.multiplicity.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_optional_subsetting("subsets", usage.subsets.as_ref())?;
        self.writer.write_str(" (redefines")?;
        match &usage.redefines {
            Some(targets) => {
                for target in targets {
                    self.writer.write_char(' ')?;
                    self.write_reference(*target)?;
                }
            }
            None => self.writer.write_str(" none")?,
        }
        self.writer.write_str(") (value ")?;
        match &usage.value {
            Some(value) => self.write_feature_value(&value.value)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") ")?;
        self.write_calc_def_body(&usage.body)?;
        self.writer.write_char(')')
    }

    fn write_calc_def_body(&mut self, body: &super::CalcDefBody) -> io::Result<()> {
        match body {
            super::CalcDefBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            super::CalcDefBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        super::CalcDefBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        super::CalcDefBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        super::CalcDefBodyElement::MetadataKeywordUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&usage.value)?;
                        }
                        super::CalcDefBodyElement::ActionMember(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_first_merge_member(&member.value, &member.span)?;
                        }
                        super::CalcDefBodyElement::InOutDecl(_declaration) => {
                            self.write_marker(&mut first, "in-out-declaration")?;
                        }
                        super::CalcDefBodyElement::KermlFeature(_member) => {
                            self.write_marker(&mut first, "kerml-feature")?;
                        }
                        super::CalcDefBodyElement::Invariant(_member) => {
                            self.write_marker(&mut first, "invariant")?;
                        }
                        super::CalcDefBodyElement::Connector(_member) => {
                            self.write_marker(&mut first, "connector")?;
                        }
                        super::CalcDefBodyElement::Binding(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_bind(&member.value)?;
                        }
                        super::CalcDefBodyElement::Succession(_member) => {
                            self.write_marker(&mut first, "succession")?;
                        }
                        super::CalcDefBodyElement::FlowUsage(_member) => {
                            self.write_marker(&mut first, "flow-usage")?;
                        }
                        super::CalcDefBodyElement::Import(_member) => {
                            self.write_marker(&mut first, "import")?;
                        }
                        super::CalcDefBodyElement::AttributeUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_usage(&usage.value)?;
                        }
                        super::CalcDefBodyElement::AssertConstraint(_member) => {
                            self.write_marker(&mut first, "assert-constraint")?;
                        }
                        super::CalcDefBodyElement::KermlClassifier(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_kerml_classifier(&declaration.value)?;
                        }
                        super::CalcDefBodyElement::DefaultReferenceUsage(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_default_reference_usage(&member.value)?;
                        }
                        super::CalcDefBodyElement::ReturnDecl(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_return_declaration(&member.value)?;
                        }
                        super::CalcDefBodyElement::Expression(expression) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(expression ")?;
                            self.write_expression(expression)?;
                            self.writer.write_char(')')?;
                        }
                        super::CalcDefBodyElement::CalcUsage(_member) => {
                            self.write_marker(&mut first, "calc-usage")?;
                        }
                        super::CalcDefBodyElement::CalcDef(_member) => {
                            self.write_marker(&mut first, "calc-def")?;
                        }
                        super::CalcDefBodyElement::PartUsage(member) => {
                            self.write_part_usage_member(&mut first, &member.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_first_merge_member(
        &mut self,
        member: &ActionDefBodyElement,
        span: &Span,
    ) -> io::Result<()> {
        match member {
            ActionDefBodyElement::Error(error) => self.write_malformed(&error.value, span),
            ActionDefBodyElement::InOutDecl(declaration) => {
                self.writer.write_str("(in-out (direction ")?;
                match declaration.value.direction {
                    InOut::In => self.writer.write_str("in")?,
                    InOut::Out => self.writer.write_str("out")?,
                    InOut::InOut => self.writer.write_str("inout")?,
                }
                self.writer.write_str(") (reference ")?;
                self.writer.write_str(if declaration.value.is_reference {
                    "true"
                } else {
                    "false"
                })?;
                self.writer.write_str(") (declaration ")?;
                self.write_usage_declaration_name(&declaration.value.name)?;
                self.writer.write_str(") (subsets ")?;
                match &declaration.value.subsets {
                    Some(subsets) => self.write_subsetting(&subsets.value)?,
                    None => self.writer.write_str("none")?,
                }
                self.writer.write_str(") (type ")?;
                if let Some(reference) = declaration.value.type_name {
                    self.write_reference(reference)?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str(") (multiplicity ")?;
                if let Some(multiplicity) = &declaration.value.multiplicity {
                    self.writer.write_str("(lower ")?;
                    if let Some(lower) = &multiplicity.value.lower {
                        self.write_expression(lower)?;
                    } else {
                        self.writer.write_str("unbounded")?;
                    }
                    self.writer.write_str(") (upper ")?;
                    if let Some(upper) = &multiplicity.value.upper {
                        self.write_expression(upper)?;
                    } else {
                        self.writer.write_str("unbounded")?;
                    }
                    self.writer.write_char(')')?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str(") ")?;
                self.write_multiplicity_modifiers(&declaration.value.multiplicity_modifiers)?;
                self.writer.write_char(' ')?;
                self.write_optional_subsetting("redefines", declaration.value.redefines.as_ref())?;
                self.writer.write_str(" (value ")?;
                if let Some(value) = &declaration.value.value {
                    self.write_feature_value(&value.value)?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str(")")?;
                if let Some(body) = &declaration.value.body {
                    // Like a dependency body this is a `Vec`, not an `ast::Body<E>`, so it has no
                    // marker of its own -- but a present one was written with braces, and every
                    // other body in this projection says so.
                    self.writer.write_str(" (body brace")?;
                    for element in body {
                        self.writer.write_char(' ')?;
                        self.write_first_merge_member(&element.value, &element.span)?;
                    }
                    self.writer.write_char(')')?;
                }
                self.writer.write_char(' ')?;
                write_span(self.writer, span)?;
                self.writer.write_char(')')
            }
            ActionDefBodyElement::Annotating(member) => self.write_annotating_member(member),
            ActionDefBodyElement::MetadataKeywordUsage(usage) => {
                self.write_metadata_keyword_usage(&usage.value)
            }
            ActionDefBodyElement::Dependency(dependency) => {
                self.write_dependency(&dependency.value)
            }
            ActionDefBodyElement::MetadataUsage(usage) => self.write_metadata_usage(&usage.value),
            ActionDefBodyElement::RefDecl(declaration) => {
                self.write_ref_declaration(&declaration.value)
            }
            ActionDefBodyElement::Perform(perform) => self.write_perform(&perform.value),
            ActionDefBodyElement::Bind(_bind) => self.writer.write_str("(bind)"),
            ActionDefBodyElement::FlowUsage(_flow) => self.writer.write_str("(flow-usage)"),
            ActionDefBodyElement::FirstStmt(first) => self.write_first_statement(&first.value),
            ActionDefBodyElement::MergeStmt(merge) => {
                self.write_control_node("merge", &merge.value.declaration, &merge.value.body)
            }
            ActionDefBodyElement::DecisionStmt(decision) => {
                self.write_control_node("decide", &decision.value.declaration, &decision.value.body)
            }
            ActionDefBodyElement::JoinStmt(join) => {
                self.write_control_node("join", &join.value.declaration, &join.value.body)
            }
            ActionDefBodyElement::ForkStmt(fork) => {
                self.write_control_node("fork", &fork.value.declaration, &fork.value.body)
            }
            ActionDefBodyElement::TerminateStmt(terminate) => {
                self.write_terminate_statement(&terminate.value)
            }
            ActionDefBodyElement::WhileStmt(_while) => self.writer.write_str("(while)"),
            ActionDefBodyElement::LoopStmt(_loop) => self.writer.write_str("(loop)"),
            ActionDefBodyElement::IfStmt(_if) => self.writer.write_str("(if)"),
            ActionDefBodyElement::StateUsage(_state) => self.writer.write_str("(state-usage)"),
            ActionDefBodyElement::ActionUsage(usage) => {
                self.writer.write_str("(action-usage (declaration ")?;
                self.write_usage_declaration_name(&usage.value.name)?;
                self.writer.write_str(") (type ")?;
                if let Some(reference) = usage.value.type_name {
                    self.write_reference(reference)?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str("))")
            }
            ActionDefBodyElement::PartUsage(part) => self.write_part_usage(&part.value),
            ActionDefBodyElement::ItemUsage(item) => self.write_item_usage(&item.value),
            ActionDefBodyElement::AssertConstraint(_constraint) => {
                self.writer.write_str("(assert-constraint)")
            }
            ActionDefBodyElement::OccurrenceUsage(occurrence) => {
                self.writer.write_str("(occurrence-usage ")?;
                self.write_occurrence_usage_prefix(&occurrence.value.prefix)?;
                self.writer.write_char(')')
            }
            ActionDefBodyElement::Assign(_assign) => self.writer.write_str("(assign)"),
            ActionDefBodyElement::ForLoop(for_loop) => self.write_for_loop(&for_loop.value),
            ActionDefBodyElement::ThenAction(action) => self.write_then_action(&action.value),
            ActionDefBodyElement::AttributeUsage(_usage) => {
                self.writer.write_str("(attribute-usage)")
            }
            ActionDefBodyElement::CalcUsage(_usage) => self.writer.write_str("(calc-usage)"),
            ActionDefBodyElement::ActionDef(_def) => self.writer.write_str("(action-def)"),
            ActionDefBodyElement::DefaultReferenceUsage(usage) => {
                self.write_default_reference_usage(&usage.value)
            }
        }
    }

    fn write_alias_definition(&mut self, definition: &super::AliasDef) -> io::Result<()> {
        self.writer.write_str("(alias (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.writer.write_str(") (target ")?;
        self.write_reference(definition.target)?;
        self.writer.write_str(") (body ")?;
        match &definition.body {
            super::AliasBody::Semicolon { .. } => self.writer.write_str("semicolon")?,
            super::AliasBody::Brace { elements, .. } => {
                write!(self.writer, "brace (element-count {})", elements.len())?;
            }
        }
        self.writer.write_str("))")
    }

    fn write_connection_definition(&mut self, definition: &super::ConnectionDef) -> io::Result<()> {
        self.writer.write_str("(connection-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_occurrence_definition_modifiers(
            definition.definition_prefix.as_ref(),
            definition.is_individual,
        )?;
        self.writer.write_str(") (role ")?;
        if let Some(role) = &definition.derivation_role {
            match role.value {
                DerivationConnectionRole::Derivation => {
                    self.writer.write_str("(derivation ")?;
                    write_span(self.writer, &role.span)?;
                    self.writer.write_char(')')?;
                }
            }
        } else {
            self.writer.write_str("ordinary")?;
        }
        self.writer.write_str(") (specializes ")?;
        if let Some(specializes) = &definition.specializes {
            self.write_typing(&specializes.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_connection_body(&definition.body)?;
        self.writer.write_char(')')
    }

    fn write_interface_definition(&mut self, definition: &super::InterfaceDef) -> io::Result<()> {
        self.writer.write_str("(interface-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_occurrence_definition_modifiers(
            definition.definition_prefix.as_ref(),
            definition.is_individual,
        )?;
        self.writer.write_str(") (specializes ")?;
        if let Some(specializes) = &definition.specializes {
            self.write_typing(&specializes.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_interface_body(&definition.body)?;
        self.writer.write_char(')')
    }

    /// The contents of a `(modifiers ...)` group for `BasicDefinitionPrefix` (SysML BNF 219).
    ///
    /// The authored span travels with the keyword: the whole point of the slot is that a consumer
    /// can tell `abstract` from `variation` from neither, *and* point at it.
    fn write_basic_definition_prefix_items(
        &mut self,
        prefix: Option<&Node<super::DefinitionPrefix>>,
    ) -> io::Result<()> {
        if let Some(prefix) = prefix {
            self.writer.write_str(" (")?;
            self.writer.write_str(match prefix.value {
                super::DefinitionPrefix::Abstract => "abstract",
                super::DefinitionPrefix::Variation => "variation",
            })?;
            self.writer.write_char(' ')?;
            write_span(self.writer, &prefix.span)?;
            self.writer.write_char(')')?;
        }
        Ok(())
    }

    /// Closes the preceding field and opens the `(modifiers ...)` group for a definition whose
    /// production reaches `BasicDefinitionPrefix` but not `OccurrenceDefinitionPrefix`'s
    /// `individual`. The caller closes the group.
    fn write_definition_modifiers(
        &mut self,
        prefix: Option<&Node<super::DefinitionPrefix>>,
    ) -> io::Result<()> {
        self.writer.write_str(") (modifiers")?;
        self.write_basic_definition_prefix_items(prefix)
    }

    fn write_occurrence_definition_modifiers(
        &mut self,
        prefix: Option<&Node<super::DefinitionPrefix>>,
        is_individual: bool,
    ) -> io::Result<()> {
        self.write_definition_modifiers(prefix)?;
        if is_individual {
            self.writer.write_str(" individual")?;
        }
        Ok(())
    }

    /// A definition kind whose projection is still a bare kind marker (its declaration and body
    /// are not projected yet), with its `BasicDefinitionPrefix` slot shown so that `abstract`,
    /// `variation` and neither are three distinguishable states rather than one marker.
    fn write_definition_prefix_marker(
        &mut self,
        first: &mut bool,
        kind: &str,
        prefix: Option<&Node<super::DefinitionPrefix>>,
    ) -> io::Result<()> {
        self.write_item_prefix(first)?;
        write!(self.writer, "({kind} (modifiers")?;
        self.write_basic_definition_prefix_items(prefix)?;
        self.writer.write_str("))")
    }

    fn write_flow_definition(&mut self, definition: &super::FlowDef) -> io::Result<()> {
        self.writer.write_str("(flow-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_occurrence_definition_modifiers(
            definition.definition_prefix.as_ref(),
            definition.is_individual,
        )?;
        self.writer.write_str("))")
    }

    fn write_allocation_definition(&mut self, definition: &super::AllocationDef) -> io::Result<()> {
        self.writer.write_str("(allocation-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_occurrence_definition_modifiers(
            definition.definition_prefix.as_ref(),
            definition.is_individual,
        )?;
        self.writer.write_str("))")
    }

    fn write_interface_body(&mut self, body: &InterfaceDefBody) -> io::Result<()> {
        match body {
            InterfaceDefBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            InterfaceDefBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        InterfaceDefBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        InterfaceDefBodyElement::MetadataKeywordUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&usage.value)?;
                        }
                        InterfaceDefBodyElement::EndDecl(end) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_end(&end.value)?;
                        }
                        InterfaceDefBodyElement::RefDecl(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                        InterfaceDefBodyElement::ConnectStmt(connect) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_connect_statement(&connect.value)?;
                        }
                        InterfaceDefBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        InterfaceDefBodyElement::AttributeDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_definition(&definition.value)?;
                        }
                        InterfaceDefBodyElement::AttributeUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_usage(&usage.value)?;
                        }
                        InterfaceDefBodyElement::ItemDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_item_definition(&definition.value)?;
                        }
                        InterfaceDefBodyElement::ItemUsage(usage) => {
                            self.write_item_usage_member(&mut first, &usage.value)?;
                        }
                        InterfaceDefBodyElement::PortDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_port_definition(&definition.value)?;
                        }
                        InterfaceDefBodyElement::PortUsage(usage) => {
                            self.write_port_usage_member(&mut first, &usage.value)?;
                        }
                        InterfaceDefBodyElement::FlowUsage(_usage) => {
                            self.write_marker(&mut first, "flow-usage")?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_connection_body(&mut self, body: &ConnectionDefBody) -> io::Result<()> {
        match body {
            ConnectionDefBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            ConnectionDefBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        ConnectionDefBodyElement::EndDecl(end) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_end(&end.value)?;
                        }
                        ConnectionDefBodyElement::RefDecl(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                        ConnectionDefBodyElement::ConnectStmt(connect) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_connect_statement(&connect.value)?;
                        }
                        ConnectionDefBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        ConnectionDefBodyElement::MetadataKeywordUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&usage.value)?;
                        }
                        ConnectionDefBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        ConnectionDefBodyElement::AttributeDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_definition(&definition.value)?;
                        }
                        ConnectionDefBodyElement::AttributeUsage(_usage) => {
                            self.write_marker(&mut first, "attribute-usage")?;
                        }
                        ConnectionDefBodyElement::ItemDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_item_definition(&definition.value)?;
                        }
                        ConnectionDefBodyElement::ItemUsage(usage) => {
                            self.write_item_usage_member(&mut first, &usage.value)?;
                        }
                        ConnectionDefBodyElement::PortDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_port_definition(&definition.value)?;
                        }
                        ConnectionDefBodyElement::PortUsage(usage) => {
                            self.write_port_usage_member(&mut first, &usage.value)?;
                        }
                        ConnectionDefBodyElement::AssertConstraint(_constraint) => {
                            self.write_marker(&mut first, "assert-constraint")?;
                        }
                        ConnectionDefBodyElement::OccurrenceUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_occurrence(&usage.value)?;
                        }
                        ConnectionDefBodyElement::SuccessionUsage(_usage) => {
                            self.write_marker(&mut first, "succession-usage")?;
                        }
                        ConnectionDefBodyElement::PartUsage(usage) => {
                            self.write_part_usage_member(&mut first, &usage.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    /// One projection for a `ref` declaration, shared by every scope that owns one.
    ///
    /// `UsageBody = DefinitionBody`, so a `ref` body holds the same members wherever it appears.
    /// Rendering it as a bare marker per owner would hide exactly the differences that matter --
    /// which members parsed, in what order, under which owner -- so every owner uses this.
    fn write_ref_declaration(&mut self, declaration: &super::RefDecl) -> io::Result<()> {
        self.writer.write_str("(ref (name ")?;
        write_quoted(self.writer, &declaration.name)?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, declaration.short_name.as_deref())?;
        // BNF `RefPrefix`, projected because these keywords are authored syntax the emitter
        // reproduces: without them a snapshot could not tell `derived ref item x` from `ref
        // item x`, which is precisely the distinction the fields were added to hold.
        self.writer.write_str(") (prefix (direction ")?;
        match declaration.direction {
            Some(InOut::In) => self.writer.write_str("in")?,
            Some(InOut::Out) => self.writer.write_str("out")?,
            Some(InOut::InOut) => self.writer.write_str("inout")?,
            None => self.writer.write_str("none")?,
        }
        write!(self.writer, ") (derived {})", declaration.is_derived)?;
        self.writer.write_str(" (usage-prefix ")?;
        match declaration.usage_prefix {
            Some(super::DefinitionPrefix::Abstract) => self.writer.write_str("abstract")?,
            Some(super::DefinitionPrefix::Variation) => self.writer.write_str("variation")?,
            None => self.writer.write_str("none")?,
        }
        write!(self.writer, ") (constant {}))", declaration.is_constant)?;
        self.writer.write_str(" (kind ")?;
        match declaration.kind_keyword {
            Some(kind) => self.writer.write_str(kind.as_str())?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (typing ")?;
        if let Some(typing) = &declaration.typing {
            self.write_typing(&typing.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_optional_subsetting("redefines", declaration.redefines.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("subsets", declaration.subsets.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_ref_body(&declaration.body)?;
        self.writer.write_char(')')
    }

    /// A part usage: its whole `OccurrenceUsagePrefix`, its declaration, and its body.
    ///
    /// `PartUsage = OccurrenceUsagePrefix 'part' Usage` (SysML BNF 623), so the projection names
    /// every part of that: the shared prefix through the same
    /// [`write_occurrence_usage_prefix`](Self::write_occurrence_usage_prefix) the other migrated
    /// families use, then `Identification`'s short name and declared name, then
    /// `FeatureSpecializationPart`'s typing, multiplicity and its `ordered`/`nonunique`
    /// modifiers, subsettings and redefinitions, then `UsageCompletion`'s value and body.
    ///
    /// It used to show only the name, the typing and the two multiplicity modifiers, and in ten
    /// of its thirteen owning scopes not even that -- a bare `(part-usage)` marker -- so a
    /// snapshot could not tell `ref individual snapshot part p[1..*] :> q = r;` from `part p;`.
    /// `PartUsageBody` and `RefBody` are both `Body<PartUsageBodyElement>`, so the exhaustive
    /// element match in [`write_ref_body`](Self::write_ref_body) covers the body members.
    fn write_part_usage(&mut self, usage: &super::PartUsage) -> io::Result<()> {
        write!(
            self.writer,
            "(part-usage (then {}) ",
            usage.then_span.is_some()
        )?;
        self.write_occurrence_usage_prefix(&usage.prefix)?;
        self.writer.write_str(" (declaration-name ")?;
        self.write_usage_declaration_name(&usage.name)?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, usage.short_name.as_deref())?;
        self.writer.write_str(") (typing ")?;
        if let Some(typing) = &usage.typing {
            self.write_typing(&typing.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") (multiplicity ")?;
        self.write_multiplicity_clause(usage.multiplicity.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_multiplicity_modifiers(&usage.multiplicity_modifiers)?;
        self.writer.write_str(" (subsets ")?;
        if let Some((subsets, value)) = &usage.subsets {
            self.writer.write_str("(clause ")?;
            self.write_subsetting(&subsets.value)?;
            self.writer.write_str(" (value ")?;
            if let Some(value) = value {
                self.write_expression(value)?;
            } else {
                self.writer.write_str("none")?;
            }
            self.writer.write_str("))")?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_optional_subsetting("redefines", usage.redefines.as_ref())?;
        self.writer.write_str(" (value ")?;
        if let Some(value) = &usage.value {
            self.write_feature_value(&value.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_ref_body(&usage.body)?;
        self.writer.write_char(')')
    }

    /// A part usage at a member position, with the scope's item separator in front of it.
    ///
    /// Every scope that owns a `PartUsage` routes through here, so the same syntax projects the
    /// same way wherever it is written; ten of the thirteen used to write a bare marker instead.
    fn write_part_usage_member(
        &mut self,
        first: &mut bool,
        usage: &super::PartUsage,
    ) -> io::Result<()> {
        self.write_item_prefix(first)?;
        self.write_part_usage(usage)
    }

    fn write_action_usage(&mut self, usage: &super::ActionUsage) -> io::Result<()> {
        self.writer.write_str("(action-usage (name ")?;
        write_quoted(self.writer, &usage.name)?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, usage.short_name.as_deref())?;
        self.writer.write_str(") ")?;
        match &usage.body {
            Some(body) => self.write_action_usage_body(body)?,
            None => self.writer.write_str("(body absent)")?,
        }
        self.writer.write_char(')')
    }

    /// `ActionUsageBody`, with its members.
    ///
    /// An action usage was a leaf however much it contained, so a part usage written inside one
    /// -- which `ActionBodyItem -> NonBehaviorBodyItem -> StructureUsageMember` admits -- had no
    /// projection at all. Its `ActionDefBodyElement` sibling has been projected for some time;
    /// this is the same decision applied to the usage scope. Exhaustive, so a new member of this
    /// scope is a compile error here.
    fn write_action_usage_body(&mut self, body: &super::ActionUsageBody) -> io::Result<()> {
        match body {
            super::ActionUsageBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            super::ActionUsageBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        super::ActionUsageBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        super::ActionUsageBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        super::ActionUsageBodyElement::MetadataKeywordUsage(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&member.value)?;
                        }
                        super::ActionUsageBodyElement::MetadataUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_usage(&usage.value)?;
                        }
                        super::ActionUsageBodyElement::PartUsage(member) => {
                            self.write_part_usage_member(&mut first, &member.value)?;
                        }
                        super::ActionUsageBodyElement::ItemUsage(usage) => {
                            self.write_item_usage_member(&mut first, &usage.value)?;
                        }
                        super::ActionUsageBodyElement::OccurrenceUsage(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_occurrence(&member.value)?;
                        }
                        super::ActionUsageBodyElement::AttributeUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_usage(&usage.value)?;
                        }
                        super::ActionUsageBodyElement::ActionUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_action_usage(&usage.value)?;
                        }
                        super::ActionUsageBodyElement::RefDecl(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                        super::ActionUsageBodyElement::ForLoop(for_loop) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_for_loop(&for_loop.value)?;
                        }
                        super::ActionUsageBodyElement::Dependency(_member) => {
                            self.write_marker(&mut first, "dependency")?;
                        }
                        super::ActionUsageBodyElement::InOutDecl(_member) => {
                            self.write_marker(&mut first, "in-out-declaration")?;
                        }
                        super::ActionUsageBodyElement::Bind(_member) => {
                            self.write_marker(&mut first, "bind")?;
                        }
                        super::ActionUsageBodyElement::FlowUsage(_member) => {
                            self.write_marker(&mut first, "flow-usage")?;
                        }
                        super::ActionUsageBodyElement::FirstStmt(_member) => {
                            self.write_marker(&mut first, "first")?;
                        }
                        super::ActionUsageBodyElement::MergeStmt(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_control_node(
                                "merge",
                                &member.value.declaration,
                                &member.value.body,
                            )?;
                        }
                        super::ActionUsageBodyElement::DecisionStmt(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_control_node(
                                "decide",
                                &member.value.declaration,
                                &member.value.body,
                            )?;
                        }
                        super::ActionUsageBodyElement::JoinStmt(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_control_node(
                                "join",
                                &member.value.declaration,
                                &member.value.body,
                            )?;
                        }
                        super::ActionUsageBodyElement::ForkStmt(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_control_node(
                                "fork",
                                &member.value.declaration,
                                &member.value.body,
                            )?;
                        }
                        super::ActionUsageBodyElement::TerminateStmt(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_terminate_statement(&member.value)?;
                        }
                        super::ActionUsageBodyElement::WhileStmt(_member) => {
                            self.write_marker(&mut first, "while")?;
                        }
                        super::ActionUsageBodyElement::LoopStmt(_member) => {
                            self.write_marker(&mut first, "loop")?;
                        }
                        super::ActionUsageBodyElement::IfStmt(_member) => {
                            self.write_marker(&mut first, "if")?;
                        }
                        super::ActionUsageBodyElement::StateUsage(_member) => {
                            self.write_marker(&mut first, "state-usage")?;
                        }
                        super::ActionUsageBodyElement::AssertConstraint(_member) => {
                            self.write_marker(&mut first, "assert-constraint")?;
                        }
                        super::ActionUsageBodyElement::Assign(_member) => {
                            self.write_marker(&mut first, "assign")?;
                        }
                        super::ActionUsageBodyElement::ThenAction(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_then_action(&member.value)?;
                        }
                        super::ActionUsageBodyElement::CalcUsage(_member) => {
                            self.write_marker(&mut first, "calc-usage")?;
                        }
                        super::ActionUsageBodyElement::ActionDef(_member) => {
                            self.write_marker(&mut first, "action-def")?;
                        }
                        super::ActionUsageBodyElement::DefaultReferenceUsage(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_default_reference_usage(&member.value)?;
                        }
                        super::ActionUsageBodyElement::VariantUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_variant_usage(&usage.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    /// A `connect` statement and its body.
    ///
    /// `ConnectionUsage`'s body is a `UsageBody`, so its members are the usage member set that
    /// [`write_ref_body`](Self::write_ref_body) already projects exhaustively. The statement was
    /// a bare `(connect)` marker, which could not show that the body holds anything at all -- and
    /// until this change it mostly did not.
    fn write_connect_statement(&mut self, connect: &super::ConnectStmt) -> io::Result<()> {
        self.writer.write_str("(connect ")?;
        self.write_ref_body(&connect.body)?;
        self.writer.write_char(')')
    }

    fn write_ref_body(&mut self, body: &super::RefBody) -> io::Result<()> {
        match body {
            super::RefBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            super::RefBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        super::PartUsageBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        super::PartUsageBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        super::PartUsageBodyElement::InOutDecl(_declaration) => {
                            self.write_marker(&mut first, "in-out-declaration")?;
                        }
                        super::PartUsageBodyElement::EndDecl(end) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_end(&end.value)?;
                        }
                        super::PartUsageBodyElement::AttributeUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_usage(&usage.value)?;
                        }
                        super::PartUsageBodyElement::DefaultReferenceUsage(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_default_reference_usage(&member.value)?;
                        }
                        super::PartUsageBodyElement::EnumerationUsage(_member) => {
                            self.write_marker(&mut first, "enumeration-usage")?;
                        }
                        super::PartUsageBodyElement::PartUsage(member) => {
                            self.write_part_usage_member(&mut first, &member.value)?;
                        }
                        super::PartUsageBodyElement::OccurrenceUsage(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_occurrence(&member.value)?;
                        }
                        super::PartUsageBodyElement::PortUsage(member) => {
                            self.write_port_usage_member(&mut first, &member.value)?;
                        }
                        super::PartUsageBodyElement::Bind(_member) => {
                            self.write_marker(&mut first, "bind")?;
                        }
                        // A nested `ref` projects the same way, so an owner cannot hide what
                        // its nested declarations hold either.
                        super::PartUsageBodyElement::Ref(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                        super::PartUsageBodyElement::InterfaceUsage(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_interface_usage(&member.value)?;
                        }
                        super::PartUsageBodyElement::Connect(_member) => {
                            self.write_marker(&mut first, "connect")?;
                        }
                        super::PartUsageBodyElement::FlowUsage(_member) => {
                            self.write_marker(&mut first, "flow-usage")?;
                        }
                        super::PartUsageBodyElement::Perform(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_perform(&member.value)?;
                        }
                        super::PartUsageBodyElement::SuccessionUsage(_member) => {
                            self.write_marker(&mut first, "succession-usage")?;
                        }
                        super::PartUsageBodyElement::Allocate(_member) => {
                            self.write_marker(&mut first, "allocate")?;
                        }
                        super::PartUsageBodyElement::Satisfy(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_satisfy_requirement_usage(&usage.value)?;
                        }
                        super::PartUsageBodyElement::StateUsage(_member) => {
                            self.write_marker(&mut first, "state-usage")?;
                        }
                        super::PartUsageBodyElement::ActionUsage(_member) => {
                            self.write_marker(&mut first, "action-usage")?;
                        }
                        super::PartUsageBodyElement::MetadataKeywordUsage(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&member.value)?;
                        }
                        super::PartUsageBodyElement::VariantUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_variant_usage(&usage.value)?;
                        }
                        super::PartUsageBodyElement::StateDef(_member) => {
                            self.write_marker(&mut first, "state-def")?;
                        }
                        super::PartUsageBodyElement::MetadataDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_definition(&definition.value)?;
                        }
                        super::PartUsageBodyElement::FlowDef(_member) => {
                            self.write_marker(&mut first, "flow-def")?;
                        }
                        super::PartUsageBodyElement::RequirementDef(_member) => {
                            self.write_marker(&mut first, "requirement-def")?;
                        }
                        super::PartUsageBodyElement::OccurrenceDef(member) => {
                            self.write_definition_prefix_marker(
                                &mut first,
                                "occurrence-def",
                                member.value.definition_prefix.as_ref(),
                            )?;
                        }
                        super::PartUsageBodyElement::PortDef(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_port_definition(&member.value)?;
                        }
                        super::PartUsageBodyElement::CalcDef(_member) => {
                            self.write_marker(&mut first, "calc-def")?;
                        }
                        super::PartUsageBodyElement::ConnectionDef(_member) => {
                            self.write_marker(&mut first, "connection-def")?;
                        }
                        super::PartUsageBodyElement::EnumDef(_member) => {
                            self.write_marker(&mut first, "enum-def")?;
                        }
                        super::PartUsageBodyElement::Connection(_member) => {
                            self.write_marker(&mut first, "connection")?;
                        }
                        super::PartUsageBodyElement::AssertConstraint(_member) => {
                            self.write_marker(&mut first, "assert-constraint")?;
                        }
                        super::PartUsageBodyElement::ConstraintDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_constraint_definition(&definition.value)?;
                        }
                        super::PartUsageBodyElement::ConstraintUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_constraint_usage(&usage.value)?;
                        }
                        super::PartUsageBodyElement::CalcUsage(_member) => {
                            self.write_marker(&mut first, "calc-usage")?;
                        }
                        super::PartUsageBodyElement::Import(_member) => {
                            self.write_marker(&mut first, "import")?;
                        }
                        super::PartUsageBodyElement::RequirementUsage(_member) => {
                            self.write_marker(&mut first, "requirement-usage")?;
                        }
                        super::PartUsageBodyElement::ItemDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_item_definition(&definition.value)?;
                        }
                        super::PartUsageBodyElement::ItemUsage(usage) => {
                            self.write_item_usage_member(&mut first, &usage.value)?;
                        }
                        super::PartUsageBodyElement::MetadataUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_usage(&usage.value)?;
                        }
                        super::PartUsageBodyElement::AnalysisCaseDef(member) => {
                            self.write_definition_prefix_marker(
                                &mut first,
                                "analysis-case-def",
                                member.value.definition_prefix.as_ref(),
                            )?;
                        }
                        super::PartUsageBodyElement::AnalysisCaseUsage(_member) => {
                            self.write_marker(&mut first, "analysis-case-usage")?;
                        }
                        super::PartUsageBodyElement::AliasDef(_member) => {
                            self.write_marker(&mut first, "alias-def")?;
                        }
                        super::PartUsageBodyElement::IncludeUseCase(_member) => {
                            self.write_marker(&mut first, "include-use-case")?;
                        }
                        super::PartUsageBodyElement::UseCaseUsage(_member) => {
                            self.write_marker(&mut first, "use-case-usage")?;
                        }
                        super::PartUsageBodyElement::VerificationCaseUsage(_member) => {
                            self.write_marker(&mut first, "verification-case-usage")?;
                        }
                        super::PartUsageBodyElement::ViewDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_view_definition(&definition.value)?;
                        }
                        super::PartUsageBodyElement::ViewUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_view_usage(&usage.value)?;
                        }
                        super::PartUsageBodyElement::ViewpointDef(definition) => {
                            self.write_definition_prefix_marker(
                                &mut first,
                                "viewpoint-def",
                                definition.value.definition_prefix.as_ref(),
                            )?;
                        }
                        super::PartUsageBodyElement::ViewpointUsage(_usage) => {
                            self.write_marker(&mut first, "viewpoint-usage")?;
                        }
                        super::PartUsageBodyElement::RenderingDef(definition) => {
                            self.write_definition_prefix_marker(
                                &mut first,
                                "rendering-def",
                                definition.value.definition_prefix.as_ref(),
                            )?;
                        }
                        super::PartUsageBodyElement::RenderingUsage(_usage) => {
                            self.write_marker(&mut first, "rendering-usage")?;
                        }
                        super::PartUsageBodyElement::KermlClassifier(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_kerml_classifier(&declaration.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    /// One projection for a comment member, shared by every scope that owns one.
    fn write_comment_annotation(&mut self, comment: &super::CommentAnnotation) -> io::Result<()> {
        self.writer.write_str("(comment (keyword ")?;
        match &comment.keyword_span {
            Some(span) => write_span(self.writer, span)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (name ")?;
        match comment
            .identification
            .as_ref()
            .and_then(|i| i.name.as_ref())
        {
            Some(name) => write_quoted(self.writer, name)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (about")?;
        for target in &comment.about_targets {
            self.writer.write_char(' ')?;
            self.write_reference(*target)?;
        }
        self.writer.write_str(") (locale ")?;
        match &comment.locale {
            Some(locale) => write_quoted(self.writer, locale)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") ")?;
        self.write_annotation_body(&comment.body_span, &comment.normalized_text())?;
        self.writer.write_char(')')
    }

    /// The `REGULAR_COMMENT` body of an annotating element: where it is in the source, and what
    /// the pinned processing rules (KerML BNF 214 note 1) make of it.
    ///
    /// The raw bytes are not repeated here -- the span already names them, and a projection that
    /// showed both would be showing the same fact twice. The normalized text is shown because
    /// every consumer is required to agree on it, and an invariant nobody can see is one that
    /// drifts.
    fn write_annotation_body(&mut self, body_span: &Span, normalized: &str) -> io::Result<()> {
        self.writer.write_str("(body ")?;
        write_span(self.writer, body_span)?;
        self.writer.write_str(" (normalized ")?;
        write_quoted(self.writer, normalized)?;
        self.writer.write_str("))")
    }

    /// One projection for a metadata feature, shared by every scope that owns one.
    ///
    /// This was a contentless `(metadata-annotation)` marker: neither the annotated type, the
    /// optional declared name, nor the `about` targets were observable, so a snapshot could not
    /// tell `@Safety;` from `@Security about X;`.
    fn write_metadata_annotation(
        &mut self,
        annotation: &super::MetadataAnnotation,
    ) -> io::Result<()> {
        self.writer.write_str("(metadata-annotation (prefixes")?;
        for prefix in &annotation.prefixes {
            self.writer.write_char(' ')?;
            self.write_metadata_keyword_usage(&prefix.value)?;
        }
        self.writer.write_str(") (introducer ")?;
        self.writer.write_str(match annotation.introducer {
            super::MetadataFeatureIntroducer::At { .. } => "at",
            super::MetadataFeatureIntroducer::Metadata { .. } => "metadata",
        })?;
        self.writer.write_str(") (declared-name ")?;
        match &annotation.declared_name {
            Some(declared) => {
                self.writer.write_str("(name ")?;
                write_optional_quoted(self.writer, declared.value.identification.name.as_deref())?;
                self.writer.write_str(") (short-name ")?;
                write_optional_quoted(
                    self.writer,
                    declared.value.identification.short_name.as_deref(),
                )?;
                self.writer.write_str(") (typed-by ")?;
                self.writer.write_str(match declared.value.typed_by {
                    super::MetadataTypedBy::Colon => "colon",
                    super::MetadataTypedBy::TypedBy => "typed-by",
                })?;
                self.writer.write_char(')')?;
            }
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (type ")?;
        self.write_reference(annotation.type_reference)?;
        self.writer.write_str(") (about")?;
        for target in &annotation.about_targets {
            self.writer.write_char(' ')?;
            self.write_reference(*target)?;
        }
        self.writer.write_str(") ")?;
        self.write_metadata_body(&annotation.body)?;
        self.writer.write_char(')')
    }

    /// `AttributeBody`, with its members.
    ///
    /// `UsageBody = DefinitionBody`, so this one container is the body of every attribute, item
    /// and metadata definition and usage in the language. It used to project as
    /// `(body brace (element-count N))` -- a number -- so every member of six owning families was
    /// invisible to the end-to-end contract, a part usage among them: `item def A { ref part c :
    /// C; }` (`Simple Tests/ItemTest.sysml:7`) showed nothing at all. The match is exhaustive, so
    /// a new member of this scope is a compile error here rather than a silently missing node.
    fn write_attribute_body(&mut self, body: &super::AttributeBody) -> io::Result<()> {
        match body {
            super::AttributeBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            super::AttributeBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        super::AttributeBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        super::AttributeBodyElement::Unsupported(unsupported) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_unsupported(&unsupported.value, &element.span)?;
                        }
                        super::AttributeBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        super::AttributeBodyElement::AttributeUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_usage(&usage.value)?;
                        }
                        super::AttributeBodyElement::DefaultReferenceUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_default_reference_usage(&usage.value)?;
                        }
                        super::AttributeBodyElement::PartUsage(member) => {
                            self.write_part_usage_member(&mut first, &member.value)?;
                        }
                        super::AttributeBodyElement::ItemUsage(usage) => {
                            self.write_item_usage_member(&mut first, &usage.value)?;
                        }
                        super::AttributeBodyElement::OccurrenceUsage(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_occurrence(&member.value)?;
                        }
                        super::AttributeBodyElement::RefDecl(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                        super::AttributeBodyElement::MetadataKeywordUsage(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&member.value)?;
                        }
                        super::AttributeBodyElement::AttributeDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_definition(&definition.value)?;
                        }
                        super::AttributeBodyElement::Connect(_member) => {
                            self.write_marker(&mut first, "connect")?;
                        }
                        super::AttributeBodyElement::AssertConstraint(_member) => {
                            self.write_marker(&mut first, "assert-constraint")?;
                        }
                        super::AttributeBodyElement::KermlFeature(_member) => {
                            self.write_marker(&mut first, "kerml-feature")?;
                        }
                        super::AttributeBodyElement::Invariant(_member) => {
                            self.write_marker(&mut first, "invariant")?;
                        }
                        super::AttributeBodyElement::KermlConnector(_member) => {
                            self.write_marker(&mut first, "kerml-connector")?;
                        }
                        super::AttributeBodyElement::KermlClassifier(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_kerml_classifier(&declaration.value)?;
                        }
                        super::AttributeBodyElement::Bind(_member) => {
                            self.write_marker(&mut first, "bind")?;
                        }
                        super::AttributeBodyElement::Connection(_member) => {
                            self.write_marker(&mut first, "connection")?;
                        }
                        super::AttributeBodyElement::CalcDef(_member) => {
                            self.write_marker(&mut first, "calc-def")?;
                        }
                        super::AttributeBodyElement::CalcUsage(_member) => {
                            self.write_marker(&mut first, "calc-usage")?;
                        }
                        super::AttributeBodyElement::ConstraintUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_constraint_usage(&usage.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    /// `ItemDefinition = OccurrenceDefinitionPrefix 'item' 'def' Definition` (SysML BNF 613).
    ///
    /// Was a contentless `(item-def)` marker in all six scopes that own one, so nothing it
    /// declared appeared anywhere -- including a part usage, which is why `item def A { ref part
    /// c : C; }` (`Simple Tests/ItemTest.sysml:7`) had no projection at all.
    fn write_item_definition(&mut self, definition: &super::ItemDef) -> io::Result<()> {
        self.writer.write_str("(item-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_definition_modifiers(definition.definition_prefix.as_ref())?;
        write!(
            self.writer,
            ") (individual {}) (specializes ",
            definition.is_individual
        )?;
        match &definition.specializes {
            Some(typing) => self.write_typing(&typing.value)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") ")?;
        self.write_attribute_body(&definition.body)?;
        self.writer.write_char(')')
    }

    /// `AttributeDefinition = DefinitionPrefix 'attribute' 'def' Definition` (SysML BNF 600).
    ///
    /// Was a contentless `(attribute-def)` marker in all seven scopes that own one.
    fn write_attribute_definition(&mut self, definition: &super::AttributeDef) -> io::Result<()> {
        self.writer.write_str("(attribute-def (declaration-name ")?;
        self.write_usage_declaration_name(&definition.name)?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, definition.short_name.as_deref())?;
        self.write_definition_modifiers(definition.definition_prefix.as_ref())?;
        self.writer.write_str(") (typing ")?;
        match &definition.typing {
            Some(typing) => self.write_typing(&typing.value)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (multiplicity ")?;
        self.write_multiplicity_clause(definition.multiplicity.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_multiplicity_modifiers(&definition.multiplicity_modifiers)?;
        self.writer.write_str(" (value ")?;
        match &definition.value {
            Some(value) => self.write_feature_value(&value.value)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") ")?;
        self.write_attribute_body(&definition.body)?;
        self.writer.write_char(')')
    }

    /// `VariantUsageMember : VariantMembership = MemberPrefix 'variant' VariantUsageElement`
    /// (SysML BNF 251).
    ///
    /// Was a contentless `(variant-usage)` marker in every scope that owns one, so the nested
    /// usage a typed variant declares -- a part usage among the six kinds `VariantUsageElement`
    /// reaches here -- was invisible, and the untyped `variant name { … }` form showed neither
    /// its target nor its body.
    fn write_variant_usage(&mut self, usage: &super::VariantUsage) -> io::Result<()> {
        self.writer.write_str("(variant-usage (target ")?;
        match usage.reference {
            Some(reference) => self.write_reference(reference)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (usage ")?;
        match &usage.typed {
            Some(super::VariantTypedUsage::Part(usage)) => self.write_part_usage(&usage.value)?,
            Some(super::VariantTypedUsage::Item(usage)) => self.write_item_usage(&usage.value)?,
            Some(super::VariantTypedUsage::Attribute(usage)) => {
                self.write_attribute_usage(&usage.value)?
            }
            Some(super::VariantTypedUsage::Port(usage)) => self.write_port_usage(&usage.value)?,
            Some(super::VariantTypedUsage::Perform(_usage)) => {
                self.writer.write_str("(perform)")?
            }
            Some(super::VariantTypedUsage::Requirement(_usage)) => {
                self.writer.write_str("(requirement-usage)")?
            }
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") ")?;
        match &usage.body {
            Some(body) => self.write_ref_body(body)?,
            None => self.writer.write_str("(body absent)")?,
        }
        self.writer.write_char(')')
    }

    /// `MetadataUsage = ( '@' | 'metadata' ) MetadataUsageDeclaration ( 'about' … )?
    /// MetadataBody` (SysML BNF 1673).
    ///
    /// Its body is an `AttributeBody`, so it owns the same members every other one does; the
    /// marker it used to be hid them.
    fn write_metadata_usage(&mut self, usage: &super::MetadataUsage) -> io::Result<()> {
        self.writer
            .write_str("(metadata-usage (declaration-name ")?;
        self.write_usage_declaration_name(&usage.name)?;
        self.writer.write_str(") (type ")?;
        match usage.type_reference {
            Some(reference) => self.write_reference(reference)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (about")?;
        for target in &usage.about_targets {
            self.writer.write_char(' ')?;
            self.write_reference(*target)?;
        }
        self.writer.write_str(") ")?;
        self.write_attribute_body(&usage.body)?;
        self.writer.write_char(')')
    }

    /// The currently supported semicolon form of `TerminateNode` (SysML BNF 1116-1121).
    fn write_terminate_statement(&mut self, terminate: &super::TerminateStmt) -> io::Result<()> {
        self.writer.write_str("(terminate (target ")?;
        match &terminate.target {
            Some(target) => self.write_expression(target)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str("))")
    }

    /// `MetadataDefinition = 'metadata' 'def' Definition` (SysML BNF 1670).
    fn write_metadata_definition(&mut self, definition: &super::MetadataDef) -> io::Result<()> {
        self.writer.write_str("(metadata-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        write!(
            self.writer,
            ") (abstract {}) (specializes ",
            definition.is_abstract
        )?;
        match &definition.specializes {
            Some(typing) => self.write_typing(&typing.value)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") ")?;
        self.write_attribute_body(&definition.body)?;
        self.writer.write_char(')')
    }

    /// `MetadataBody` is an `AttributeBody`, and projects like every other one.
    fn write_metadata_body(&mut self, body: &super::AttributeBody) -> io::Result<()> {
        self.write_attribute_body(body)
    }

    /// One projection for the `#` spelling of a metadata reference, shared by every scope that
    /// owns one.
    ///
    /// This was a contentless `(metadata-keyword-usage)` marker whose only typed content -- the
    /// referenced metadata type -- was an unqualifiable `String` copied out of the source. The
    /// body distinguishes the two `#` productions: `none` is `PrefixMetadataMember`, a body is
    /// `ExtendedUsage` with an empty declaration.
    fn write_metadata_keyword_usage(
        &mut self,
        usage: &super::MetadataKeywordUsage,
    ) -> io::Result<()> {
        self.writer.write_str("(metadata-keyword-usage (type ")?;
        self.write_reference(usage.reference)?;
        self.writer.write_str(") ")?;
        match &usage.body {
            Some(body) => self.write_metadata_body(body)?,
            None => self.writer.write_str("(body none)")?,
        }
        self.writer.write_char(')')
    }

    fn write_annotating_member(&mut self, member: &super::AnnotatingMember) -> io::Result<()> {
        match member {
            super::AnnotatingMember::Doc(doc) => {
                self.writer.write_str("(doc (name ")?;
                match doc
                    .value
                    .identification
                    .as_ref()
                    .and_then(|i| i.name.as_ref())
                {
                    Some(name) => write_quoted(self.writer, name)?,
                    None => self.writer.write_str("none")?,
                }
                self.writer.write_str(") (locale ")?;
                match &doc.value.locale {
                    Some(locale) => write_quoted(self.writer, locale)?,
                    None => self.writer.write_str("none")?,
                }
                self.writer.write_str(") ")?;
                self.write_annotation_body(&doc.value.body_span, &doc.value.normalized_text())?;
                self.writer.write_char(')')
            }
            // The keyword and the locale are grammatical facts, not formatting: a comment member
            // emitted without its authored keyword becomes a bare block comment, which reparses
            // as trivia and disappears. A bare `(comment)` marker could not tell the two
            // spellings apart, so it names them.
            super::AnnotatingMember::Comment(comment) => {
                self.write_comment_annotation(&comment.value)
            }
            super::AnnotatingMember::TextualRep(rep) => {
                self.writer.write_str("(textual-rep (name ")?;
                match rep
                    .value
                    .rep_identification
                    .as_ref()
                    .and_then(|i| i.name.as_ref())
                {
                    Some(name) => write_quoted(self.writer, name)?,
                    None => self.writer.write_str("none")?,
                }
                self.writer.write_str(") (language ")?;
                write_quoted(self.writer, &rep.value.language)?;
                self.writer.write_str(") ")?;
                self.write_annotation_body(&rep.value.body_span, &rep.value.normalized_text())?;
                self.writer.write_char(')')
            }
            super::AnnotatingMember::MetadataAnnotation(annotation) => {
                self.write_metadata_annotation(&annotation.value)
            }
        }
    }

    fn write_end(&mut self, end: &super::EndDecl) -> io::Result<()> {
        self.writer.write_str("(end (short-name ")?;
        write_optional_quoted(self.writer, end.short_name.as_deref())?;
        self.writer.write_str(") (identity ")?;
        match &end.identity {
            EndIdentity::Declaration(name) => {
                self.writer.write_str("(declaration (name ")?;
                write_quoted(self.writer, &name.value)?;
                self.writer.write_str(") ")?;
                write_span(self.writer, &name.span)?;
                self.writer.write_char(')')?;
            }
            EndIdentity::Derivation(role) => {
                self.writer.write_str("(derivation-role (kind ")?;
                match role.value {
                    DerivationEndRole::Original => self.writer.write_str("original")?,
                    DerivationEndRole::Derive => self.writer.write_str("derive")?,
                }
                self.writer.write_str(") ")?;
                write_span(self.writer, &role.span)?;
                self.writer.write_char(')')?;
            }
        }
        self.writer.write_str(") (typing ")?;
        if let Some(typing) = &end.typing {
            self.write_typing(&typing.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_optional_subsetting("references", end.references.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("redefines", end.redefines.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("crosses", end.crosses.as_ref())?;
        self.writer.write_char(')')
    }

    fn write_return_declaration(&mut self, declaration: &super::ReturnDecl) -> io::Result<()> {
        self.writer.write_str("(return-declaration (name ")?;
        if declaration.name.is_empty() {
            self.writer.write_str("none")?;
        } else {
            write_quoted(self.writer, &declaration.name)?;
        }
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, declaration.short_name.as_deref())?;
        self.writer.write_str("))")
    }

    fn write_case_return(&mut self, declaration: &super::CaseReturnDecl) -> io::Result<()> {
        self.writer.write_str("(case-return (declaration ")?;
        write_quoted(self.writer, &declaration.declaration_name)?;
        self.writer.write_str(") (target ")?;
        if let Some(reference) = declaration.target {
            self.write_reference(reference)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") (type ")?;
        if let Some(reference) = declaration.type_name {
            self.write_reference(reference)?;
        } else {
            self.writer.write_str("none")?;
        }
        // The trailing `:>>` clause is a different fact from `target` (the leading anonymous
        // form), so it is projected separately rather than folded into it.
        self.writer.write_str(") (redefines ")?;
        if let Some(redefines) = &declaration.redefines {
            self.write_subsetting(&redefines.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") (feature-kind ")?;
        match declaration.feature_kind {
            Some(CaseReturnFeatureKind::Part) => self.writer.write_str("part")?,
            Some(CaseReturnFeatureKind::Attribute) => self.writer.write_str("attribute")?,
            None => self.writer.write_str("none")?,
        }
        write!(
            self.writer,
            ") (subsetting {}) (value ",
            declaration.is_subsetting
        )?;
        if let Some(value) = &declaration.value {
            self.write_feature_value(&value.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str("))")
    }

    fn write_feature_value(&mut self, value: &FeatureValue) -> io::Result<()> {
        self.writer.write_str("(feature-value (kind ")?;
        match value.kind {
            FeatureValueKind::Bind => self.writer.write_str("bind")?,
            FeatureValueKind::Assign => self.writer.write_str("assign")?,
        }
        write!(self.writer, ") (default {}) (expression ", value.is_default)?;
        self.write_expression(&value.expression)?;
        self.writer.write_str("))")
    }

    fn write_typing(&mut self, relationship: &TypingRelationship) -> io::Result<()> {
        self.writer.write_str("(typing (kind ")?;
        match relationship.kind {
            TypingKind::Typing => self.writer.write_str("typing")?,
            TypingKind::Subclassification => self.writer.write_str("subclassification")?,
        }
        write!(
            self.writer,
            ") (conjugated {}) (implied {}) (targets",
            relationship.is_conjugated, relationship.is_implied
        )?;
        for reference in &relationship.target {
            self.writer.write_char(' ')?;
            self.write_reference(*reference)?;
        }
        self.writer.write_str("))")
    }

    /// The three nested case usages (`use case`, `case`, `verification`) are the same declaration
    /// shape under different keywords, so they project through one writer. The type and subsets
    /// targets are named rather than reduced to a marker because the declaration's tail used to
    /// be discarded outright, and a marker could not show that it no longer is.
    fn write_case_like_usage(
        &mut self,
        label: &str,
        name: &str,
        is_abstract: bool,
        type_name: Option<QualifiedReferenceId>,
        subsets: Option<&Node<SubsettingRelationship>>,
    ) -> io::Result<()> {
        self.writer.write_str("(")?;
        self.writer.write_str(label)?;
        self.writer.write_str(" (name ")?;
        write_quoted(self.writer, name)?;
        self.writer.write_str(") (abstract ")?;
        self.writer
            .write_str(if is_abstract { "true" } else { "false" })?;
        self.writer.write_str(") (type ")?;
        if let Some(reference) = type_name {
            self.write_reference(reference)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") (subsets ")?;
        if let Some(subsets) = subsets {
            self.write_subsetting(&subsets.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str("))")
    }

    fn write_subsetting(&mut self, relationship: &SubsettingRelationship) -> io::Result<()> {
        self.writer.write_str("(relationship (kind ")?;
        match relationship.kind {
            SubsettingKind::Subsets => self.writer.write_str("subsets")?,
            SubsettingKind::References => self.writer.write_str("references")?,
            SubsettingKind::Redefines => self.writer.write_str("redefines")?,
            SubsettingKind::Crosses => self.writer.write_str("crosses")?,
            SubsettingKind::Intersects => self.writer.write_str("intersects")?,
        }
        write!(
            self.writer,
            ") (implied {}) (targets",
            relationship.is_implied
        )?;
        for reference in &relationship.target {
            self.writer.write_char(' ')?;
            self.write_reference(*reference)?;
        }
        self.writer.write_str("))")
    }

    fn write_optional_subsetting(
        &mut self,
        role: &str,
        relationship: Option<&Node<SubsettingRelationship>>,
    ) -> io::Result<()> {
        write!(self.writer, "({role} ")?;
        if let Some(relationship) = relationship {
            self.write_subsetting(&relationship.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_char(')')
    }

    fn write_connect(&mut self, connect: &super::Connect) -> io::Result<()> {
        self.writer.write_str("(connect (from ")?;
        self.write_expression(&connect.from.value.expression)?;
        self.writer.write_str(") (to ")?;
        self.write_expression(&connect.to.value.expression)?;
        self.writer.write_str(") ")?;
        self.write_ref_body(&connect.body)?;
        self.writer.write_str(" ")?;
        self.write_optional_subsetting("subsets", connect.subsets.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("redefines", connect.redefines.as_ref())?;
        self.writer.write_char(')')
    }

    /// A KerML `binding` member owns a `TypeBody`; project it rather than reducing the member to
    /// a marker so nested annotating members remain observable.
    fn write_bind(&mut self, bind: &super::KermlBindingMember) -> io::Result<()> {
        self.writer.write_str("(binding (name ")?;
        if bind.name.is_empty() {
            self.writer.write_str("none")?;
        } else {
            write_quoted(self.writer, &bind.name)?;
        }
        self.writer.write_str(") ")?;
        self.write_calc_def_body(&bind.body)?;
        self.writer.write_char(')')
    }

    /// `InterfaceUsage` is a member-bearing production in all three of its declaration forms.
    /// Project the form and typed body so a regular comment cannot be invisible in a snapshot.
    fn write_interface_usage(&mut self, usage: &super::InterfaceUsage) -> io::Result<()> {
        self.writer.write_str("(interface-usage (form ")?;
        let body = match usage {
            super::InterfaceUsage::TypedConnect { body, .. } => {
                self.writer.write_str("typed-connect")?;
                body
            }
            super::InterfaceUsage::Connection { body, .. } => {
                self.writer.write_str("connection")?;
                body
            }
            super::InterfaceUsage::Declaration { body, .. } => {
                self.writer.write_str("declaration")?;
                body
            }
        };
        self.writer.write_str(") ")?;
        self.write_interface_usage_body(body)?;
        self.writer.write_char(')')
    }

    fn write_interface_usage_body(
        &mut self,
        body: &super::Body<super::InterfaceUsageBodyElement>,
    ) -> io::Result<()> {
        match body {
            super::Body::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            super::Body::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        super::InterfaceUsageBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        super::InterfaceUsageBodyElement::RefRedef {
                            target,
                            value,
                            body,
                        } => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(ref-redef (target ")?;
                            self.write_reference(*target)?;
                            self.writer.write_str(") (value ")?;
                            self.write_expression(value)?;
                            self.writer.write_str(") ")?;
                            self.write_ref_body(body)?;
                            self.writer.write_char(')')?;
                        }
                        super::InterfaceUsageBodyElement::EndDecl(end) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_end(&end.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_perform(&mut self, perform: &super::Perform) -> io::Result<()> {
        self.writer.write_str("(perform (declaration ")?;
        write_quoted(self.writer, &perform.action_name)?;
        self.writer.write_str(") (action ")?;
        if let Some(reference) = perform.action_reference {
            self.write_reference(reference)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") (typing ")?;
        if let Some(typing) = &perform.typing {
            self.write_typing(&typing.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_optional_subsetting("subsets", perform.subsets.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("redefines", perform.redefines.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_perform_body(&perform.body)?;
        self.writer.write_char(')')
    }

    fn write_perform_body(&mut self, body: &PerformBody) -> io::Result<()> {
        match body {
            PerformBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            PerformBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        PerformBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        PerformBodyElement::InOut(binding) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(binding (direction ")?;
                            match binding.value.direction {
                                InOut::In => self.writer.write_str("in")?,
                                InOut::Out => self.writer.write_str("out")?,
                                InOut::InOut => self.writer.write_str("inout")?,
                            }
                            self.writer.write_str(") (target ")?;
                            self.write_reference(binding.value.target)?;
                            self.writer.write_str(") (value ")?;
                            self.write_expression(&binding.value.value)?;
                            self.writer.write_str("))")?;
                        }
                        PerformBodyElement::Variant(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_variant_usage(&usage.value)?;
                        }
                        PerformBodyElement::Action(_action) => {
                            self.write_marker(&mut first, "action")?;
                        }
                        PerformBodyElement::PartUsage(usage) => {
                            self.write_part_usage_member(&mut first, &usage.value)?;
                        }
                        PerformBodyElement::ItemUsage(usage) => {
                            self.write_item_usage_member(&mut first, &usage.value)?;
                        }
                        PerformBodyElement::AttributeUsage(_usage) => {
                            self.write_marker(&mut first, "attribute-usage")?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_occurrence(&mut self, occurrence: &super::OccurrenceUsage) -> io::Result<()> {
        self.writer.write_str("(occurrence ")?;
        self.write_occurrence_usage_prefix(&occurrence.prefix)?;
        // `event` selects `EventOccurrenceUsage`, a distinct production from the ordinary
        // occurrence/individual family. Keep that grammatical identity visible to snapshots so
        // an event member cannot round-trip as an indistinguishable plain occurrence.
        if occurrence.is_event {
            self.writer.write_str(" (event true)")?;
        }
        self.writer.write_str(" (declaration ")?;
        write_quoted(self.writer, &occurrence.name)?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, occurrence.short_name.as_deref())?;
        self.writer.write_str(") (target ")?;
        if let Some(reference) = occurrence.occurrence_reference {
            self.write_reference(reference)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_occurrence_usage_body(&occurrence.body)?;
        self.writer.write_char(')')
    }

    /// `OccurrenceUsageBody`, with its members.
    ///
    /// The prefix and declaration were projected but the body was not, so an occurrence usage was
    /// a leaf however much it contained -- and a part usage written inside one
    /// (`individual part vehicle_1 { snapshot part vehicle_1_t0 { … } }`) had no projection at
    /// all. Exhaustive, so a new member of this scope is a compile error here.
    fn write_occurrence_usage_body(&mut self, body: &super::OccurrenceUsageBody) -> io::Result<()> {
        match body {
            super::OccurrenceUsageBody::Semicolon { .. } => {
                self.writer.write_str("(body semicolon)")
            }
            super::OccurrenceUsageBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    self.write_occurrence_body_element(&mut first, element)?;
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_occurrence_body_element(
        &mut self,
        first: &mut bool,
        element: &Node<super::OccurrenceBodyElement>,
    ) -> io::Result<()> {
        match &element.value {
            super::OccurrenceBodyElement::Error(error) => {
                self.write_item_prefix(first)?;
                self.write_malformed(&error.value, &element.span)
            }
            super::OccurrenceBodyElement::Annotating(member) => {
                self.write_item_prefix(first)?;
                self.write_annotating_member(member)
            }
            super::OccurrenceBodyElement::MetadataKeywordUsage(member) => {
                self.write_item_prefix(first)?;
                self.write_metadata_keyword_usage(&member.value)
            }
            super::OccurrenceBodyElement::AttributeUsage(usage) => {
                self.write_item_prefix(first)?;
                self.write_attribute_usage(&usage.value)
            }
            super::OccurrenceBodyElement::PartUsage(member) => {
                self.write_part_usage_member(first, &member.value)
            }
            super::OccurrenceBodyElement::ItemUsage(usage) => {
                self.write_item_usage_member(first, &usage.value)
            }
            super::OccurrenceBodyElement::OccurrenceUsage(member) => {
                self.write_item_prefix(first)?;
                self.write_occurrence(&member.value)
            }
            super::OccurrenceBodyElement::Satisfy(usage) => {
                self.write_item_prefix(first)?;
                self.write_satisfy_requirement_usage(&usage.value)
            }
            super::OccurrenceBodyElement::RefDecl(declaration) => {
                self.write_item_prefix(first)?;
                self.write_ref_declaration(&declaration.value)
            }
            super::OccurrenceBodyElement::EndDecl(end) => {
                self.write_item_prefix(first)?;
                self.write_end(&end.value)
            }
            super::OccurrenceBodyElement::AssertConstraint(_) => {
                self.write_marker(first, "assert-constraint")
            }
            super::OccurrenceBodyElement::FlowUsage(_) => self.write_marker(first, "flow-usage"),
            super::OccurrenceBodyElement::Bind(_) => self.write_marker(first, "bind"),
            super::OccurrenceBodyElement::SuccessionUsage(_) => {
                self.write_marker(first, "succession-usage")
            }
            super::OccurrenceBodyElement::Allocate(_) => self.write_marker(first, "allocate"),
            super::OccurrenceBodyElement::StateUsage(_) => self.write_marker(first, "state-usage"),
            super::OccurrenceBodyElement::ConnectionUsage(_) => {
                self.write_marker(first, "connection-usage")
            }
        }
    }

    fn write_definition_body(&mut self, body: &super::DefinitionBody) -> io::Result<()> {
        match body {
            super::DefinitionBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            super::DefinitionBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        super::DefinitionBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        super::DefinitionBodyElement::Unsupported(unsupported) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_unsupported(&unsupported.value, &element.span)?;
                        }
                        super::DefinitionBodyElement::OccurrenceMember(member) => {
                            self.write_occurrence_body_element(&mut first, member)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_subject_decl(&mut self, subject: &super::SubjectDecl) -> io::Result<()> {
        self.writer.write_str("(subject (name ")?;
        write_quoted(self.writer, &subject.name)?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, subject.short_name.as_deref())?;
        self.writer.write_str(") (type ")?;
        if let Some(reference) = subject.type_name {
            self.write_reference(reference)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") (redefines ")?;
        match &subject.redefines {
            Some(redefines) => self.write_subsetting(&redefines.value)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (value ")?;
        if let Some(value) = &subject.value {
            self.write_feature_value(&value.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_definition_body(&subject.body)?;
        self.writer.write_char(')')
    }

    fn write_requirement_definition(
        &mut self,
        definition: &super::RequirementDef,
    ) -> io::Result<()> {
        self.writer.write_str("(requirement-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_definition_modifiers(definition.definition_prefix.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_requirement_body(&definition.body)?;
        self.writer.write_char(')')
    }

    /// `ViewDefinition` (SysML 8.2.2.26.1).
    ///
    /// Was a contentless `(view-def)` marker in all three scopes that own one, so a snapshot
    /// could show neither the declaration, nor `abstract`, nor the subclassification clause, nor
    /// a single member of the body -- a `view def` was the one definition family whose entire
    /// contents were invisible to the end-to-end contract, which is why a satisfy usage inside
    /// one could only be pinned through `FORMAT`.
    fn write_view_definition(&mut self, definition: &super::ViewDef) -> io::Result<()> {
        self.writer.write_str("(view-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, definition.identification.short_name.as_deref())?;
        self.write_definition_modifiers(definition.definition_prefix.as_ref())?;
        self.writer.write_str(") (specializes ")?;
        match &definition.specializes {
            Some(specializes) => self.write_typing(&specializes.value)?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") ")?;
        self.write_view_def_body(&definition.body)?;
        self.writer.write_char(')')
    }

    fn write_view_def_body(&mut self, body: &super::ViewDefBody) -> io::Result<()> {
        match body {
            super::ViewDefBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            super::ViewDefBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        super::ViewDefBodyElement::Unsupported(node) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_unsupported(&node.value, &element.span)?;
                        }
                        super::ViewDefBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        super::ViewDefBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        super::ViewDefBodyElement::MetadataKeywordUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&usage.value)?;
                        }
                        super::ViewDefBodyElement::AliasDef(alias) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_alias_definition(&alias.value)?;
                        }
                        super::ViewDefBodyElement::Filter(_filter) => {
                            self.write_marker(&mut first, "filter")?;
                        }
                        super::ViewDefBodyElement::ViewRendering(_rendering) => {
                            self.write_marker(&mut first, "view-rendering")?;
                        }
                        super::ViewDefBodyElement::RenderingUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_rendering_usage(&usage.value)?;
                        }
                        super::ViewDefBodyElement::RefDecl(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                        super::ViewDefBodyElement::ViewpointUsage(_usage) => {
                            self.write_marker(&mut first, "viewpoint-usage")?;
                        }
                        super::ViewDefBodyElement::Satisfy(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_satisfy_requirement_usage(&usage.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_view_usage(&mut self, usage: &super::ViewUsage) -> io::Result<()> {
        self.writer.write_str("(view (name ")?;
        write_quoted(self.writer, &usage.name)?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, usage.short_name.as_deref())?;
        self.writer.write_str(") (type ")?;
        if let Some(reference) = usage.type_name {
            self.write_reference(reference)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_view_body(&usage.body)?;
        self.writer.write_char(')')
    }

    /// A `RenderingUsage` member carries the generic usage header, not merely the view-specific
    /// `render` binding marker. Keep each retained clause visible so the same typed node has the
    /// same semantic projection in both view body scopes.
    fn write_rendering_usage(&mut self, usage: &super::RenderingUsage) -> io::Result<()> {
        self.writer.write_str("(rendering-usage (abstract ")?;
        self.writer
            .write_str(if usage.is_abstract { "true" } else { "false" })?;
        self.writer.write_str(") (name ")?;
        self.write_usage_declaration_name(&usage.name)?;
        self.writer.write_str(") (type ")?;
        if let Some(reference) = usage.type_name {
            self.write_reference(reference)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") (multiplicity ")?;
        self.write_multiplicity_clause(usage.multiplicity.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_multiplicity_modifiers(&usage.multiplicity_modifiers)?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("subsets", usage.subsets.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("redefines", usage.redefines.as_ref())?;
        self.writer.write_str(" (value ")?;
        if let Some(value) = &usage.value {
            self.write_feature_value(&value.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_rendering_usage_body(&usage.body)?;
        self.writer.write_char(')')
    }

    fn write_rendering_usage_body(&mut self, body: &super::RenderingUsageBody) -> io::Result<()> {
        match body {
            super::RenderingUsageBody::Semicolon { .. } => {
                self.writer.write_str("(body semicolon)")
            }
            super::RenderingUsageBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        super::RenderingUsageBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        super::RenderingUsageBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        super::RenderingUsageBodyElement::ViewUsage(view) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_view_usage(&view.value)?;
                        }
                        super::RenderingUsageBodyElement::Rendering(nested) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_rendering_usage(&nested.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_use_case_definition(&mut self, definition: &super::UseCaseDef) -> io::Result<()> {
        self.writer.write_str("(use-case-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_definition_modifiers(definition.definition_prefix.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_use_case_body(&definition.body)?;
        self.writer.write_char(')')
    }

    fn write_state_definition(&mut self, definition: &super::StateDef) -> io::Result<()> {
        self.writer.write_str("(state-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_definition_modifiers(definition.definition_prefix.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_state_body(&definition.body)?;
        self.writer.write_char(')')
    }

    fn write_enumeration_definition(&mut self, definition: &super::EnumDef) -> io::Result<()> {
        self.writer.write_str("(enum-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.writer.write_str(") ")?;
        self.write_enumeration_body(&definition.body)?;
        self.writer.write_char(')')
    }

    /// `EnumerationBody` is the one production that names `AnnotatingMember` directly, so its
    /// members are projected rather than counted: this scope discarded annotating members
    /// entirely, and a count alone could not show that they are retained in authored order
    /// between the values.
    fn write_enumeration_body(&mut self, body: &super::EnumerationBody) -> io::Result<()> {
        match body {
            super::EnumerationBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            super::EnumerationBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        super::EnumerationBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        super::EnumerationBodyElement::Value(value) => {
                            self.write_item_prefix(&mut first)?;
                            self.writer.write_str("(enum-value (name ")?;
                            write_quoted(self.writer, &value.value.name)?;
                            self.writer.write_str(") (short-name ")?;
                            write_optional_quoted(self.writer, value.value.short_name.as_deref())?;
                            self.writer.write_str(") (value ")?;
                            if let Some(initializer) = &value.value.value {
                                self.write_feature_value(&initializer.value)?;
                            } else {
                                self.writer.write_str("none")?;
                            }
                            self.writer.write_str(") ")?;
                            self.write_ref_body(&value.value.body)?;
                            self.writer.write_char(' ')?;
                            write_span(self.writer, &value.span)?;
                            self.writer.write_char(')')?;
                        }
                        super::EnumerationBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_part_definition(&mut self, definition: &super::PartDef) -> io::Result<()> {
        self.writer.write_str("(part-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_occurrence_definition_modifiers(
            definition.definition_prefix.as_ref(),
            definition.is_individual,
        )?;
        self.writer.write_str(") ")?;
        self.write_part_body(&definition.body)?;
        self.writer.write_char(')')
    }

    fn write_usage_declaration_name(&mut self, name: &str) -> io::Result<()> {
        if name.is_empty() {
            self.writer.write_str("none")
        } else {
            write_quoted(self.writer, name)
        }
    }

    fn write_direction(&mut self, direction: Option<InOut>) -> io::Result<()> {
        match direction {
            Some(InOut::In) => self.writer.write_str("in"),
            Some(InOut::Out) => self.writer.write_str("out"),
            Some(InOut::InOut) => self.writer.write_str("inout"),
            None => self.writer.write_str("none"),
        }
    }

    /// The shared `OccurrenceUsagePrefix`, projected in the production's own slot order.
    ///
    /// One projection for every family that owns the prefix, so a snapshot shows the same
    /// language-level facts wherever it appears. Each slot names the alternative that was
    /// authored -- presence is the grammatical fact, exactly as `(assert true)` is for
    /// `SatisfyRequirementUsage` -- and the extension keywords appear as the references they are,
    /// in authored order, so a snapshot can tell `#a #b` from `#b #a`.
    fn write_occurrence_usage_prefix(
        &mut self,
        prefix: &super::OccurrenceUsagePrefix,
    ) -> io::Result<()> {
        let ref_prefix = &prefix.basic.ref_prefix;
        self.writer.write_str("(prefix (direction ")?;
        self.write_direction(ref_prefix.direction.as_ref().map(|node| node.value))?;
        write!(
            self.writer,
            ") (derived {}) (variance ",
            ref_prefix.derived_span.is_some()
        )?;
        match ref_prefix.variance.as_ref().map(|node| node.value) {
            Some(super::DefinitionPrefix::Abstract) => self.writer.write_str("abstract")?,
            Some(super::DefinitionPrefix::Variation) => self.writer.write_str("variation")?,
            None => self.writer.write_str("none")?,
        }
        write!(
            self.writer,
            ") (constant {}) (reference {}) (individual {}) (portion ",
            ref_prefix.constant_span.is_some(),
            prefix.basic.reference_span.is_some(),
            prefix.individual_span.is_some()
        )?;
        match prefix.portion.as_ref().map(|node| node.value) {
            Some(super::OccurrencePortionKind::Snapshot) => self.writer.write_str("snapshot")?,
            Some(super::OccurrencePortionKind::Timeslice) => self.writer.write_str("timeslice")?,
            None => self.writer.write_str("none")?,
        }
        self.writer.write_str(") (extensions")?;
        for keyword in &prefix.extension_keywords {
            self.writer.write_char(' ')?;
            self.write_reference(keyword.value.annotation)?;
        }
        self.writer.write_str("))")
    }

    fn write_attribute_usage(&mut self, usage: &super::AttributeUsage) -> io::Result<()> {
        self.writer
            .write_str("(attribute-usage (declaration-name ")?;
        self.write_usage_declaration_name(&usage.name)?;
        self.writer.write_str(") (direction ")?;
        self.write_direction(usage.direction)?;
        // BNF `RefPrefix` plus the `ref` of `BasicUsagePrefix`: authored keywords the emitter
        // reproduces, so the projection has to name them or a snapshot cannot tell `derived
        // abstract constant ref attribute x` from a bare `attribute x`.
        write!(self.writer, ") (derived {})", usage.is_derived)?;
        self.writer.write_str(" (usage-prefix ")?;
        match usage.usage_prefix {
            Some(super::DefinitionPrefix::Abstract) => self.writer.write_str("abstract")?,
            Some(super::DefinitionPrefix::Variation) => self.writer.write_str("variation")?,
            None => self.writer.write_str("none")?,
        }
        write!(
            self.writer,
            ") (constant {}) (reference {}) (end {}",
            usage.is_constant, usage.is_reference, usage.is_end
        )?;
        self.writer.write_str(") (typing ")?;
        if let Some(typing) = &usage.typing {
            self.write_typing(&typing.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_optional_subsetting("subsets", usage.subsets.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("redefines", usage.redefines.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("references", usage.references.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("crosses", usage.crosses.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("intersects", usage.intersects.as_ref())?;
        self.writer.write_str(" (value ")?;
        if let Some(value) = &usage.value {
            self.write_feature_value(&value.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_attribute_body(&usage.body)?;
        self.writer.write_char(')')
    }

    /// Semantic projection of the pinned `DefaultReferenceUsage = RefPrefix Usage` production.
    fn write_default_reference_usage(
        &mut self,
        usage: &super::DefaultReferenceUsage,
    ) -> io::Result<()> {
        self.writer
            .write_str("(default-reference-usage (prefix (direction ")?;
        self.write_direction(usage.prefix.direction.as_ref().map(|node| node.value))?;
        write!(
            self.writer,
            ") (derived {}) (variance ",
            usage.prefix.derived_span.is_some()
        )?;
        match usage.prefix.variance.as_ref().map(|node| node.value) {
            Some(super::DefinitionPrefix::Abstract) => self.writer.write_str("abstract")?,
            Some(super::DefinitionPrefix::Variation) => self.writer.write_str("variation")?,
            None => self.writer.write_str("none")?,
        }
        write!(
            self.writer,
            ") (constant {})) (declaration-name ",
            usage.prefix.constant_span.is_some()
        )?;
        self.write_usage_declaration_name(&usage.name)?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, usage.short_name.as_deref())?;
        self.writer.write_str(") (typing ")?;
        if let Some(typing) = &usage.typing {
            self.write_typing(&typing.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") (multiplicity ")?;
        self.write_multiplicity_clause(usage.multiplicity.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_multiplicity_modifiers(&usage.multiplicity_modifiers)?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("subsets", usage.subsets.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("redefines", usage.redefines.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("references", usage.references.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("crosses", usage.crosses.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("intersects", usage.intersects.as_ref())?;
        self.writer.write_str(" (value ")?;
        if let Some(value) = &usage.value {
            self.write_feature_value(&value.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_attribute_body(&usage.body)?;
        self.writer.write_char(')')
    }

    /// A port usage, projected from its pinned production
    /// (`PortUsage = OccurrenceUsagePrefix 'port' Usage`, SysML BNF 645).
    ///
    /// The prefix comes first, exactly as `write_part_usage` shows it, then `Identification`'s
    /// short name and declared name, then `FeatureSpecializationPart`'s typing, multiplicity and
    /// its `ordered`/`nonunique` modifiers, the specialization clauses, and finally
    /// `UsageCompletion`'s value and body. It used to show only the name, the direction and the
    /// clauses, with the body reduced to an element count, so a snapshot could not tell
    /// `individual derived port p;` from `port p;` nor see a single member of a port body.
    fn write_port_usage(&mut self, usage: &super::PortUsage) -> io::Result<()> {
        self.writer.write_str("(port-usage ")?;
        self.write_occurrence_usage_prefix(&usage.prefix)?;
        self.writer.write_str(" (declaration-name ")?;
        self.write_usage_declaration_name(&usage.name)?;
        self.writer.write_str(") (short-name ")?;
        write_optional_quoted(self.writer, usage.short_name.as_deref())?;
        self.writer.write_str(") (typing ")?;
        if let Some(typing) = &usage.typing {
            self.write_typing(&typing.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") (multiplicity ")?;
        self.write_multiplicity_clause(usage.multiplicity.as_ref())?;
        self.writer.write_str(") ")?;
        self.write_multiplicity_modifiers(&usage.multiplicity_modifiers)?;
        self.writer.write_str(" (subsets ")?;
        if let Some((subsets, value)) = &usage.subsets {
            self.writer.write_str("(clause ")?;
            self.write_subsetting(&subsets.value)?;
            self.writer.write_str(" (value ")?;
            if let Some(value) = value {
                self.write_expression(value)?;
            } else {
                self.writer.write_str("none")?;
            }
            self.writer.write_str("))")?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_optional_subsetting("redefines", usage.redefines.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("references", usage.references.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("crosses", usage.crosses.as_ref())?;
        self.writer.write_char(' ')?;
        self.write_optional_subsetting("intersects", usage.intersects.as_ref())?;
        self.writer.write_str(" (value ")?;
        if let Some(value) = &usage.value {
            self.write_feature_value(&value.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_port_body(&usage.body)?;
        self.writer.write_char(')')
    }

    /// A port usage at a member position, with the scope's item separator in front of it.
    ///
    /// Every scope that owns a `PortUsage` routes through here, so the same syntax projects the
    /// same way wherever it is written; three of the nine used to write a bare marker instead.
    fn write_port_usage_member(
        &mut self,
        first: &mut bool,
        usage: &super::PortUsage,
    ) -> io::Result<()> {
        self.write_item_prefix(first)?;
        self.write_port_usage(usage)
    }

    /// The members of a port usage body, which used to project as an element count.
    fn write_port_body(&mut self, body: &super::PortBody) -> io::Result<()> {
        match body {
            super::PortBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            super::PortBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        super::PortBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        super::PortBodyElement::InOutDecl(_declaration) => {
                            self.write_marker(&mut first, "in-out-declaration")?;
                        }
                        super::PortBodyElement::PortUsage(usage) => {
                            self.write_port_usage_member(&mut first, &usage.value)?;
                        }
                        super::PortBodyElement::OccurrenceUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_occurrence(&usage.value)?;
                        }
                        super::PortBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        super::PortBodyElement::AttributeUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_usage(&usage.value)?;
                        }
                        super::PortBodyElement::ItemUsage(usage) => {
                            self.write_item_usage_member(&mut first, &usage.value)?;
                        }
                        super::PortBodyElement::RefDecl(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_port_definition(&mut self, definition: &super::PortDef) -> io::Result<()> {
        self.writer.write_str("(port-def (name ")?;
        write_optional_quoted(self.writer, definition.identification.name.as_deref())?;
        self.write_definition_modifiers(definition.definition_prefix.as_ref())?;
        self.writer.write_str(") (specializes ")?;
        if let Some(specializes) = &definition.specializes {
            self.write_typing(&specializes.value)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str(") ")?;
        self.write_port_definition_body(&definition.body)?;
        self.writer.write_char(')')
    }

    fn write_port_definition_body(&mut self, body: &PortDefBody) -> io::Result<()> {
        match body {
            PortDefBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            PortDefBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    match &element.value {
                        PortDefBodyElement::InOutDecl(_declaration) => {
                            self.write_marker(&mut first, "in-out-declaration")?;
                        }
                        PortDefBodyElement::RefDecl(declaration) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_ref_declaration(&declaration.value)?;
                        }
                        PortDefBodyElement::Annotating(member) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_annotating_member(member)?;
                        }
                        PortDefBodyElement::Error(error) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_malformed(&error.value, &element.span)?;
                        }
                        PortDefBodyElement::AttributeDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_definition(&definition.value)?;
                        }
                        PortDefBodyElement::AttributeUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_attribute_usage(&usage.value)?;
                        }
                        PortDefBodyElement::ItemDef(definition) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_item_definition(&definition.value)?;
                        }
                        PortDefBodyElement::ItemUsage(usage) => {
                            self.write_item_usage_member(&mut first, &usage.value)?;
                        }
                        PortDefBodyElement::EnumerationUsage(_usage) => {
                            self.write_marker(&mut first, "enumeration-usage")?;
                        }
                        PortDefBodyElement::PortUsage(usage) => {
                            self.write_port_usage_member(&mut first, &usage.value)?;
                        }
                        PortDefBodyElement::MetadataKeywordUsage(usage) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_metadata_keyword_usage(&usage.value)?;
                        }
                        super::PortDefBodyElement::Unsupported(unsupported) => {
                            self.write_item_prefix(&mut first)?;
                            self.write_unsupported(&unsupported.value, &element.span)?;
                        }
                    }
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_library_package(&mut self, package: &super::LibraryPackage) -> io::Result<()> {
        self.writer.write_str("(library-package (name ")?;
        self.write_qualified_identification_name(&package.identification)?;
        write!(self.writer, ") (standard {}) ", package.is_standard)?;
        self.write_package_body(&package.body)?;
        self.writer.write_char(')')
    }

    fn write_package(&mut self, package: &super::Package) -> io::Result<()> {
        self.writer.write_str("(package (name ")?;
        self.write_qualified_identification_name(&package.identification)?;
        self.writer.write_str(") ")?;
        self.write_package_body(&package.body)?;
        self.writer.write_char(')')
    }

    fn write_qualified_identification_name(
        &mut self,
        identification: &super::QualifiedIdentification,
    ) -> io::Result<()> {
        match &identification.name {
            Some(super::DeclarationName::Simple(name)) => write_quoted(self.writer, name),
            Some(super::DeclarationName::Qualified(name)) => {
                self.writer.write_str("(qualified-declaration ")?;
                self.write_reference(name.storage_id())?;
                self.writer.write_char(')')
            }
            None => self.writer.write_str("none"),
        }
    }

    fn write_package_element(
        &mut self,
        element: &Node<PackageBodyElement>,
        first: &mut bool,
    ) -> io::Result<()> {
        match &element.value {
            PackageBodyElement::Error(error) => {
                self.write_item_prefix(first)?;
                self.write_malformed(&error.value, &element.span)
            }
            PackageBodyElement::Unsupported(unsupported) => {
                self.write_item_prefix(first)?;
                self.write_unsupported(&unsupported.value, &element.span)
            }
            PackageBodyElement::Annotating(member) => {
                self.write_item_prefix(first)?;
                self.write_annotating_member(member)
            }
            PackageBodyElement::Filter(_filter) => self.write_marker(first, "filter"),
            PackageBodyElement::Package(package) => {
                self.write_item_prefix(first)?;
                self.write_package(&package.value)
            }
            PackageBodyElement::LibraryPackage(package) => {
                self.write_item_prefix(first)?;
                self.write_library_package(&package.value)
            }
            PackageBodyElement::Import(import) => {
                self.write_item_prefix(first)?;
                self.writer.write_str("(import ")?;
                self.write_import_target(&import.value.target)?;
                self.writer.write_char(')')
            }
            PackageBodyElement::PartDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_part_definition(&definition.value)
            }
            PackageBodyElement::PartUsage(usage) => {
                self.write_item_prefix(first)?;
                self.write_part_usage(&usage.value)
            }
            PackageBodyElement::PortDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_port_definition(&definition.value)
            }
            PackageBodyElement::InterfaceDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_interface_definition(&definition.value)
            }
            PackageBodyElement::AliasDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_alias_definition(&definition.value)
            }
            PackageBodyElement::AttributeDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_attribute_definition(&definition.value)
            }
            PackageBodyElement::ActionDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_action_definition(&definition.value)
            }
            PackageBodyElement::ActionUsage(usage) => {
                self.write_item_prefix(first)?;
                self.write_action_usage(&usage.value)
            }
            PackageBodyElement::RequirementDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_requirement_definition(&definition.value)
            }
            PackageBodyElement::RequirementUsage(usage) => {
                self.write_item_prefix(first)?;
                self.write_named_multiplicity(
                    "requirement-usage",
                    &usage.value.name,
                    usage.value.multiplicity.as_ref(),
                )
            }
            PackageBodyElement::Satisfy(usage) => {
                self.write_item_prefix(first)?;
                self.write_satisfy_requirement_usage(&usage.value)
            }
            PackageBodyElement::UseCaseDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_use_case_definition(&definition.value)
            }
            PackageBodyElement::Actor(_actor) => self.write_marker(first, "actor"),
            PackageBodyElement::StateDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_state_definition(&definition.value)
            }
            PackageBodyElement::StateUsage(_usage) => self.write_marker(first, "state-usage"),
            PackageBodyElement::ItemDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_item_definition(&definition.value)
            }
            PackageBodyElement::IndividualDef(definition) => self.write_definition_prefix_marker(
                first,
                "individual-def",
                definition.value.definition_prefix.as_ref(),
            ),
            PackageBodyElement::ConstraintDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_constraint_definition(&definition.value)
            }
            PackageBodyElement::ConstraintUsage(usage) => {
                self.write_item_prefix(first)?;
                self.write_constraint_usage(&usage.value)
            }
            PackageBodyElement::CalcUsage(usage) => {
                self.write_item_prefix(first)?;
                self.write_calculation_usage(&usage.value)
            }
            PackageBodyElement::CalcDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_calc_definition(&definition.value)
            }
            PackageBodyElement::ViewDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_view_definition(&definition.value)
            }
            PackageBodyElement::ViewpointDef(definition) => self.write_definition_prefix_marker(
                first,
                "viewpoint-def",
                definition.value.definition_prefix.as_ref(),
            ),
            PackageBodyElement::RenderingDef(definition) => self.write_definition_prefix_marker(
                first,
                "rendering-def",
                definition.value.definition_prefix.as_ref(),
            ),
            PackageBodyElement::ViewUsage(usage) => {
                self.write_item_prefix(first)?;
                self.write_view_usage(&usage.value)
            }
            PackageBodyElement::ViewpointUsage(_usage) => {
                self.write_marker(first, "viewpoint-usage")
            }
            PackageBodyElement::RenderingUsage(_usage) => {
                self.write_marker(first, "rendering-usage")
            }
            PackageBodyElement::ConnectionDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_connection_definition(&definition.value)
            }
            PackageBodyElement::MetadataDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_metadata_definition(&definition.value)
            }
            PackageBodyElement::MetadataUsage(usage) => {
                self.write_item_prefix(first)?;
                self.write_metadata_usage(&usage.value)
            }
            PackageBodyElement::EnumDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_enumeration_definition(&definition.value)
            }
            PackageBodyElement::OccurrenceDef(definition) => self.write_definition_prefix_marker(
                first,
                "occurrence-def",
                definition.value.definition_prefix.as_ref(),
            ),
            PackageBodyElement::OccurrenceUsage(occurrence) => {
                self.write_item_prefix(first)?;
                self.write_occurrence(&occurrence.value)
            }
            PackageBodyElement::Dependency(dependency) => {
                self.write_item_prefix(first)?;
                self.write_dependency(&dependency.value)
            }
            PackageBodyElement::AllocationDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_allocation_definition(&definition.value)
            }
            PackageBodyElement::AllocationUsage(_usage) => {
                self.write_marker(first, "allocation-usage")
            }
            PackageBodyElement::FlowDef(definition) => {
                self.write_item_prefix(first)?;
                self.write_flow_definition(&definition.value)
            }
            PackageBodyElement::FlowUsage(_usage) => self.write_marker(first, "flow-usage"),
            PackageBodyElement::ConcernUsage(_usage) => self.write_marker(first, "concern-usage"),
            PackageBodyElement::CaseDef(definition) => self.write_definition_prefix_marker(
                first,
                "case-def",
                definition.value.definition_prefix.as_ref(),
            ),
            PackageBodyElement::CaseUsage(_usage) => self.write_marker(first, "case-usage"),
            PackageBodyElement::AnalysisCaseDef(definition) => self.write_definition_prefix_marker(
                first,
                "analysis-case-def",
                definition.value.definition_prefix.as_ref(),
            ),
            PackageBodyElement::AnalysisCaseUsage(_usage) => {
                self.write_marker(first, "analysis-case-usage")
            }
            PackageBodyElement::VerificationCaseDef(definition) => {
                self.write_item_prefix(first)?;
                self.writer.write_str("(verification-case-def (name ")?;
                write_optional_quoted(
                    self.writer,
                    definition.value.identification.name.as_deref(),
                )?;
                self.write_definition_modifiers(definition.value.definition_prefix.as_ref())?;
                self.writer.write_str(") ")?;
                self.write_use_case_body(&definition.value.body)?;
                self.writer.write_char(')')
            }
            PackageBodyElement::VerificationCaseUsage(_usage) => {
                self.write_marker(first, "verification-case-usage")
            }
            PackageBodyElement::UseCaseUsage(_usage) => self.write_marker(first, "use-case-usage"),
            PackageBodyElement::FeatureDecl(_declaration) => {
                self.write_marker(first, "feature-declaration")
            }
            PackageBodyElement::ClassifierDecl(_declaration) => {
                self.write_marker(first, "classifier-declaration")
            }
            PackageBodyElement::KermlSemanticDecl(_declaration) => {
                self.write_marker(first, "kerml-semantic-declaration")
            }
            PackageBodyElement::KermlConnector(_connector) => {
                self.write_marker(first, "kerml-connector")
            }
            PackageBodyElement::KermlRelationship(relationship) => {
                self.write_item_prefix(first)?;
                self.writer.write_str("(kerml-relationship (keyword ")?;
                self.writer.write_str(relationship.value.keyword.as_str())?;
                self.writer.write_str(") (source ")?;
                self.write_reference(relationship.value.source)?;
                self.writer.write_str(") (target ")?;
                self.write_reference(relationship.value.target)?;
                self.writer.write_str("))")
            }
            PackageBodyElement::KermlInvariant(invariant) => {
                self.write_item_prefix(first)?;
                self.writer.write_str("(kerml-invariant (negated ")?;
                self.writer.write_str(if invariant.value.is_negated {
                    "true"
                } else {
                    "false"
                })?;
                self.writer.write_str(") (name ")?;
                if invariant.value.name.is_empty() {
                    self.writer.write_str("none")?;
                } else {
                    write_quoted(self.writer, &invariant.value.name)?;
                }
                self.writer.write_str("))")
            }
            PackageBodyElement::KermlFeature(feature) => {
                self.write_item_prefix(first)?;
                self.writer.write_str("(kerml-feature (name ")?;
                self.write_usage_declaration_name(&feature.value.name)?;
                self.writer.write_str(") ")?;
                self.write_calc_def_body(&feature.value.body)?;
                self.writer.write_char(')')
            }
            PackageBodyElement::KermlClassifier(declaration) => {
                self.write_item_prefix(first)?;
                self.write_kerml_classifier(&declaration.value)
            }
            PackageBodyElement::KermlBareDeclaration(declaration) => {
                self.write_item_prefix(first)?;
                self.writer.write_str("(kerml-bare-declaration (keyword ")?;
                write_quoted(self.writer, declaration.value.keyword.as_str())?;
                self.writer.write_str(") (name ")?;
                let name = declaration
                    .value
                    .name_span
                    .as_ref()
                    .and_then(|span| self.document.source.slice(span));
                write_optional_quoted(self.writer, name)?;
                self.writer.write_str(") (multiplicity ")?;
                if let Some(multiplicity) = &declaration.value.multiplicity {
                    self.writer.write_str("(lower ")?;
                    if let Some(lower) = &multiplicity.value.lower {
                        self.write_expression(lower)?;
                    } else {
                        self.writer.write_str("unbounded")?;
                    }
                    self.writer.write_str(") (upper ")?;
                    if let Some(upper) = &multiplicity.value.upper {
                        self.write_expression(upper)?;
                    } else {
                        self.writer.write_str("unbounded")?;
                    }
                    self.writer.write_char(')')?;
                } else {
                    self.writer.write_str("none")?;
                }
                self.writer.write_str("))")
            }
            PackageBodyElement::KermlFeatureDecl(_declaration) => {
                self.write_marker(first, "kerml-feature-declaration")
            }
            PackageBodyElement::ExtendedLibraryDecl(_declaration) => {
                self.write_marker(first, "extended-library-declaration")
            }
            PackageBodyElement::AttributeUsage(_usage) => {
                self.write_marker(first, "attribute-usage")
            }
            PackageBodyElement::ItemUsage(usage) => {
                self.write_item_usage_member(first, &usage.value)
            }
            PackageBodyElement::PortUsage(usage) => {
                self.write_port_usage_member(first, &usage.value)
            }
            PackageBodyElement::ConnectionUsage(_usage) => {
                self.write_marker(first, "connection-usage")
            }
            PackageBodyElement::InterfaceUsage(usage) => {
                self.write_item_prefix(first)?;
                self.write_interface_usage(&usage.value)
            }
            PackageBodyElement::Ref(declaration) => {
                self.write_item_prefix(first)?;
                self.write_ref_declaration(&declaration.value)
            }
            PackageBodyElement::EnumerationUsage(_usage) => {
                self.write_marker(first, "enumeration-usage")
            }
            PackageBodyElement::MetadataKeywordUsage(usage) => {
                self.write_item_prefix(first)?;
                self.write_metadata_keyword_usage(&usage.value)
            }
            PackageBodyElement::ExtendedDefinition(definition) => {
                self.write_item_prefix(first)?;
                self.write_extended_definition(&definition.value)
            }
            PackageBodyElement::Connect(connect) => {
                self.write_item_prefix(first)?;
                self.write_connect(&connect.value)
            }
            PackageBodyElement::DefaultReferenceUsage(usage) => {
                self.write_item_prefix(first)?;
                self.write_default_reference_usage(&usage.value)
            }
            PackageBodyElement::AssertConstraint(_constraint) => {
                self.write_marker(first, "assert-constraint")
            }
            PackageBodyElement::PerformUsage(perform) => {
                self.write_item_prefix(first)?;
                self.write_perform(&perform.value)
            }
            PackageBodyElement::BindingConnectorUsage(_binding) => {
                self.write_marker(first, "binding-connector-usage")
            }
            PackageBodyElement::Succession(statement) => {
                self.write_item_prefix(first)?;
                self.write_first_statement(&statement.value)
            }
            PackageBodyElement::ExhibitState(exhibit) => {
                self.write_item_prefix(first)?;
                self.write_exhibit_state(&exhibit.value)
            }
            PackageBodyElement::IncludeUseCase(include) => {
                self.write_item_prefix(first)?;
                self.writer.write_str("(include (target ")?;
                self.write_reference(include.value.target)?;
                self.writer.write_str("))")
            }
        }
    }

    fn write_exhibit_state(&mut self, exhibit: &super::ExhibitState) -> io::Result<()> {
        self.writer.write_str("(exhibit (declaration ")?;
        write_quoted(self.writer, &exhibit.name)?;
        self.writer.write_str(") (state ")?;
        if let Some(reference) = exhibit.state_reference {
            self.write_reference(reference)?;
        } else {
            self.writer.write_str("none")?;
        }
        self.writer.write_str("))")
    }

    fn write_package_body(&mut self, body: &PackageBody) -> io::Result<()> {
        match body {
            PackageBody::Semicolon { .. } => self.writer.write_str("(body semicolon)"),
            PackageBody::Brace { elements, .. } => {
                let mut first = self.open_brace_body()?;
                for element in elements {
                    self.write_package_element(element, &mut first)?;
                }
                self.writer.write_char(')')
            }
        }
    }

    fn write_root(&mut self) -> io::Result<()> {
        self.writer.write_str("(root ")?;
        let mut first = true;
        for element in &self.document.root.elements {
            match &element.value {
                RootElement::Package(package) => {
                    self.write_item_prefix(&mut first)?;
                    self.write_package(&package.value)?;
                }
                RootElement::LibraryPackage(package) => {
                    self.write_item_prefix(&mut first)?;
                    self.write_library_package(&package.value)?;
                }
                RootElement::Namespace(namespace) => {
                    self.write_item_prefix(&mut first)?;
                    self.writer.write_str("(namespace (name ")?;
                    self.write_qualified_identification_name(&namespace.value.identification)?;
                    self.writer.write_str(") ")?;
                    self.write_package_body(&namespace.value.body)?;
                    self.writer.write_char(')')?;
                }
                RootElement::Import(import) => {
                    self.write_item_prefix(&mut first)?;
                    self.writer.write_str("(import ")?;
                    self.write_import_target(&import.value.target)?;
                    self.writer.write_char(')')?;
                }
                RootElement::Member(member) => {
                    self.write_package_element(member, &mut first)?;
                }
            }
        }
        self.writer.write_char(')')
    }
}

fn write_reference_definition<W: io::Write + ?Sized>(
    document: &ParsedDocument,
    writer: &mut SemanticOutput<'_, W>,
    index: usize,
    id: QualifiedReferenceId,
) -> io::Result<()> {
    let reference = document.qualified_reference(id).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("semantic AST contains dangling reference {id:?}"),
        )
    })?;
    write!(writer, "\n    (reference r{index} (scope ")?;
    writer.write_str(if reference.metadata.is_absolute {
        "absolute"
    } else {
        "relative"
    })?;
    writer.write_str(") ")?;
    write_span(writer, &reference.metadata.span)?;
    writer.write_str(" (segments")?;
    for (segment_index, segment) in reference.segments.iter().enumerate() {
        let separator = match segment.separator_before {
            None => "none",
            Some(ReferenceSeparator::ColonColon) => "colon-colon",
            Some(ReferenceSeparator::Dot) => "dot",
        };
        let authored = reference
            .segment_authored_text(segment_index)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid segment token"))?;
        let decoded = reference
            .segment_decoded_text(segment_index)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid decoded segment"))?;
        write!(writer, " (segment {segment_index} (token ")?;
        write_quoted(writer, authored)?;
        writer.write_str(") (name ")?;
        write_quoted(writer, decoded.as_ref())?;
        write!(writer, ") (separator {separator}) ")?;
        write_span(writer, &segment.source_span)?;
        writer.write_char(')')?;
    }
    writer.write_str("))")
}

fn separator_name(separator: ReferenceSeparator) -> &'static str {
    match separator {
        ReferenceSeparator::ColonColon => "colon-colon",
        ReferenceSeparator::Dot => "dot",
    }
}

fn type_check_name(kind: &TypeCheckKind) -> &'static str {
    match kind {
        TypeCheckKind::Istype => "istype",
        TypeCheckKind::Hastype => "hastype",
        TypeCheckKind::As => "as",
    }
}

fn collection_operator_name(operator: &CollectionOperator) -> &str {
    match operator {
        CollectionOperator::Collect => "collect",
        CollectionOperator::Select => "select",
        CollectionOperator::SelectOne => "selectOne",
        CollectionOperator::Size => "size",
        CollectionOperator::IsEmpty => "isEmpty",
        CollectionOperator::NotEmpty => "notEmpty",
        CollectionOperator::Includes => "includes",
        CollectionOperator::Including => "including",
        CollectionOperator::Excludes => "excludes",
        CollectionOperator::Excluding => "excluding",
        CollectionOperator::ExcludingAt => "excludingAt",
        CollectionOperator::ExcludingOnce => "excludingOnce",
        CollectionOperator::Equals => "equals",
        CollectionOperator::ForAll => "forAll",
        CollectionOperator::Exists => "exists",
        CollectionOperator::Sum => "sum",
        CollectionOperator::Sort => "sort",
        CollectionOperator::Filter => "filter",
        CollectionOperator::Reduce => "reduce",
        CollectionOperator::Other(name) => name,
    }
}

fn write_optional_quoted<W: io::Write + ?Sized>(
    writer: &mut W,
    value: Option<&str>,
) -> io::Result<()> {
    if let Some(value) = value {
        write_quoted(writer, value)
    } else {
        writer.write_str("none")
    }
}

fn write_quoted<W: io::Write + ?Sized>(writer: &mut W, value: &str) -> io::Result<()> {
    writer.write_char('"')?;
    for character in value.chars() {
        if character == '\\' {
            writer.write_str("\\\\")?;
        } else if character == '"' {
            writer.write_str("\\\"")?;
        } else if character == '\n' {
            writer.write_str("\\n")?;
        } else if character == '\r' {
            writer.write_str("\\r")?;
        } else if character == '\t' {
            writer.write_str("\\t")?;
        } else {
            writer.write_char(character)?;
        }
    }
    writer.write_char('"')
}

/// `MemberPrefix`'s `visibility = VisibilityIndicator`, or `none` when the author wrote no
/// visibility keyword. The membership's own syntax, distinct from the usage's prefix.
fn visibility_name(visibility: Option<Visibility>) -> &'static str {
    match visibility {
        Some(Visibility::Public) => "public",
        Some(Visibility::Private) => "private",
        Some(Visibility::Protected) => "protected",
        None => "none",
    }
}

fn write_span<W: io::Write + ?Sized>(
    writer: &mut SemanticOutput<'_, W>,
    span: &Span,
) -> io::Result<()> {
    let span = if writer.include_source_spans {
        span
    } else {
        &Span::dummy()
    };
    write!(
        writer,
        "(span (offset {}) (line {}) (column {}) (len {}))",
        span.offset, span.line, span.column, span.len
    )
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::WriteSemanticAst;

    struct ShortWriter {
        bytes: Vec<u8>,
        maximum_write: usize,
    }

    impl io::Write for ShortWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            let length = bytes.len().min(self.maximum_write);
            self.bytes.extend_from_slice(&bytes[..length]);
            Ok(length)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn semantic_ast_streams_through_partial_io_writer() {
        let document = crate::parse(
            r#"package Demo {
                import Domain::Types;
                use case def Analysis {
                    return part :>> Outputs::choice : Types::Item = Factory(input = Source::value);
                }
                part def Vehicle {
                    perform Actions::run :>> Actions::base {
                        in Inputs::speed = Sensors::speed;
                    }
                    connect Ports::source to Ports::sink; :> Links::base :>> Links::override;
                }
            }"#,
        )
        .expect("parse document");
        let mut writer = ShortWriter {
            bytes: Vec::new(),
            maximum_write: 3,
        };

        document
            .write_semantic_ast(&mut writer)
            .expect("stream semantic AST");

        let rendered = String::from_utf8(writer.bytes).expect("UTF-8 semantic AST");
        assert!(rendered.starts_with("(parsed-document\n  (references"));
        assert!(rendered.ends_with("\n)"));
    }
}
