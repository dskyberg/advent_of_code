//! Text to number conversion methods
use anyhow::Result;
use std::str::pattern::Pattern;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("Failed to convert {0}")]
    Failed(char),
    #[error("Failed to convert from string")]
    NumberFromStr,
    #[error("Failed to parse data to type T")]
    TryFromFailed,
}

/// Convert from char to scalar type T (such as u8, u32, i32 etc).
/// Returns an error if the char cannot be converted to a number.
///
/// Example:
///
/// ```
/// use aoc_utils::*;
///
/// assert_eq!(c_to_num::<u32>('0').unwrap(), 0u32);
/// ```
///
/// # Errors
/// `ConvertError::Failed` is returned on failure
///
pub fn c_to_num<T: std::convert::From<u8>>(c: char) -> Result<T> {
    let result = c.to_digit(10).ok_or(ConvertError::Failed(c))? as u8;
    Ok(result.into())
}

/// Convert a string of digits to a `Vec<T>` (such as `Vec<u8>`, `Vec<u32>`, `Vec<i32>`, etc.).
///
/// # Example
/// ```
/// use aoc_utils::*;
///
/// assert_eq!(line_to_digits::<u32>("123").unwrap(), vec![1,2,3])
/// ```
///
/// # Errors
/// `ConvertError::Failed` is returned on failure
pub fn line_to_digits<T: std::convert::From<u8>>(line: &str) -> Result<Vec<T>> {
    line.chars().map(c_to_num).collect()
}

/// Convert a string of digits to a `Vec<T>` (such as `Vec<u8>`, `Vec<u32>`, `Vec<i32>`, etc.).
/// without error checking.
///
/// # Example
/// ```
/// use aoc_utils::*;
///
/// assert_eq!(unsafe_line_to_digits::<u32>("123"), vec![1,2,3])
/// ```
///
/// # Panics
///
/// Panics if any character in the string is not a digit.
pub fn unsafe_line_to_digits<T: std::convert::From<u8>>(line: &str) -> Vec<T> {
    line.chars()
        .map(|c| (c.to_digit(10).unwrap() as u8).into())
        .collect()
}

/// Convert a string with multiple lines (delimited by `\n`) to `Vec<Vec<T>>`.
///
/// # Example
///
/// ```
/// use aoc_utils::*;
///
/// let input = r#"123
/// 456
/// 789"#;
///
/// assert_eq!(data_to_digits::<u32>(input).unwrap(), vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]]);
/// ```
///
/// # Errors
/// `ConvertError::Failed` is returned on failure
pub fn data_to_digits<T: std::convert::From<u8>>(data: &str) -> Result<Vec<Vec<T>>> {
    data.lines().map(line_to_digits).collect()
}

/// Convert a string with multiple lines (delimited by `\n`) to `Vec<Vec<T>>`,
/// without error checking.
///
/// # Example
///
/// ```
/// use aoc_utils::*;
///
/// let input = r#"123
/// 456
/// 789"#;
///
/// assert_eq!(unsafe_data_to_digits::<u32>(input), vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]]);
/// ```
/// # Panics
///
/// Panics if any character in the string is not a digit.
pub fn unsafe_data_to_digits<T: std::convert::From<u8>>(data: &str) -> Vec<Vec<T>> {
    data.lines().map(unsafe_line_to_digits).collect()
}

/// Splits a line on `pat` and parses each into T
///
/// # Example
///
/// ```
/// use aoc_utils::*;
///
/// let line = "  1    2 3";
/// assert_eq!(vec![1, 2, 3], line_to_numbers::<u32>(line, ' ').unwrap());
/// ```
///
/// # Errors
/// `ConvertError::NumberFromStr` if parsing failes
pub fn line_to_numbers<T: std::str::FromStr>(line: &str, pat: impl Pattern) -> Result<Vec<T>> {
    line.trim()
        .split(pat)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.trim()
                .parse::<T>()
                .map_err(|_| ConvertError::NumberFromStr.into())
        })
        .collect::<Vec<Result<T>>>()
        .into_iter()
        .collect()
}

/// Converts random data to a `Vec<Vec<T>>`.
///
/// # Example
///
/// ```
/// use aoc_utils::*;
///
/// let input = r#"######
/// #..#.#
/// #######"#;
///
/// assert_eq!(data_to_grid::<u8>(input).unwrap(), vec![
///  vec![b'#',b'#',b'#',b'#',b'#',b'#'],
///  vec![b'#',b'.',b'.',b'#',b'.',b'#'],
///  vec![b'#',b'#',b'#',b'#',b'#',b'#']]);
/// ```
/// # Errors
/// `ConvertError::TryFromFailed` is returned if any value cannot be parsed
pub fn data_to_grid<T: std::convert::TryFrom<char>>(data: &str) -> Result<Vec<Vec<T>>> {
    data.lines()
        .map(|c| {
            c.chars()
                .map(|c| T::try_from(c))
                .collect::<Vec<Result<T, _>>>()
                .into_iter()
                .collect::<Result<Vec<T>, _>>()
        })
        .collect::<Result<Vec<Vec<T>>, _>>()
        .map_err(|_| ConvertError::TryFromFailed.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        let input = "123\n456";
        println!("{:?}", data_to_digits::<u32>(input));
    }

    #[test]
    fn test_line_to_numbers() {
        let line = "  1    2 3";
        assert_eq!(vec![1, 2, 3], line_to_numbers(line, ' ').expect("oops"));
    }

    #[test]
    fn test_data_to_u8_grid() {
        let input = "....\n....\n....\n....";
        let grid = vec![vec![b'.'; 4]; 4];
        let result = data_to_grid::<u8>(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grid);
    }

    #[test]
    fn test_data_to_char_grid() {
        let input = "....\n....\n....\n....";
        let grid = vec![vec!['.'; 4]; 4];
        let result = data_to_grid::<char>(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grid);
    }

    #[test]
    fn test_data_to_i32_grid() {
        let input = "....\n....\n....\n....";
        let grid = vec![vec![46; 4]; 4];
        let result = data_to_grid::<u32>(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), grid);
    }
}
