//! Deserialization-time validation of the facts a serialized document cannot be trusted to carry.
//!
//! A wire document arrives as data, so accepting it is validation rather than reconstruction: every
//! qualified-reference identity has to resolve in the arena that travelled with it, and every AST
//! position that owns exact delimiter provenance has to still agree with the source it claims to
//! index. Both checks run in one pass over the [owning traversal boundary](crate::ast::visit), so a
//! new AST position that carries an identity or a delimiter span is covered the moment the
//! traversal knows about it -- there is no separate list here to keep in step.

use super::visit::{
    walk_comment_annotation, walk_first_merge_body, walk_flow_payload_clause, walk_import_target,
    walk_in_out_decl, walk_interface_end, walk_interface_part, walk_metadata_annotation,
    walk_metadata_body_usage, walk_metadata_keyword_usage, walk_occurrence_usage_prefix,
    walk_satisfy_requirement_usage, walk_usage_extension_keyword, Visitor,
};
use super::*;

fn span_end(span: &Span) -> usize {
    span.offset.saturating_add(span.len)
}

/// A span that belongs to one node rather than merely to the enclosing declaration.
///
/// [`ProvenanceValidator::owned`] checks against the innermost node the traversal is *inside*,
/// which for a node's own fields is its parent. This checks against the node itself, so
/// "the `@` of *this* annotation" is a stronger claim than "somewhere in this part def".
fn sigil_within(span: &Span, node: &Span, role: &str) -> Result<(), String> {
    if span.offset < node.offset || span_end(span) > span_end(node) {
        return Err(format!(
            "{role} at {} lies outside the node that owns it, {}..{}",
            span.offset,
            node.offset,
            span_end(node)
        ));
    }
    Ok(())
}

pub(super) fn validate_ast_provenance(document: &ParsedDocument) -> Result<(), String> {
    let mut validator = ProvenanceValidator {
        document,
        error: None,
        owners: Vec::new(),
    };
    validator.visit_root_namespace(&document.root);
    match validator.error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

struct ProvenanceValidator<'a> {
    document: &'a ParsedDocument,
    /// The first failure wins; later checks are skipped so validation reports one stable reason.
    error: Option<String>,
    /// Spans of the nodes currently being walked, outermost first. A body's delimiters have to
    /// sit inside the declaration that owns them, and only the traversal knows which that is.
    owners: Vec<Span>,
}

impl ProvenanceValidator<'_> {
    fn check(&mut self, result: Result<(), String>) {
        if self.error.is_none() {
            self.error = result.err();
        }
    }

    /// A delimiter must lie within the declaration that owns it, which is the innermost node the
    /// traversal is inside.
    fn owned(&self, span: &Span, role: &str) -> Result<(), String> {
        let Some(owner) = self.owners.last() else {
            return Ok(());
        };
        if span.offset < owner.offset || span_end(span) > span_end(owner) {
            return Err(format!(
                "{role} at {} lies outside the declaration that owns it, {}..{}",
                span.offset,
                owner.offset,
                span_end(owner)
            ));
        }
        Ok(())
    }

    /// `typed by` is two keywords with authored whitespace between them, so its span cannot be
    /// compared byte-for-byte against a fixed spelling the way a single-token delimiter can.
    fn declaration_separator(&self, span: &Span, token: &str, role: &str) -> Result<(), String> {
        let Some(text) = self.document.source.slice(span) else {
            return Err(format!(
                "{role} span at offset {} is not a valid slice of the document source",
                span.offset
            ));
        };
        let normalized: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if normalized == token {
            Ok(())
        } else {
            Err(format!("{role} span covers {text:?} rather than {token:?}"))
        }
    }

    /// A delimiter span must lie inside the source, land on character boundaries, and contain
    /// exactly the token it claims.
    fn delimiter(&self, span: &Span, token: &str, role: &str) -> Result<(), String> {
        match self.document.source.slice(span) {
            Some(text) if text == token => Ok(()),
            Some(text) => Err(format!("{role} span covers {text:?} rather than {token:?}")),
            None => Err(format!(
                "{role} span at offset {} is not a valid slice of the document source",
                span.offset
            )),
        }
    }
}

impl Visitor for ProvenanceValidator<'_> {
    fn visit_flow_payload_clause(&mut self, node: &Node<FlowPayloadClause>) {
        if self.error.is_some() {
            return;
        }
        self.check(self.delimiter(
            &node.value.of_span,
            "of",
            "flow payload clause `of` keyword",
        ));
        if self.error.is_none() {
            self.check(sigil_within(
                &node.value.of_span,
                &node.span,
                "flow payload clause `of` keyword",
            ));
        }
        walk_flow_payload_clause(self, node);
    }

    fn visit_in_out_decl(&mut self, node: &Node<InOutDecl>) {
        if self.error.is_some() {
            return;
        }
        if let Some(kind) = &node.value.kind {
            let (token, role) = match kind.value {
                InOutDeclKind::Action => ("action", "directed parameter `action` keyword"),
            };
            self.check(self.delimiter(&kind.span, token, role));
            if self.error.is_none() {
                self.check(sigil_within(&kind.span, &node.span, role));
            }
        }
        walk_in_out_decl(self, node);
    }

    /// An identity is meaningful only against the arena it was serialized with.
    fn visit_qualified_reference(&mut self, reference: &QualifiedReferenceId) {
        if self.error.is_some() {
            return;
        }
        if !self.document.qualified_references.contains(*reference) {
            self.error = Some(
                qualified_reference::QualifiedReferenceValidationError::DanglingReference {
                    id: *reference,
                }
                .to_string(),
            );
        }
    }

    fn visit_interface_part(&mut self, node: &Node<InterfacePart>) {
        if self.error.is_some() {
            return;
        }
        match &node.value {
            InterfacePart::Binary { to_span, .. } => {
                self.check(self.delimiter(to_span, "to", "interface-part to keyword"));
            }
            InterfacePart::Nary {
                open_span,
                ends,
                close_span,
            } => {
                self.check(self.delimiter(open_span, "(", "interface-part open parenthesis"));
                self.check(self.delimiter(close_span, ")", "interface-part close parenthesis"));
                for member in ends.iter().skip(1) {
                    if let Some(comma_span) = &member.comma_span {
                        self.check(self.delimiter(comma_span, ",", "interface-part comma"));
                    } else {
                        self.error =
                            Some("non-first interface endpoint has no comma span".to_owned());
                        return;
                    }
                }
            }
        }
        walk_interface_part(self, node);
    }

    fn visit_interface_end(&mut self, node: &Node<InterfaceEnd>) {
        if self.error.is_some() {
            return;
        }
        if let InterfaceEndTarget::Named { name, operator, .. } = &node.value.target {
            self.check(sigil_within(&name.span, &node.span, "interface end name"));
            let (span, token) = match operator {
                InterfaceEndReferenceOperator::Symbol { span } => (span, "::>"),
                InterfaceEndReferenceOperator::Keyword { span } => (span, "references"),
            };
            self.check(self.delimiter(span, token, "interface end reference operator"));
        }
        walk_interface_end(self, node);
    }

    fn enter_node(&mut self, span: &Span) {
        self.owners.push(*span);
    }

    fn leave_node(&mut self, _span: &Span) {
        self.owners.pop();
    }

    /// A brace body claims two delimiter tokens, and claiming them is not enough: a wire document
    /// could point both at any other `{ ... }` pair and still spell them correctly. The pair has
    /// to be the one belonging to this body -- inside the declaration that owns it, in order, and
    /// wrapped around its own members.
    fn visit_body_braces<E>(&mut self, open: &Span, elements: &[Node<E>], close: &Span) {
        if self.error.is_some() {
            return;
        }
        self.check(self.delimiter(open, "{", "body open brace"));
        self.check(self.delimiter(close, "}", "body close brace"));
        self.check(self.owned(open, "body open brace"));
        self.check(self.owned(close, "body close brace"));
        if self.error.is_some() {
            return;
        }
        if span_end(open) > close.offset {
            self.error = Some(format!(
                "body open brace at {} does not precede its close brace at {}",
                open.offset, close.offset
            ));
            return;
        }
        for element in elements {
            if element.span.offset < span_end(open) || span_end(&element.span) > close.offset {
                self.error = Some(format!(
                    "body member at {} lies outside its own delimiters {}..{}",
                    element.span.offset,
                    open.offset,
                    span_end(close)
                ));
                return;
            }
        }
    }

    /// The semicolon form claims one token; the same reasoning applies.
    fn visit_body_semicolon(&mut self, semicolon: &Span) {
        if self.error.is_some() {
            return;
        }
        self.check(self.delimiter(semicolon, ";", "body semicolon"));
        self.check(self.owned(semicolon, "body semicolon"));
    }

    /// Import targets own ordered separator, wildcard, and recursion spans that must still line up
    /// with the source text and with the reference they name.
    fn visit_import_target(&mut self, target: &ImportTarget) {
        if self.error.is_some() {
            return;
        }
        let result = target
            .validate_provenance(&self.document.source, &self.document.qualified_references)
            .map_err(|error| error.to_string());
        self.check(result);
        walk_import_target(self, target);
    }

    /// A comment's keyword span is a delimiter in everything but name. `comment` is optional in
    /// the production, and its presence is the only thing separating a member from a bare block
    /// comment, which reparses as trivia and disappears -- so emission reads this span and a wire
    /// document that points it at other text silently changes what the document says. It is
    /// checked against its own node rather than the enclosing declaration, which is what makes
    /// "the keyword of *this* comment" a stronger claim than "somewhere in this part def".
    fn visit_comment_annotation(&mut self, node: &Node<CommentAnnotation>) {
        if self.error.is_some() {
            return;
        }
        if let Some(keyword) = &node.value.keyword_span {
            self.check(self.delimiter(keyword, "comment", "comment keyword"));
            if self.error.is_none()
                && (keyword.offset < node.span.offset || span_end(keyword) > span_end(&node.span))
            {
                self.error = Some(format!(
                    "comment keyword at {} lies outside the comment that owns it, {}..{}",
                    keyword.offset,
                    node.span.offset,
                    span_end(&node.span)
                ));
                return;
            }
        }
        walk_comment_annotation(self, node);
    }

    /// The `#` sigil is syntax, not part of the reference behind it. Emission writes the sigil
    /// from this span, so a wire document that points it at other text -- or at a `#` belonging
    /// to a different member -- would change what the document says while every reference in it
    /// still resolved.
    fn visit_metadata_keyword_usage(&mut self, node: &Node<MetadataKeywordUsage>) {
        if self.error.is_some() {
            return;
        }
        self.check(self.delimiter(&node.value.hash_span, "#", "metadata keyword sigil"));
        if self.error.is_none() {
            self.check(sigil_within(
                &node.value.hash_span,
                &node.span,
                "metadata keyword sigil",
            ));
        }
        walk_metadata_keyword_usage(self, node);
    }

    /// The authored introducer and the two halves of `MetadataFeatureDeclaration`.
    ///
    /// `( ':' | 'typed' 'by' )` is an authored choice emission reproduces, so its span has to
    /// spell the keyword the variant claims; and the `OwnedFeatureTyping` span has to sit inside
    /// the annotation that owns it rather than pointing at an unrelated name elsewhere.
    fn visit_metadata_annotation(&mut self, node: &Node<MetadataAnnotation>) {
        if self.error.is_some() {
            return;
        }
        let (introducer_span, token) = match &node.value.introducer {
            crate::ast::MetadataFeatureIntroducer::At { span } => (span, "@"),
            crate::ast::MetadataFeatureIntroducer::Metadata { span } => (span, "metadata"),
        };
        self.check(self.delimiter(introducer_span, token, "metadata feature introducer"));
        if self.error.is_none() {
            self.check(sigil_within(
                introducer_span,
                &node.span,
                "metadata feature introducer",
            ));
        }
        if self.error.is_none() {
            self.check(sigil_within(
                &node.value.type_span,
                &node.span,
                "metadata annotation typing",
            ));
        }
        if let Some(declared) = &node.value.declared_name {
            if self.error.is_none() {
                let token = match declared.value.typed_by {
                    MetadataTypedBy::Colon => ":",
                    MetadataTypedBy::TypedBy => "typed by",
                };
                self.check(self.declaration_separator(
                    &declared.value.typed_by_span,
                    token,
                    "metadata declaration separator",
                ));
            }
        }
        walk_metadata_annotation(self, node);
    }

    fn visit_metadata_body_usage(&mut self, node: &Node<MetadataBodyUsage>) {
        if self.error.is_some() {
            return;
        }
        if let Some(span) = &node.value.ref_span {
            self.check(self.delimiter(span, "ref", "metadata body reference keyword"));
        }
        if let Some(operator) = &node.value.operator {
            let (span, token) = match operator {
                MetadataBodyRedefinitionOperator::ColonGreaterGreater { span } => (span, ":>>"),
                MetadataBodyRedefinitionOperator::Redefines { span } => (span, "redefines"),
            };
            self.check(self.delimiter(span, token, "metadata body redefinition operator"));
        }
        walk_metadata_body_usage(self, node);
    }

    /// `SatisfyRequirementUsage` records every keyword whose presence is a grammatical choice.
    ///
    /// `assert`, `not`, `by`, and the `requirement` keyword that selects the inline-declaration
    /// alternative are all written back from these spans, so a wire document that points one of
    /// them at other text -- or at a keyword belonging to a different member -- would change what
    /// the document says while every reference in it still resolved. `satisfy` itself is checked
    /// too: it is the token the whole usage is anchored on.
    ///
    /// The alternative/field consistency the enum cannot express is checked here as well: the
    /// declaration alternative may not carry a `by` subject identity in place of its declared
    /// name, and the ordering of the keywords has to match the production.
    fn visit_satisfy_requirement_usage(&mut self, node: &Node<SatisfyRequirementUsage>) {
        if self.error.is_some() {
            return;
        }
        let usage = &node.value;
        for (span, token, role) in [
            (
                usage.assert_span.as_ref(),
                "assert",
                "satisfy assert keyword",
            ),
            (usage.not_span.as_ref(), "not", "satisfy negation keyword"),
            (Some(&usage.satisfy_span), "satisfy", "satisfy keyword"),
        ] {
            let Some(span) = span else { continue };
            if self.error.is_none() {
                self.check(self.delimiter(span, token, role));
            }
            if self.error.is_none() {
                self.check(sigil_within(span, &node.span, role));
            }
        }
        if self.error.is_none() {
            // `OccurrenceUsagePrefix 'assert' ( isNegated ?= 'not' ) 'satisfy'` fixes the order,
            // so a document that reorders the prefixes is not a document this parser produced.
            let mut previous = node.span.offset;
            for (span, role) in [
                (usage.assert_span.as_ref(), "satisfy assert keyword"),
                (usage.not_span.as_ref(), "satisfy negation keyword"),
                (Some(&usage.satisfy_span), "satisfy keyword"),
            ] {
                let Some(span) = span else { continue };
                if span.offset < previous {
                    self.error = Some(format!(
                        "{role} at {} does not follow the prefix before it at {previous}",
                        span.offset
                    ));
                    return;
                }
                previous = span_end(span);
            }
        }
        if self.error.is_none() {
            if let SatisfiedRequirement::Declaration(declaration) = &usage.requirement {
                self.check(self.delimiter(
                    &declaration.value.keyword_span,
                    "requirement",
                    "satisfy requirement keyword",
                ));
                if self.error.is_none() {
                    self.check(sigil_within(
                        &declaration.value.keyword_span,
                        &declaration.span,
                        "satisfy requirement keyword",
                    ));
                }
            }
        }
        if self.error.is_none() {
            if let Some(subject) = &usage.subject {
                self.check(self.delimiter(&subject.value.by_span, "by", "satisfy by keyword"));
                if self.error.is_none() {
                    self.check(sigil_within(
                        &subject.value.by_span,
                        &subject.span,
                        "satisfy by keyword",
                    ));
                }
                if self.error.is_none() && subject.span.offset < span_end(&usage.satisfy_span) {
                    self.error = Some(format!(
                        "satisfy by clause at {} precedes the satisfy keyword at {}",
                        subject.span.offset, usage.satisfy_span.offset
                    ));
                    return;
                }
            }
        }
        walk_satisfy_requirement_usage(self, node);
    }

    /// `OccurrenceUsagePrefix` records one span per authored keyword, and the production fixes
    /// their order.
    ///
    /// Every component is written back from its own span, so a wire document that points one at
    /// other text -- or reorders two of them -- would change what the document says while every
    /// reference in it still resolved. The type already makes the three mutually exclusive slots
    /// (`in`/`out`/`inout`, `abstract`/`variation`, `snapshot`/`timeslice`) unrepresentable in
    /// combination; what a type cannot express, and this checks, is that each span covers the
    /// keyword its slot claims, that each lies inside the declaration that owns it, and that the
    /// authored order is
    /// `[direction] [derived] [abstract|variation] [constant] [ref] [individual] [portion] ('#' Ref)*`.
    fn visit_occurrence_usage_prefix(&mut self, prefix: &OccurrenceUsagePrefix) {
        if self.error.is_some() {
            return;
        }
        // In production order, so the same slice drives both the token check and the ordering
        // check below and the two cannot disagree about what the order is. The head is a choice:
        // `end` (then its cross feature) or the basic slots, never both.
        let mut slots: Vec<(&Span, &str, &str)> = Vec::new();
        match &prefix.head {
            OccurrenceUsagePrefixHead::End(end) => {
                slots.push((&end.end_span, "end", "occurrence prefix `end` keyword"));
                if let Some(cross) = &end.cross {
                    slots.push((&cross.span, "", "owned cross feature"));
                }
            }
            OccurrenceUsagePrefixHead::Basic {
                basic,
                individual_span,
                portion,
            } => {
                let ref_prefix = &basic.ref_prefix;
                if let Some(direction) = ref_prefix.direction.as_ref() {
                    slots.push((
                        &direction.span,
                        match direction.value {
                            InOut::In => "in",
                            InOut::Out => "out",
                            InOut::InOut => "inout",
                        },
                        "usage prefix direction keyword",
                    ));
                }
                if let Some(span) = ref_prefix.derived_span.as_ref() {
                    slots.push((span, "derived", "usage prefix `derived` keyword"));
                }
                if let Some(variance) = ref_prefix.variance.as_ref() {
                    slots.push((
                        &variance.span,
                        match variance.value {
                            DefinitionPrefix::Abstract => "abstract",
                            DefinitionPrefix::Variation => "variation",
                        },
                        "usage prefix variance keyword",
                    ));
                }
                if let Some(span) = ref_prefix.constant_span.as_ref() {
                    slots.push((span, "constant", "usage prefix `constant` keyword"));
                }
                if let Some(span) = basic.reference_span.as_ref() {
                    slots.push((span, "ref", "usage prefix `ref` keyword"));
                }
                if let Some(span) = individual_span.as_ref() {
                    slots.push((span, "individual", "occurrence prefix `individual` keyword"));
                }
                if let Some(portion) = portion.as_ref() {
                    slots.push((
                        &portion.span,
                        match portion.value {
                            OccurrencePortionKind::Snapshot => "snapshot",
                            OccurrencePortionKind::Timeslice => "timeslice",
                        },
                        "occurrence prefix portion keyword",
                    ));
                }
            }
        }
        for (span, token, role) in &slots {
            // A cross feature is a declaration, not a keyword: it is checked for ownership and
            // order here and for its own tokens when walked below.
            if !token.is_empty() && self.error.is_none() {
                self.check(self.delimiter(span, token, role));
            }
            if self.error.is_none() {
                self.check(self.owned(span, role));
            }
        }
        if self.error.is_none() {
            let mut previous: Option<(usize, &str)> = None;
            let ordered = slots.iter().map(|(span, _, role)| (*span, *role)).chain(
                prefix
                    .extension_keywords
                    .iter()
                    .map(|keyword| (&keyword.value.hash_span, "usage extension keyword")),
            );
            for (span, role) in ordered {
                if let Some((end, before)) = previous {
                    if span.offset < end {
                        self.error = Some(format!(
                            "{role} at {} does not follow the {before} before it, which ends at {end}",
                            span.offset
                        ));
                        return;
                    }
                }
                previous = Some((span_end(span), role));
            }
        }
        walk_occurrence_usage_prefix(self, prefix);
    }

    /// The `#` of a `UsageExtensionKeyword` is syntax, not part of the reference behind it.
    fn visit_usage_extension_keyword(&mut self, node: &Node<UsageExtensionKeyword>) {
        if self.error.is_some() {
            return;
        }
        self.check(self.delimiter(&node.value.hash_span, "#", "usage extension keyword sigil"));
        if self.error.is_none() {
            self.check(sigil_within(
                &node.value.hash_span,
                &node.span,
                "usage extension keyword sigil",
            ));
        }
        walk_usage_extension_keyword(self, node);
    }

    /// A `first`/`merge`/`decide`/`join`/`fork` body carries stricter rules than the delimiter
    /// checks `visit_body_braces` applies to every body; see
    /// [`validate_first_merge_body_provenance`].
    fn visit_first_merge_body(&mut self, node: &FirstMergeBody) {
        if self.error.is_some() {
            return;
        }
        let result =
            crate::ast::behavior::validate_first_merge_body_provenance(node, &self.document.source);
        self.check(result);
        walk_first_merge_body(self, node);
    }
}
