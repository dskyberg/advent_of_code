//! Nom is great!  But it helps to have a few starting functions.
use std::str::FromStr;

use nom::{
    IResult,
    character::complete::{digit1, one_of},
    combinator::{map_res, opt, recognize},
    sequence::tuple,
};

/// Parse a simple number, such as "123".
/// Returns a str.
/// Example:
/// ```
/// let (_, result) = parsed_number::<usize>("123").expect("failed to parse");
/// assert_eq!(result, "123");
/// ```
pub fn number(input: &str) -> IResult<&str, &str> {
    recognize(digit1)(input)
}

/// Parse a simple number, such as "123", to type T.
/// Returns T
/// Example:
/// ```
/// let (_, result) = parsed_number::<usize>("123").expect("failed to parse");
/// assert_eq!(result, 123usize);
/// ```
pub fn parsed_number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(number, str::parse)(input)
}

/// Parse a signed integer.
/// Returns a str.
/// Example:
/// ```
/// let (_, result) = integer::<usize>("-123").expect("failed to parse");
/// assert_eq!(result, "-123");
/// ```
pub fn integer(input: &str) -> IResult<&str, &str> {
    recognize(tuple((opt(one_of("+-")), number)))(input)
}

/// Parse a signed integer, such as "-123", to type T.
/// Returns T
/// Example:
/// ```
/// let (_, result) = parsed_integer::<isize>("-123").expect("failed to parse");
/// assert_eq!(result, -123isize);
/// ```
pub fn parsed_integer<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(integer, str::parse)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let (_, result) = number("123").expect("failed to parse");
        assert_eq!(result, "123");
    }

    #[test]
    fn test_parsed_number() {
        let (_, result) = parsed_number::<u32>("123").expect("failed to parse");
        assert_eq!(result, 123);
    }

    #[test]
    fn test_integer() {
        let (_, result) = integer("-10").expect("Failed to parse");
        assert_eq!(result, "-10");
    }

    #[test]
    fn test_parsed_integer() {
        let (_, result) = parsed_integer::<i64>("-10").expect("failed");
        assert_eq!(result, -10);
    }
}
