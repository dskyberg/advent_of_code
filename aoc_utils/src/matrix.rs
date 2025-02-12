//! Create 2D matrix of any scalar type (u8, u32, etc.)
use anyhow::Result;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use thiserror::Error;

use crate::{Point, data_to_grid};

pub trait ItemTrait = TryFrom<char> + Copy + Display + PartialEq;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Error)]
pub enum MatrixError {
    #[error("failed to parse")]
    FailedToParse,
    #[error("Point is out of range: {0}:{1} ")]
    OutOfRange(isize, isize),
    #[error("Cannot create a matrixfrom empty vec")]
    EmptyVec,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T: ItemTrait> {
    pub grid: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

/// Returns the valid neighbors for the given point.
impl<T: ItemTrait> Matrix<T> {
    pub fn neighbors(&self, p: &Point) -> Vec<Point> {
        p.neighbors()
            .into_iter()
            .filter(|point| self.valid_point(point))
            .collect()
    }

    /// Tests to ensure the point is within the matrix space.
    #[inline]
    pub fn valid_point(&self, point: &Point) -> bool {
        point.y >= 0
            && point.y < self.height as isize
            && point.x >= 0
            && point.x < self.width as isize
    }

    /// Tests whether the point lies on a boundary of the matrix
    #[inline]
    pub fn is_edge(&self, p: &Point) -> bool {
        (p.x == 0 || p.x == self.width as isize - 1)
            && (p.y == 0 || p.y == self.height as isize - 1)
    }

    /// Gets a value at position `Point`. Returns Err if the position is not valid.
    ///
    /// # Errors
    /// `MatrixError::OutOfRange` if the point is not valid
    #[inline]
    pub fn get(&self, point: &Point) -> Result<T> {
        let x = *self
            .grid
            .get(point.y as usize)
            .and_then(|row| row.get(point.x as usize))
            .ok_or(MatrixError::OutOfRange(point.x, point.y))?;
        Ok(x)
    }

    /// Gets a value at position `Point`.
    ///
    /// # Panics
    /// Panics if the position is not valid.
    #[inline]
    pub fn get_unsafe(&self, point: &Point) -> T {
        *self
            .grid
            .get(point.y as usize)
            .and_then(|row| row.get(point.x as usize))
            .unwrap()
    }

    /// Sets a value at position `Point`. Returns Err if the position is not valid.
    ///
    /// # Errors
    /// `MatrixError::OutOfRange` if the point is not valid
    #[inline]
    pub fn set(&mut self, point: &Point, t: T) -> Result<()> {
        match self.valid_point(point) {
            true => {
                self.set_unsafe(point, t);
                Ok(())
            }
            false => Err(MatrixError::OutOfRange(point.x, point.y).into()),
        }
    }

    /// Sets a value at position `Point`. Panics if the position is not valid.
    ///
    /// # Panics
    /// Panics if the point is not valid
    #[inline]
    pub fn set_unsafe(&mut self, point: &Point, t: T) {
        self[point.y as usize][point.x as usize] = t;
    }

    /// Iterate the entire matrix.
    pub fn iter(&self) -> MatrixIterator<T> {
        MatrixIterator {
            matrix: self,
            index: 0,
        }
    }

    #[inline]
    /// Calculates the (x,y) point offset for the index in the form of:
    /// - y: index / self.width
    /// - x: index % self.width
    ///
    /// This is the reciprocal of [Matrix::iter()]
    pub fn index2point(&self, index: usize) -> Option<Point> {
        if index >= self.width * self.height {
            return None;
        }
        Some(Point {
            x: (index % self.width) as isize,
            y: (index / self.width) as isize,
        })
    }

    /// Find all points in the matrix containing `input`
    pub fn find(&self, input: &T) -> Vec<Point> {
        self.iter()
            .filter(|(_, t)| t == input)
            .map(|(p, _)| p)
            .collect()
    }

    /// Find the first point containing `input`, if it exists.
    pub fn find_one(&self, input: &T) -> Option<Point> {
        self.find(input).first().cloned()
    }

    /// Swap the values of two points on the mtrix
    ///
    /// # Errors
    /// MatrixError::OutOfRange if either point is invalid.
    pub fn swap(&mut self, lhs: &Point, rhs: &Point) -> Result<()> {
        if !self.valid_point(lhs) {
            return Err(MatrixError::OutOfRange(lhs.x, lhs.y).into());
        }
        if !self.valid_point(rhs) {
            return Err(MatrixError::OutOfRange(rhs.x, rhs.y).into());
        }
        self.swap_unsafe(lhs, rhs);
        Ok(())
    }

    /// Swap the values of two points on the mtrix
    ///
    /// # Panics
    /// Panics if either point is invalid
    pub fn swap_unsafe(&mut self, lhs: &Point, rhs: &Point) {
        let hold = self.get_unsafe(lhs);
        self.set_unsafe(lhs, self.get_unsafe(rhs));
        self.set_unsafe(rhs, hold);
    }
}

impl<T: ItemTrait> TryFrom<&str> for Matrix<T> {
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

impl<T: ItemTrait> TryFrom<Vec<Vec<T>>> for Matrix<T> {
    type Error = MatrixError;

    fn try_from(grid: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let height = grid.len();
        let width = grid[0].len();
        if height == 0 || width == 0 {
            return Err(MatrixError::EmptyVec);
        }

        Ok(Self {
            grid,
            height,
            width,
        })
    }
}

impl<T: ItemTrait> FromStr for Matrix<T> {
    type Err = MatrixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Matrix::try_from(s)
    }
}

impl<T: ItemTrait> Display for Matrix<T> {
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

impl<T: ItemTrait> Deref for Matrix<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<T: ItemTrait> DerefMut for Matrix<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

pub struct MatrixIterator<'a, T: ItemTrait> {
    matrix: &'a Matrix<T>,
    index: usize,
}

impl<'a, T: ItemTrait> IntoIterator for &'a Matrix<T> {
    type Item = (Point, T);
    type IntoIter = MatrixIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIterator {
            matrix: self,
            index: 0,
        }
    }
}

#[allow(clippy::needless_lifetimes)]
/// Iterates the matrix, returning (Point, T) for each iteration
impl<'a, T: ItemTrait> Iterator for MatrixIterator<'a, T> {
    type Item = (Point, T);

    fn next(&mut self) -> Option<Self::Item> {
        let y = self.index / self.matrix.width;
        let x = self.index % self.matrix.width;
        let point = Point::from((x, y));
        self.index += 1;
        match self.matrix.valid_point(&point) {
            true => Some((point, self.matrix.get_unsafe(&point))),
            false => None,
        }
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

    #[test]
    fn test_iter() {
        let input = "f...\n....\n....\n....\n...l";
        let matrix = Matrix::<u8>::try_from(input).expect("Failed to create matrix");

        let mut iter = matrix.iter();
        assert_eq!(Some((Point::from((0, 0)), b'f')), iter.next());
        assert_eq!(Some((Point::from((3, 4)), b'l')), iter.last());
    }

    #[test]
    fn test_index2point() {
        let input = "f...\n....\n....\n....\n...l";
        let matrix = Matrix::<u8>::try_from(input).expect("Failed to create matrix");

        assert_eq!(Some(Point::from((0, 0))), matrix.index2point(0));
        assert_eq!(Some(Point::from((3, 4))), matrix.index2point(19));
    }
}
