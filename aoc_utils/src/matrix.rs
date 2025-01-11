use anyhow::Result;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use thiserror::Error;

use crate::{Point, data_to_grid};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Error)]
pub enum MatrixError {
    #[error("failed to parse")]
    FailedToParse,
    #[error("Point is out of range: {0}:{1} ")]
    OutOfRange(isize, isize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T: Copy + Display + TryFrom<char>> {
    pub grid: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T: Copy + Display + TryFrom<char>> Matrix<T> {
    pub fn neighbors(&self, p: &Point) -> Vec<Point> {
        p.neighbors()
            .into_iter()
            .filter(|point| self.valid_point(point))
            .collect()
    }

    #[inline]
    pub fn valid_point(&self, point: &Point) -> bool {
        point.y >= 0
            && point.y < self.height as isize
            && point.x >= 0
            && point.x < self.width as isize
    }

    /// Gets a value at position `Point`. Returns Err if the position is not valid.
    #[inline]
    pub fn get(&self, point: &Point) -> Result<T> {
        match self.valid_point(point) {
            true => Ok(self[point.y as usize][point.x as usize]),
            false => Err(MatrixError::OutOfRange(point.x, point.y).into()),
        }
    }

    /// Gets a value at position `Point`. Panics if the position is not valid.
    #[inline]
    pub fn get_unsafe(&self, point: &Point) -> T {
        match self.valid_point(point) {
            true => self[point.y as usize][point.x as usize],
            false => panic!("Out of range: [{}:{}]", point.x, point.y),
        }
    }

    /// Sets a value at position `Point`. Returns Err if the position is not valid.
    #[inline]
    pub fn set(&mut self, point: &Point, t: T) -> Result<()> {
        match self.valid_point(point) {
            true => {
                self[point.y as usize][point.x as usize] = t;
                Ok(())
            }
            false => Err(MatrixError::OutOfRange(point.x, point.y).into()),
        }
    }

    /// Sets a value at position `Point`. Panics if the position is not valid.
    #[inline]
    pub fn set_unsafe(&mut self, point: &Point, t: T) {
        match self.valid_point(point) {
            true => self[point.y as usize][point.x as usize] = t,

            false => panic!("Out of range: [{}:{}]", point.x, point.y),
        };
    }
}

impl<T: TryFrom<char> + Copy + Display> TryFrom<&str> for Matrix<T> {
    type Error = MatrixError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let grid = data_to_grid(value).map_err(|_| MatrixError::FailedToParse)?;

        let height = grid.len();
        let width = grid[0].len();
        Ok(Self {
            grid,
            height,
            width,
        })
    }
}

impl<T: TryFrom<char> + Copy + Display> FromStr for Matrix<T> {
    type Err = MatrixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Matrix::try_from(s)
    }
}

impl<T: TryFrom<char> + Copy + Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in &self.grid {
            for x in y {
                write!(f, "{}", x)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl<T: TryFrom<char> + Copy + Display> Deref for Matrix<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<T: TryFrom<char> + Copy + Display> DerefMut for Matrix<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_matrix() {
        let input = "....\n....\n....\n....\n....";
        let matrix = Matrix::<u8>::try_from(input);
        let result = Matrix {
            grid: vec![vec![b'.'; 4]; 5],
            height: 5,
            width: 4,
        };
        assert!(matrix.is_ok());
        assert_eq!(matrix.unwrap(), result);
    }

    #[test]
    fn test_u8_matrix_fromstr() {
        let input = "....\n....\n....\n....\n....";
        let matrix = input.parse::<Matrix<u8>>();
        let result = Matrix {
            grid: vec![vec![b'.'; 4]; 5],
            height: 5,
            width: 4,
        };
        assert!(matrix.is_ok());
        assert_eq!(matrix.unwrap(), result);
    }

    #[test]
    fn test_good_get() {
        let input = ".#..\n....\n....\n....\n....";
        let matrix = input
            .parse::<Matrix<u8>>()
            .expect("Matrix failed to parse!");
        let point = Point::from((1, 0));
        let result = matrix.get(&point);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b'#');
    }
    #[test]
    fn test_bad_get() {
        let input = ".#..\n....\n....\n....\n....";
        let matrix = input
            .parse::<Matrix<u8>>()
            .expect("Matrix failed to parse!");
        let point = Point::from((matrix.width + 1, matrix.height + 1));
        assert!(matrix.get(&point).is_err());
    }
}
