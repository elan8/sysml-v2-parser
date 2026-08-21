//! BNF production surface parsers: lexical terminals, empty productions, and shared grammar hooks.
//!
//! Each public entry point corresponds to a named production in the SysML/KerML textual BNF and is
//! covered by unit tests in this module. These functions exist for grammar-production traceability
//! (each names and exercises one BNF production directly) rather than because production code
//! calls them elsewhere -- production parsing composes the same underlying pieces directly. `dead_code`
//! is expected here: the `#[cfg(test)]` module below is each function's only caller.
#![allow(dead_code)]

use crate::parser::expr::expression;
use crate::parser::lex::name;
use crate::parser::usage::usage_header;
use crate::parser::Input;
use nom::combinator::opt;
use nom::IResult;
use nom::Parser;

/// EmptyFeature, EmptyUsage, EmptyMultiplicity, and similar zero-width BNF alternatives.
pub(crate) fn empty_production(input: Input<'_>) -> IResult<Input<'_>, ()> {
    Ok((input, ()))
}

/// FeatureDirection and other optional prefix fragments with no tokens.
pub(crate) fn optional_empty_prefix(input: Input<'_>) -> IResult<Input<'_>, ()> {
    empty_production(input)
}

/// OwnedExpression delegates to the full expression parser.
pub(crate) fn owned_expression(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::Node<crate::ast::Expression>> {
    expression(input)
}

/// UsageDeclaration surface: name with optional usage header fragments.
pub(crate) fn usage_declaration_surface(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, n) = name(input)?;
    let (input, _) = opt(usage_header).parse(input)?;
    Ok((input, n))
}

/// DefinitionDeclaration surface: identification with optional typing/specialization header.
pub(crate) fn definition_declaration_surface(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::Identification> {
    crate::parser::lex::identification(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span_input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    #[test]
    fn empty_productions_succeed_without_consuming() {
        let input = span_input("part x;");
        let (rest, ()) = empty_production(input).expect("EmptyFeature");
        assert_eq!(rest.location_offset(), input.location_offset());
    }

    #[test]
    fn owned_expression_parses_binary() {
        let (_, node) = owned_expression(span_input("1 + 2")).expect("OwnedExpression");
        assert!(matches!(
            node.value,
            crate::ast::Expression::BinaryOp { .. }
        ));
    }

    #[test]
    fn usage_declaration_surface_parses_header() {
        let (_, name) =
            usage_declaration_surface(span_input("wheel : Wheel ;")).expect("UsageDeclaration");
        assert_eq!(name, "wheel");
    }

    #[test]
    fn definition_declaration_surface_parses_identification() {
        let (_, id) =
            definition_declaration_surface(span_input("MyPart ;")).expect("DefinitionDeclaration");
        assert_eq!(id.name.as_deref(), Some("MyPart"));
    }

    #[test]
    fn bnf_specialization_operators() {
        use crate::parser::lex::typed_by_operator;
        use crate::parser::usage::{
            cross_subsetting, redefinition, reference_subsetting, subsetting,
        };
        let _ = typed_by_operator(span_input(": ")).expect("TypedBy");
        let _ = subsetting(span_input(":> Base ;")).expect("Subsets");
        let _ = redefinition(span_input(":>> old ;")).expect("Redefines");
        let _ = reference_subsetting(span_input("::> ref ;")).expect("References");
        let _ = cross_subsetting(span_input("=> cross ;")).expect("Crosses");
    }

    #[test]
    fn bnf_lexical_terminals() {
        use crate::parser::lex::{
            decimal_value_text, qualified_reference, string_value, ws_and_comments,
        };
        use crate::parser::usage::{feature_usage_header, multiplicity_node, typings};
        let _ = name(span_input("foo")).expect("NAME");
        let _ = string_value(span_input("'bar'")).expect("STRING_VALUE");
        let _ = decimal_value_text(span_input("42")).expect("DECIMAL_VALUE");
        let _ = qualified_reference(span_input("A::B")).expect("QualifiedName");
        let _ = ws_and_comments(span_input("  // c\n x")).expect("WHITE_SPACE");
        let _ = multiplicity_node(span_input("[1..*]")).expect("Multiplicity");
        let _ = typings(span_input(": Type ;")).expect("Typings");
        let _ = feature_usage_header(span_input(": T subsets b ;")).expect("UsagePrefix");
    }
}
