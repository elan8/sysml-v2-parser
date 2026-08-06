//! Expression emission.

use super::writer::{format_feature_path, format_name, format_qualified_name, EmitWriter};
use super::EmitError;
use crate::ast::{
    Argument, BinaryOperator, CollectionOperator, Expression, FeatureValue, FeatureValueKind, Node,
    TypeCheckKind, UnaryOperator,
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
        Expression::FeatureRef(name) => {
            if name.contains("::") {
                w.push_str(&format_qualified_name(name));
            } else if name.contains('.') {
                w.push_str(&format_feature_path(name));
            } else {
                w.push_str(&format_name(name));
            }
        }
        Expression::MemberAccess(base, member) => {
            emit_expression(w, &base.value)?;
            w.push_char('.');
            w.push_str(&format_name(member));
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
                other => {
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
            w.push_str(metaclass);
        }
        Expression::MetaCast { base, metaclass } => {
            emit_expression(w, &base.value)?;
            w.push_str(" meta ");
            w.push_str(metaclass);
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
            w.push_str(&format_qualified_name(type_name));
        }
        Expression::Select { base, selector } => {
            emit_expression(w, &base.value)?;
            w.push_str(".?");
            w.push_str(selector);
        }
        Expression::Collect { base, selector } => {
            emit_expression(w, &base.value)?;
            w.push_str(".**");
            w.push_str(selector);
        }
        Expression::Null => w.push_str("null"),
        Expression::Parenthesized(inner) => {
            w.push_char('(');
            emit_expression(w, &inner.value)?;
            w.push_char(')');
        }
        Expression::Constructor { type_name, args } => {
            w.push_str("new ");
            w.push_str(&format_qualified_name(type_name));
            emit_args(w, args)?;
        }
        Expression::FeatureChainRef(chain) => {
            for (i, seg) in chain.segments.iter().enumerate() {
                if i > 0 {
                    w.push_char('.');
                }
                w.push_str(&format_name(seg));
            }
        }
        Expression::CollectionOp { op, base, args } => {
            emit_expression(w, &base.value)?;
            w.push_str("->");
            w.push_str(collection_op_str(op));
            emit_args(w, args)?;
        }
        Expression::MetadataAccess(base) => {
            emit_expression(w, &base.value)?;
            w.push_str(".metadata");
        }
    }
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
        if let Some(name) = &arg.name {
            w.push_str(name);
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
