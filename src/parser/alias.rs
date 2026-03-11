//! Alias definition parsing.

use crate::ast::{AliasBody, AliasDef};
use crate::parser::lex::{identification, qualified_name, skip_until_brace_end, ws1, ws_and_comments};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

/// Alias body: `;` or `{` ... `}`
fn alias_body(input: &[u8]) -> IResult<&[u8], AliasBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| AliasBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag("{"),
                skip_until_brace_end,
                preceded(ws_and_comments, tag("}")),
            ),
            |_| AliasBody::Brace,
        ),
    ))(input)
}

/// Alias definition: `alias` Identification `for` qualified_name body
pub(crate) fn alias_def(input: &[u8]) -> IResult<&[u8], AliasDef> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("alias")(input)?;
    let (input, _) = ws1(input)?;
    let (input, identification) = identification(input)?;
    let (input, _) = preceded(ws_and_comments, tag("for"))(input)?;
    let (input, target) = preceded(ws1, qualified_name)(input)?;
    let (input, body) = alias_body(input)?;
    Ok((
        input,
        AliasDef {
            identification,
            target,
            body,
        },
    ))
}
