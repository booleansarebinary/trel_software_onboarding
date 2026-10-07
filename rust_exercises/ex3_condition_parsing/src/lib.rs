//! Parsing abort conditions from text.
//!
//! Aborts are configured, not hardcoded. Somewhere upstream, a person writes
//! "trip if ai_12 goes above 450 psi" into a config file, and the ground
//! software has to turn that text into something it can evaluate every cycle:
//!
//! ```text
//! "ai_12 > 450.0"   ->   Condition { lhs: Channel(12), comparison: GreaterThan, rhs: Constant(450.0) }
//! ```
//!
//! A config typo should be caught when the config is loaded, with an error that
//! says what was wrong. Not mid-hotfire, and not by quietly guessing.
//!
//! # Rust notes for this file
//!
//! - `FromStr` is the standard trait for "build this type from a string". Once
//!   a type implements it, `"...".parse::<ThatType>()` works, exactly like
//!   `"42".parse::<u16>()`. `Comparison` below is a finished example.
//! - Enums can carry data. `Operand::Channel(12)` and `Operand::Constant(450.0)`
//!   are both `Operand`s, and `match` tells you which one you have.
//! - `ParseError` is our own error enum. Each variant is one way parsing can
//!   fail, so a caller (or a test) can tell exactly what went wrong.
//! - No regex here. For a grammar this small, splitting on whitespace and
//!   matching on the pieces is shorter and gives better error messages.
//!
//! Your job is one function, `parse_operand`. Everything else is finished.

use std::fmt;
use std::str::FromStr;

/// Identifies an analog input channel. `ai_12` is channel 12.
pub type ChannelId = u16;

/// Why a condition could not be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// A condition is exactly three space-separated tokens:
    /// `<operand> <comparison> <operand>`. This holds how many there were.
    WrongTokenCount(usize),
    /// The middle token was not one of `>`, `<`, `>=`, `<=`.
    UnknownComparison(String),
    /// A token was neither a channel like `ai_12` nor a finite number.
    BadOperand(String),
}

/// How the two sides of a condition are compared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    GreaterThan,
    LessThan,
    AtLeast,
    AtMost,
}

/// A worked example of `FromStr`. Read this before writing the one for
/// `Condition` - yours has the same shape.
impl FromStr for Comparison {
    /// The error type `parse` returns when this fails.
    type Err = ParseError;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        match token {
            ">" => Ok(Self::GreaterThan),
            "<" => Ok(Self::LessThan),
            ">=" => Ok(Self::AtLeast),
            "<=" => Ok(Self::AtMost),
            other => Err(ParseError::UnknownComparison(other.to_string())),
        }
    }
}

/// `Display` is the reverse of `FromStr`: it is what `format!("{}", x)` and
/// `x.to_string()` use.
impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::GreaterThan => ">",
            Self::LessThan => "<",
            Self::AtLeast => ">=",
            Self::AtMost => "<=",
        };
        write!(f, "{symbol}")
    }
}

/// One side of a condition: a live channel reading, or a fixed number.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operand {
    Channel(ChannelId),
    Constant(f64),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Channel(id) => write!(f, "ai_{id}"),
            Self::Constant(value) => write!(f, "{value}"),
        }
    }
}

/// "`lhs` <comparison> `rhs`", for example "ai_12 > 450.0".
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Condition {
    pub lhs: Operand,
    pub comparison: Comparison,
    pub rhs: Operand,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.comparison, self.rhs)
    }
}

/// Parses one operand token.
///
/// - `ai_<n>` is a channel, where `<n>` must fit in a `ChannelId`.
///   `ai_12` is `Channel(12)`.
/// - Anything else must be a number: `450`, `450.0`, `-3.5`.
/// - Everything else is a `BadOperand` holding the token.
///
/// One trap: Rust happily parses `"NaN"` and `"inf"` as `f64`. Reject them.
/// Every comparison against NaN is false, so a NaN threshold is an abort that
/// can never fire - and nobody finds out until the day it needed to.
///
/// TODO(you): implement.
pub fn parse_operand(token: &str) -> Result<Operand, ParseError> {
    // This line only exists so the unimplemented stub compiles without
    // "unused" errors. Delete it when you implement the function.
    let _ = token;
    todo!("ex3: implement parse_operand")
}

/// Parses a whole condition, like `"ai_12 > 450.0"`.
///
/// Tokens are separated by whitespace, and there must be exactly three of
/// them. `"ai_12>450"` is one token, so it is a `WrongTokenCount(1)`.
///
/// When something is wrong, return the error for the FIRST problem, reading
/// left to right.
///
/// Finished, like `Comparison` above. Read it: it is your `parse_operand`'s
/// only caller, and it shows two things worth knowing - matching on the shape
/// of a slice, and `?` passing your errors straight through.
impl FromStr for Condition {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = text.split_whitespace().collect();
        let [lhs, comparison, rhs] = tokens.as_slice() else {
            return Err(ParseError::WrongTokenCount(tokens.len()));
        };

        Ok(Condition {
            lhs: parse_operand(lhs)?,
            comparison: comparison.parse()?,
            rhs: parse_operand(rhs)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(">", Comparison::GreaterThan)]
    #[case("<", Comparison::LessThan)]
    #[case(">=", Comparison::AtLeast)]
    #[case("<=", Comparison::AtMost)]
    fn test_comparison_from_str_parses_each_symbol(
        #[case] token: &str,
        #[case] expected: Comparison,
    ) {
        let parsed = token.parse::<Comparison>();

        assert_eq!(parsed, Ok(expected));
    }

    #[rstest]
    fn test_comparison_from_str_returns_unknown_comparison_when_the_symbol_is_not_supported() {
        let token = "=>";

        let parsed = token.parse::<Comparison>();

        assert_eq!(
            parsed,
            Err(ParseError::UnknownComparison(String::from("=>")))
        );
    }

    #[rstest]
    fn test_condition_from_str_parses_a_channel_against_a_constant() {
        let text = "ai_12 > 450.0";

        let parsed = text.parse::<Condition>();

        assert_eq!(
            parsed,
            Ok(Condition {
                lhs: Operand::Channel(12),
                comparison: Comparison::GreaterThan,
                rhs: Operand::Constant(450.0),
            })
        );
    }

    #[rstest]
    #[case("", 0)]
    #[case("ai_12>450", 1)]
    #[case("ai_12 > 450 psi", 4)]
    fn test_condition_from_str_returns_wrong_token_count_when_there_are_not_three_tokens(
        #[case] text: &str,
        #[case] count: usize,
    ) {
        let parsed = text.parse::<Condition>();

        assert_eq!(parsed, Err(ParseError::WrongTokenCount(count)));
    }

    // ------------------------------------------------------------------
    // TODO(you): `parse_operand` has one test so far, through `from_str`.
    // Write these, in the same style as the tests above:
    //
    //   1. `parse_operand` accepts channels and numbers. Use `#[case]` for at
    //      least: "ai_0", "ai_65535" (the largest ChannelId), a whole number,
    //      and a negative decimal.
    //   2. `parse_operand` returns `BadOperand` for anything else. Use
    //      `#[case]` for at least: "ai_" (no number), "ai_65536" (too big),
    //      "pressure", "NaN", and "inf".
    //   3. `Condition::from_str` returns `BadOperand` for "ai_12 > NaN". This
    //      one checks your function and the finished parser work together.
    //
    // Sabotage check before you open the PR: delete your NaN/inf check. If no
    // test fails, tests 2 and 3 are not doing their job. Put it back.
    // ------------------------------------------------------------------
}
