#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple fixed-size 2d grid container suitable for `no_std` game development
//!
//! See [`Grid::new`], [`Grid::new_with`] and [`Grid::from_row_major_iter`] for examples on
//! how to create a grid.
//!
//! ## Features
//!
//! * `std`: *(enabled by default)* enable use of the standard library. Must be disabled for `no_std` crates.
//! * `aline-v01`: add conversions between [`Coord`] and [aline 0.1](https://docs.rs/aline/0.1) vectors

extern crate alloc;

mod coord;
mod rect;

use alloc::vec::Vec;

pub use coord::Coord;
pub use rect::Rect;

/// A 2d fixed-size grid containers for cells of type `T`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
}

impl<T: Default> Grid<T> {
    /// Create a grid using the default value of `T` for each cell
    ///
    /// # Example
    ///
    /// ```
    /// # use cell_grid::{Grid, Coord};
    /// let grid: Grid<i32> = Grid::new(2, 2);
    /// assert_eq!(grid.get([0, 0]), Some(&0));
    /// assert_eq!(grid.get([1, 0]), Some(&0));
    /// assert_eq!(grid.get([0, 1]), Some(&0));
    /// assert_eq!(grid.get([1, 1]), Some(&0));
    /// ```
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with(width, height, |_| T::default())
    }
}

impl<T> Grid<T> {
    /// Create a grid using `init_cell` function for each cell
    ///
    /// # Example
    ///
    /// ```
    /// # use cell_grid::{Grid, Coord};
    /// let grid = Grid::new_with(2, 2, |coord| coord);
    /// assert_eq!(grid.get([0, 0]), Some(&Coord { x: 0, y: 0 }));
    /// assert_eq!(grid.get([1, 0]), Some(&Coord { x: 1, y: 0 }));
    /// assert_eq!(grid.get([0, 1]), Some(&Coord { x: 0, y: 1 }));
    /// assert_eq!(grid.get([1, 1]), Some(&Coord { x: 1, y: 1 }));
    /// ```
    #[must_use]
    pub fn new_with(width: usize, height: usize, init_cell: impl FnMut(Coord) -> T) -> Self {
        Self::from_row_major_iter(
            width,
            (0..(width * height))
                .map(|i| Coord::from_index(width, i))
                .map(init_cell),
        )
    }

    /// Create a grid using the row-major `iter`
    ///
    /// # Example
    ///
    /// ```
    /// # use cell_grid::Grid;
    /// let grid = Grid::from_row_major_iter(2, [1, 2, 3, 4]);
    /// assert_eq!(grid.get([0, 0]), Some(&1));
    /// assert_eq!(grid.get([1, 0]), Some(&2));
    /// assert_eq!(grid.get([0, 1]), Some(&3));
    /// assert_eq!(grid.get([1, 1]), Some(&4));
    /// ```
    #[must_use]
    pub fn from_row_major_iter(width: usize, iter: impl IntoIterator<Item = T>) -> Self {
        let cells = iter.into_iter().collect();
        Self { cells, width }
    }

    /// Access the cell at `coord`
    ///
    /// Returns `None` if the `coord` is out of bounds
    #[must_use]
    pub fn get(&self, coord: impl Into<Coord>) -> Option<&T> {
        let index = coord.into().into_index(self.width)?;
        self.cells.get(index)
    }

    /// Iterator over all cells
    pub fn cells(&self) -> impl Iterator<Item = &T> {
        self.cells.iter()
    }

    /// Iterator over cells in `rect`
    pub fn cells_in_rect(&self, rect: impl Into<Rect>) -> impl Iterator<Item = &T> {
        rect.into().coords().filter_map(|c| self.get(c))
    }
}
