#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple fixed-size 2d grid container suitable for `no_std` game development
//!
//! # Example
//!
//! ```
//! # use cell_grid::dynamic::Grid;
//! let mut grid: Grid<i32> = Grid::new();
//! assert!(grid.is_empty());
//!
//! grid.push_row([1, 2]).unwrap();
//! grid.push_row([3, 4]).unwrap();
//!
//! assert_eq!(grid.get(0, 0), Some(&1));
//! assert_eq!(grid.get(1, 0), Some(&2));
//! assert_eq!(grid.get(0, 1), Some(&3));
//! assert_eq!(grid.get(1, 1), Some(&4));
//! ```
//!
//! ## Features
//!
//! * `std`: *(enabled by default)* enable use of the standard library. Must be disabled for `no_std` crates.
//! * `aline`: add conversions between [`Coord`] and [aline](https://docs.rs/aline/1) vectors

extern crate alloc;

#[deprecated(
    since = "0.1.4",
    note = "The content of this module has been moved to the crate root"
)]
#[doc(hidden)]
pub mod dynamic;
mod legacy;

#[allow(deprecated)]
pub use legacy::{Coord, Grid, Rect};

use core::{fmt::Display, mem};

use alloc::vec::Vec;

/// A row-major 2d grid
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DynamicGrid<T> {
    cells: Vec<T>,
    width: usize,
}

impl<T> Default for DynamicGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DynamicGrid<T>
where
    T: Default,
{
    /// Create a new grid of the given size, with each cells being initialized with the default value of `T`
    #[must_use]
    pub fn new_with_default(width: usize, height: usize) -> Self {
        Self::new_with(width, height, |_, _| T::default())
    }
}

impl<T> DynamicGrid<T> {
    /// Create a new empty grid
    ///
    /// Use [`Self::push_row`] to add content
    #[must_use]
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            width: 0,
        }
    }

    /// Create a new empty grid with pre-allocated `capacity` cells
    ///
    /// Use [`Self::push_row`] to add content
    #[must_use]
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            cells: Vec::with_capacity(capacity),
            width: 0,
        }
    }

    /// Create a new grid of the given size, with each cells being initialized with the given function
    #[must_use]
    pub fn new_with(width: usize, height: usize, mut init: impl FnMut(usize, usize) -> T) -> Self {
        Self {
            cells: (0..(width * height))
                .map(|i| {
                    let (x, y) = Self::index_to_coord(i, width);
                    init(x, y)
                })
                .collect(),
            width,
        }
    }

    /// Create a new grid of the given width and row-major iterator
    ///
    ///
    /// The given iterator must emits cells by row first, then by column
    ///
    /// # Errors
    ///
    /// Returns [`IncompatibleRowSize`] if the number of elements yielded by the iterator is not compaible
    /// with the given `width`
    ///
    /// # Example
    ///
    /// ```
    /// # use cell_grid::dynamic::Grid;
    /// let grid = Grid::new_from_iter(2, [1, 2, 3, 4]).unwrap();
    /// assert_eq!(grid.get(0, 0), Some(&1));
    /// assert_eq!(grid.get(1, 0), Some(&2));
    /// assert_eq!(grid.get(0, 1), Some(&3));
    /// assert_eq!(grid.get(1, 1), Some(&4));
    /// ```
    pub fn new_from_iter(
        width: usize,
        iter: impl IntoIterator<Item = T>,
    ) -> Result<Self, IncompatibleRowSize> {
        let cells: Vec<T> = iter.into_iter().collect();
        if !cells.is_empty() && (width == 0 || cells.len() % width != 0) {
            return Err(IncompatibleRowSize);
        };
        Ok(Self { cells, width })
    }

    /// Push a row to the grid
    ///
    /// If the grid is not empty, the row length should match the current width of the grid.
    ///
    /// # Errors
    ///
    /// Returns [`IncompatibleRowSize`] if the grid is not empty and the length of the added row does not match the current width of the grid.
    pub fn push_row(
        &mut self,
        row: impl IntoIterator<Item = T>,
    ) -> Result<(), IncompatibleRowSize> {
        let old_len = self.cells.len();
        self.cells.extend(row);
        if self.width == 0 {
            self.width = self.cells.len();
        } else if self.width != self.cells.len() - old_len {
            self.cells.truncate(old_len);
            return Err(IncompatibleRowSize);
        }
        Ok(())
    }

    /// Returns `true` if the grid is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Returns the width of the grid
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid
    #[must_use]
    pub fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    /// Get a reference to the cell at col `x` and row `y`
    ///
    /// Returns `None` if `x` and `y` are out of bounds
    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.index(x, y).and_then(|i| self.cells.get(i))
    }

    /// Get a mutable reference to the cell at col `x` and row `y`
    ///
    /// Returns `None` if `x` and `y` are out of bounds
    #[must_use]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.index(x, y).and_then(|i| self.cells.get_mut(i))
    }

    /// Set the new value to the cell at col `x` and row `y` and return the old value.
    ///
    /// Returns `None` if `x` and `y` are out of bounds
    pub fn set(&mut self, x: usize, y: usize, mut new_value: T) -> Option<T> {
        let cell = self.get_mut(x, y)?;
        mem::swap(cell, &mut new_value);
        Some(new_value)
    }

    /// Returns an iterator over the cells
    #[must_use]
    pub fn cells(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.cells.iter()
    }

    /// Returns a mutable iterator over the cells
    #[must_use]
    pub fn cells_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> {
        self.cells.iter_mut()
    }

    /// Returns an iterator over the cells in the rectangle that start at col `x`, row `y` and of size given by `width` and `height`
    ///
    /// # Example
    ///
    /// ```
    /// # use cell_grid::dynamic::Grid;
    /// let grid = Grid::new_with(5, 5, |x, y| (x, y));
    /// let in_rect: Vec<_> = grid.cells_in_rect(2, 2, 2, 2).copied().collect();
    /// assert_eq!(in_rect, &[(2, 2), (3, 2), (2, 3), (3, 3)]);
    /// ```
    #[must_use]
    pub fn cells_in_rect(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> impl DoubleEndedIterator<Item = &T> {
        let width = width.min(self.width - x);
        let grid_height = self.height();
        (y..(y + height))
            .filter(move |y| *y < grid_height)
            .filter_map(move |y| self.index(x, y))
            .flat_map(move |from| &self.cells[from..(from + width)])
    }

    /// Returns an iterator over the rows
    #[must_use]
    pub fn rows(&self) -> impl DoubleEndedIterator<Item = &[T]> {
        (0..self.cells.len())
            .step_by(self.width)
            .map(|i| &self.cells[i..(i + self.width)])
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width {
            None
        } else {
            Some(y * self.width + x)
        }
    }

    fn index_to_coord(index: usize, width: usize) -> (usize, usize) {
        (index % width, index / width)
    }
}

/// Error returned by [`DynamicGrid::push_row`] if the length of the row being pushed
/// is incompatible with the current width of the grid
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct IncompatibleRowSize;

impl Display for IncompatibleRowSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "The row size is not compatible with the current content of the grid"
        )
    }
}

#[rustversion::since(1.81)]
impl core::error::Error for IncompatibleRowSize {}
