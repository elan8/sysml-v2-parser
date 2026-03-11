//! Expression and path parsing for values and bind/connect.

use crate::ast::Expression;
use crate::parser::lex::{name, ws_and_comments};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, opt};
use nom::sequence::{delimited, preceded};
use nom::IResult;

/// Integer literal.
fn literal_integer(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, _) = ws_and_comments(input)?;
    let (input, sign) = opt(alt((tag("-"), tag("+"))))(input)?;
    let (input, digits) = digit1(input)?;
    let s = String::from_utf8_lossy(digits);
    let n: i64 = s.parse().unwrap_or(0);
    let n = if sign == Some(&b"-"[..]) { -n } else { n };
    Ok((input, Expression::LiteralInteger(n)))
}

/// Real literal (simple: digits.digits).
fn literal_real(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, _) = ws_and_comments(input)?;
    let (input, whole) = digit1(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, frac) = digit1(input)?;
    let s = format!(
        "{}.{}",
        String::from_utf8_lossy(whole),
        String::from_utf8_lossy(frac)
    );
    Ok((input, Expression::LiteralReal(s)))
}

/// String literal: double-quoted.
fn literal_string(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("\"")(input)?;
    let mut i = 0;
    let input_slice = input;
    while i < input_slice.len() {
        if input_slice[i] == b'\\' && i + 1 < input_slice.len() {
            i += 2;
            continue;
        }
        if input_slice[i] == b'"' {
            let s = String::from_utf8_lossy(&input_slice[..i]).into_owned();
            let s = s.replace("\\\"", "\"");
            return Ok((&input_slice[i + 1..], Expression::LiteralString(s)));
        }
        i += 1;
    }
    let s = String::from_utf8_lossy(input_slice).into_owned();
    Ok((&input_slice[input_slice.len()..], Expression::LiteralString(s)))
}

/// Boolean literal: true | false.
fn literal_boolean(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, _) = ws_and_comments(input)?;
    let (input, v) = alt((
        map(tag("true"), |_| true),
        map(tag("false"), |_| false),
    ))(input)?;
    Ok((input, Expression::LiteralBoolean(v)))
}

/// Feature reference: single name (for expression context we use name, not qualified_name, so we don't consume ::).
fn feature_ref_primary(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, _) = ws_and_comments(input)?;
    let (input, n) = name(input)?;
    Ok((input, Expression::FeatureRef(n)))
}

/// Literal only (no unit): integer, real, string, boolean.
fn literal_only(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        literal_boolean,
        literal_integer,
        literal_real,
        literal_string,
    ))(input)
}

/// Literal with optional [ unit ]: 1750 [kg] -> LiteralWithUnit(LiteralInteger(1750), Bracket(FeatureRef("kg"))).
fn literal_with_unit(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, value) = literal_only(input)?;
    let (input, _) = ws_and_comments(input)?;
    if !input.starts_with(b"[") {
        return Ok((input, value));
    }
    let (input, _) = tag("[")(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, unit_name) = name(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("]")(input)?;
    let unit = Expression::Bracket(Box::new(Expression::FeatureRef(unit_name)));
    let expr = Expression::LiteralWithUnit {
        value: Box::new(value),
        unit: Box::new(unit),
    };
    Ok((input, expr))
}

/// Parenthesized expression: ( expression ).
fn parenthesized(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, _) = ws_and_comments(input)?;
    delimited(
        tag("("),
        preceded(ws_and_comments, expression),
        preceded(ws_and_comments, tag(")")),
    )(input)
}

/// Primary expression: literal with unit, literal only, feature ref, or parenthesized.
fn primary(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        literal_with_unit,
        literal_only,
        feature_ref_primary,
        parenthesized,
    ))(input)
}

/// Apply postfix #( expr ) or . name to an expression.
fn postfix(input: &[u8], mut expr: Expression) -> IResult<&[u8], Expression> {
    let (input, _) = ws_and_comments(input)?;
    if input.starts_with(b"#") {
        let (input, _) = tag("#")(input)?;
        let (input, _) = preceded(ws_and_comments, tag("("))(input)?;
        let (input, index_expr) = preceded(ws_and_comments, expression)(input)?;
        let (input, _) = preceded(ws_and_comments, tag(")"))(input)?;
        expr = Expression::Index {
            base: Box::new(expr),
            index: Box::new(index_expr),
        };
        return postfix(input, expr);
    }
    if input.starts_with(b".") {
        let (input, _) = tag(".")(input)?;
        let (input, _) = ws_and_comments(input)?;
        let (input, member) = name(input)?;
        expr = Expression::MemberAccess(Box::new(expr), member);
        return postfix(input, expr);
    }
    Ok((input, expr))
}

/// Full expression: primary then zero or more postfix (#(expr), .name).
pub(crate) fn expression(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, expr) = primary(input)?;
    postfix(input, expr)
}

/// Path expression: name or name.name.name... (for bind/connect). Returns Expression (FeatureRef or MemberAccess chain).
pub(crate) fn path_expression(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, _) = ws_and_comments(input)?;
    let (input, first) = name(input)?;
    let mut expr = Expression::FeatureRef(first);
    let mut rest = input;
    loop {
        let (next, _) = ws_and_comments(rest)?;
        if !next.starts_with(b".") {
            break;
        }
        let (next, _) = tag(".")(next)?;
        let (next, _) = ws_and_comments(next)?;
        let (next, member) = name(next)?;
        expr = Expression::MemberAccess(Box::new(expr), member);
        rest = next;
    }
    Ok((rest, expr))
}
