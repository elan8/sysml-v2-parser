//! Expression emission.

use super::writer::EmitWriter;
use super::EmitError;
use crate::ast::{
    Argument, BinaryOperator, CollectionOperator, CollectionOperatorBody, Expression, FeatureValue,
    FeatureValueKind, InOut, Node, SequenceExpressionList, TypeCheckKind, UnaryOperator,
};

pub(crate) fn emit_expression(w: &mut EmitWriter<'_>, expr: &Expression) -> Result<(), EmitError> {
    match expr {
        Expression::LiteralInteger(i) => w.push_str(&i.to_string()),
        Expression::LiteralReal(s) => w.push_str(s),
        Expression::LiteralString(s) => {
            w.push_char('"');
            w.push_str(s);
            w.push_char('"');
        }
        Expression::LiteralBoolean(b) => w.push_str(if *b { "true" } else { "false" }),
        Expression::FeatureRef(reference) => {
            w.push_qualified_reference("expression feature", *reference)?
        }
        Expression::MemberAccess {
            base,
            member,
            separator,
        } => {
            emit_expression(w, &base.value)?;
            w.push_str(match separator {
                crate::ast::ReferenceSeparator::ColonColon => "::",
                crate::ast::ReferenceSeparator::Dot => ".",
            });
            w.push_qualified_reference("expression member", *member)?;
        }
        Expression::Index { base, operands, .. } => {
            emit_expression(w, &base.value)?;
            w.push_str("#(");
            emit_sequence_expression_list(w, &operands.value)?;
            w.push_char(')');
        }
        Expression::Bracket { base, operands, .. } => {
            emit_expression(w, &base.value)?;
            w.push_char('[');
            emit_sequence_expression_list(w, &operands.value)?;
            w.push_char(']');
        }
        Expression::BinaryOp { op, left, right } => {
            emit_expression(w, &left.value)?;
            w.push_char(' ');
            w.push_str(binary_op_str(op));
            w.push_char(' ');
            emit_expression(w, &right.value)?;
        }
        Expression::UnaryOp { op, operand } => {
            w.push_str(unary_op_str(op));
            if matches!(op, UnaryOperator::Not) {
                w.push_char(' ');
            }
            emit_expression(w, &operand.value)?;
        }
        Expression::Invocation { callee, args } => {
            emit_expression(w, &callee.value)?;
            emit_args(w, args)?;
        }
        Expression::Sequence { operands, .. } => {
            w.push_char('(');
            emit_sequence_expression_list(w, &operands.value)?;
            w.push_char(')');
        }
        Expression::Classification { metaclass } => {
            w.push_char('@');
            w.push_qualified_reference("classification", *metaclass)?;
        }
        Expression::MetaCast { base, metaclass } => {
            emit_expression(w, &base.value)?;
            w.push_str(" meta ");
            w.push_qualified_reference("meta cast", *metaclass)?;
        }
        Expression::TypeCheck {
            kind,
            operand,
            type_name,
        } => {
            if let Some(op) = operand {
                emit_expression(w, &op.value)?;
                w.push_char(' ');
            }
            w.push_str(type_check_str(kind));
            w.push_char(' ');
            w.push_qualified_reference("type check", *type_name)?;
        }
        Expression::Select { base, selector } => {
            emit_expression(w, &base.value)?;
            w.push_str(".?");
            w.push_qualified_reference("select", *selector)?;
        }
        Expression::Collect { base, selector } => {
            emit_expression(w, &base.value)?;
            w.push_str(".**");
            w.push_qualified_reference("collect", *selector)?;
        }
        Expression::Null => w.push_str("null"),
        Expression::Constructor { type_name, args } => {
            w.push_str("new ");
            w.push_qualified_reference("constructor", *type_name)?;
            emit_args(w, args)?;
        }
        Expression::FeatureChainRef(reference) => {
            w.push_qualified_reference("feature chain", *reference)?
        }
        Expression::CollectionOp {
            op,
            base,
            args,
            brace_body,
            dot_shorthand,
        } => {
            emit_expression(w, &base.value)?;
            if *dot_shorthand {
                // KerML dot sugar: `x.{...}` (collect) / `x.?{...}` (select). Only those two
                // operators have a dot spelling; anything else could not have parsed with the
                // flag set and falls back to the arrow form.
                if matches!(op, CollectionOperator::Select) {
                    w.push_str(".?");
                } else if matches!(op, CollectionOperator::Collect) {
                    w.push_str(".");
                } else {
                    w.push_str("->");
                    w.push_str(collection_op_str(op));
                }
                if let Some(body) = brace_body {
                    emit_body_expression_bare(w, &body.value)?;
                } else {
                    emit_args(w, args)?;
                }
            } else {
                w.push_str("->");
                w.push_str(collection_op_str(op));
                if let Some(body) = brace_body {
                    emit_collection_operator_body(w, &body.value)?;
                } else {
                    emit_args(w, args)?;
                }
            }
        }
        Expression::Conditional {
            test,
            then_expr,
            else_expr,
        } => {
            w.push_str("if ");
            emit_expression(w, &test.value)?;
            w.push_str(" ? ");
            emit_expression(w, &then_expr.value)?;
            w.push_str(" else ");
            emit_expression(w, &else_expr.value)?;
        }
        Expression::Extent { target } => {
            w.push_str("all ");
            w.push_qualified_reference("extent", *target)?;
        }
        Expression::BodyExpr(body) => {
            // `emit_collection_operator_body` writes the leading space its `->op {...}` context
            // needs; a standalone body expression sits directly after the operator/keyword's own
            // spacing.
            emit_body_expression_bare(w, &body.value)?;
        }
        Expression::MetadataAccess(base) => {
            emit_expression(w, &base.value)?;
            w.push_str(".metadata");
        }
    }
    Ok(())
}

fn emit_sequence_expression_list(
    w: &mut EmitWriter<'_>,
    operands: &SequenceExpressionList,
) -> Result<(), EmitError> {
    for (index, element) in operands.elements.iter().enumerate() {
        if index > 0 {
            w.push_str(", ");
        }
        emit_expression(w, &element.expression.value)?;
    }
    if operands.trailing_comma_span.is_some() {
        w.push_char(',');
    }
    Ok(())
}

fn emit_collection_operator_body(
    w: &mut EmitWriter<'_>,
    body: &CollectionOperatorBody,
) -> Result<(), EmitError> {
    w.push_char(' ');
    emit_body_expression_bare(w, body)
}

/// Emit `{ parameters* result? }` with no leading space -- the caller supplies its own spacing.
fn emit_body_expression_bare(
    w: &mut EmitWriter<'_>,
    body: &CollectionOperatorBody,
) -> Result<(), EmitError> {
    w.push_str("{");
    if let Some(doc) = &body.doc {
        w.push_char(' ');
        super::root::emit_doc(w, &doc.value)?;
    }
    for parameter in &body.parameters {
        w.push_char(' ');
        if let Some(direction) = &parameter.value.direction {
            w.push_str(match direction.value {
                InOut::In => "in",
                InOut::Out => "out",
                InOut::InOut => "inout",
            });
            w.push_char(' ');
        }
        if parameter.value.reference_keyword_span.is_some() {
            w.push_str("ref ");
        }
        w.push_str(&parameter.value.name);
        if let Some(typing) = &parameter.value.typing {
            w.push_str(" : ");
            w.push_qualified_reference("collection body parameter type", typing.target)?;
        }
        match &parameter.value.terminator {
            crate::ast::CollectionOperatorParameterTerminator::Semicolon { .. } => {
                w.push_char(';');
            }
            crate::ast::CollectionOperatorParameterTerminator::Body { doc, .. } => {
                w.push_str(" {");
                if let Some(doc) = doc {
                    w.push_char(' ');
                    super::root::emit_doc(w, &doc.value)?;
                }
                w.push_str(" }");
            }
        }
    }
    if let Some(result) = &body.result {
        w.push_char(' ');
        emit_expression(w, &result.value)?;
    }
    w.push_str(" }");
    Ok(())
}

pub(crate) fn emit_feature_value(
    w: &mut EmitWriter<'_>,
    value: &Node<FeatureValue>,
) -> Result<(), EmitError> {
    let v = &value.value;
    if v.is_default {
        w.push_str(" default");
        if v.has_operator {
            match v.kind {
                FeatureValueKind::Bind => w.push_str(" = "),
                FeatureValueKind::Assign => w.push_str(" := "),
            }
        } else {
            // Bare `default expr` / `default {expr}`: no operator was authored, so none is
            // emitted.
            w.push_char(' ');
        }
    } else {
        match v.kind {
            FeatureValueKind::Bind => w.push_str(" = "),
            FeatureValueKind::Assign => w.push_str(" := "),
        }
    }
    emit_expression(w, &v.expression.value)
}

fn emit_args(w: &mut EmitWriter<'_>, args: &[Argument]) -> Result<(), EmitError> {
    w.push_char('(');
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            w.push_str(", ");
        }
        if let Some(parameter) = arg.parameter {
            w.push_qualified_reference("argument parameter", parameter)?;
            w.push_str(" = ");
        }
        emit_expression(w, &arg.value.value)?;
    }
    w.push_char(')');
    Ok(())
}

fn binary_op_str(op: &BinaryOperator) -> &str {
    op.as_str()
}

fn unary_op_str(op: &UnaryOperator) -> &str {
    op.as_str()
}

fn collection_op_str(op: &CollectionOperator) -> &str {
    op.as_str()
}

fn type_check_str(kind: &TypeCheckKind) -> &'static str {
    match kind {
        TypeCheckKind::Istype => "istype",
        TypeCheckKind::Hastype => "hastype",
        TypeCheckKind::As => "as",
    }
}
