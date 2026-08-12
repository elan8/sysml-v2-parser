//! Expression emission.

use super::writer::EmitWriter;
use super::EmitError;
use crate::ast::{
    Argument, BinaryOperator, CollectionOperator, CollectionOperatorBody, Expression, FeatureValue,
    FeatureValueKind, InOut, Node, TypeCheckKind, UnaryOperator,
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
        Expression::Unit(unit) => w.push_str(unit),
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
        Expression::Index { base, index } => {
            emit_expression(w, &base.value)?;
            w.push_str("#(");
            emit_expression(w, &index.value)?;
            w.push_char(')');
        }
        Expression::Bracket(inner) => {
            w.push_char('[');
            emit_expression(w, &inner.value)?;
            w.push_char(']');
        }
        Expression::LiteralWithUnit { value, unit } => {
            emit_expression(w, &value.value)?;
            w.push_char(' ');
            // `unit` is often already `Expression::Bracket(...)`; avoid `[[kg]]`.
            match &unit.value {
                Expression::Bracket(_) => emit_expression(w, &unit.value)?,
                other @ (Expression::LiteralInteger(_)
                | Expression::LiteralReal(_)
                | Expression::LiteralString(_)
                | Expression::LiteralBoolean(_)
                | Expression::Unit(_)
                | Expression::FeatureRef(_)
                | Expression::MemberAccess { .. }
                | Expression::Index { .. }
                | Expression::LiteralWithUnit { .. }
                | Expression::BinaryOp { .. }
                | Expression::UnaryOp { .. }
                | Expression::Invocation { .. }
                | Expression::Tuple(_)
                | Expression::Classification { .. }
                | Expression::MetaCast { .. }
                | Expression::TypeCheck { .. }
                | Expression::Select { .. }
                | Expression::Collect { .. }
                | Expression::Null
                | Expression::Parenthesized(_)
                | Expression::Constructor { .. }
                | Expression::FeatureChainRef(_)
                | Expression::CollectionOp { .. }
                | Expression::MetadataAccess(_)
                | Expression::Conditional { .. }
                | Expression::Extent { .. }) => {
                    w.push_char('[');
                    emit_expression(w, other)?;
                    w.push_char(']');
                }
            }
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
        Expression::Tuple(items) => {
            w.push_char('(');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    w.push_str(", ");
                }
                emit_expression(w, &item.value)?;
            }
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
        Expression::Parenthesized(inner) => {
            w.push_char('(');
            emit_expression(w, &inner.value)?;
            w.push_char(')');
        }
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
        } => {
            emit_expression(w, &base.value)?;
            w.push_str("->");
            w.push_str(collection_op_str(op));
            if let Some(body) = brace_body {
                emit_collection_operator_body(w, &body.value)?;
            } else {
                emit_args(w, args)?;
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
        Expression::MetadataAccess(base) => {
            emit_expression(w, &base.value)?;
            w.push_str(".metadata");
        }
    }
    Ok(())
}

fn emit_collection_operator_body(
    w: &mut EmitWriter<'_>,
    body: &CollectionOperatorBody,
) -> Result<(), EmitError> {
    w.push_str(" {");
    for parameter in &body.parameters {
        w.push_char(' ');
        w.push_str(match parameter.value.direction.value {
            InOut::In => "in",
            InOut::Out => "out",
            InOut::InOut => "inout",
        });
        if parameter.value.reference_keyword_span.is_some() {
            w.push_str(" ref");
        }
        w.push_char(' ');
        w.push_str(&parameter.value.name);
        if let Some(typing) = &parameter.value.typing {
            w.push_str(" : ");
            w.push_qualified_reference("collection body parameter type", typing.target)?;
        }
        w.push_char(';');
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
        match v.kind {
            FeatureValueKind::Bind => {
                // bare `default expr` or `default = expr` — prefer `=` for roundtrip of
                // explicitly-bound defaults when expression follows.
                w.push_str(" = ");
            }
            FeatureValueKind::Assign => w.push_str(" := "),
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
