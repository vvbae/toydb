use std::str::FromStr;

use bigdecimal::BigDecimal;
use derive_more::Display;
use nom::{
    branch::alt,
    bytes::complete::{take_until, take_while},
    character::complete::multispace0,
    error::context,
    sequence::{preceded, terminated, tuple},
    Parser,
};
use nom_supreme::tag::complete::tag;
use serde::{Deserialize, Serialize};

use crate::parse::{peek_then_cut, Parse, ParseResult, RawSpan};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
pub enum Value {
    Number(BigDecimal), // TODO: should we make literals for ints vs floats?
    String(String),
}

/// Parse a single quoted string value
/// TODO: escaped strings
fn parse_string_value(input: RawSpan<'_>) -> ParseResult<'_, Value> {
    let (remaining, (_, str_value, _)) = context(
        "String Literal",
        tuple((
            tag("'"),
            take_until("'").map(|s: RawSpan| Value::String(s.fragment().to_string())),
            tag("'"),
        )),
    )(input)?;

    Ok((remaining, str_value))
}

/// Parse a numeric literal
/// TODO: handle floats
fn parse_number_value(input: RawSpan<'_>) -> ParseResult<'_, Value> {
    let (remaining, digits) =
        context("Number Literal", take_while(|c: char| c.is_numeric()))(input)?;

    let digits = digits.fragment();

    Ok((
        remaining,
        Value::Number(BigDecimal::from_str(digits).unwrap()),
    ))
}

/// If string (has single quote) -> parse_string_value
/// else -> parse_number_value
impl<'a> Parse<'a> for Value {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        context(
            "Value",
            preceded(
                multispace0,
                terminated(
                    alt((peek_then_cut("'", parse_string_value), parse_number_value)),
                    multispace0,
                ),
            ),
        )(input)
    }
}

impl<'a> Into<String> for Value {
    fn into(self) -> String {
        match self {
            Value::String(s) => s.to_string(),
            Value::Number(n) => n.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let expected = Value::String("123abc new".to_string());
        let expected_remaining = "fart '123'";

        let (remaining, value) = Value::parse_from_raw("'123abc new' fart '123'").unwrap();

        assert_eq!(value, expected);

        assert_eq!(remaining.fragment().to_string(), expected_remaining)
    }

    #[test]
    fn test_number() {
        let num = BigDecimal::from_str("123456").unwrap();
        let expected = Value::Number(num);

        assert_eq!(Value::parse_from_raw("123456").unwrap().1, expected)
    }
}
