//! Import and relationship body parsing.

use crate::ast::{Import, Visibility};
use crate::parser::lex::{qualified_name, skip_until_brace_end, ws1, ws_and_comments};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

/// RelationshipBody: ';' or '{' ... '}'. For '{' we skip content until matching '}'.
pub(crate) fn relationship_body(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| ()),
        map(
            delimited(
                tag("{"),
                skip_until_brace_end,
                preceded(ws_and_comments, tag("}")),
            ),
            |_| (),
        ),
    ))(input)
}

/// Import: visibility? 'import' isImportAll? (QualifiedName | QualifiedName '::' '*') RelationshipBody
pub(crate) fn import_(input: &[u8]) -> IResult<&[u8], Import> {
    let (input, _) = ws_and_comments(input)?;
    let (input, visibility) = opt(alt((
        map(preceded(tag("public"), ws1), |_| Visibility::Public),
        map(preceded(tag("private"), ws1), |_| Visibility::Private),
        map(preceded(tag("protected"), ws1), |_| Visibility::Protected),
    )))(input)?;
    let (input, _) = tag("import")(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = opt(preceded(tag("all"), ws1))(input)?;
    let (input, (target, is_import_all)) = alt((
        map(
            tuple((
                qualified_name,
                preceded(ws_and_comments, tag("::")),
                preceded(ws_and_comments, tag("*")),
            )),
            |(q, _, _)| (format!("{}::*", q), true),
        ),
        map(qualified_name, |q| (q, false)),
    ))(input)?;
    let (input, _) = relationship_body(input)?;
    Ok((
        input,
        Import {
            visibility,
            is_import_all,
            target,
        },
    ))
}
