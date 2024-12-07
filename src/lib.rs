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
//! * `aline`: add conversions between [`Coord`] and [aline](https://docs.rs/aline/1) vectors

extern crate alloc;

mod coord;
mod rect;

#[allow(deprecated)]
pub use coord::Coord;
#[allow(deprecated)]
pub use rect::Rect;

use alloc::vec::Vec;

/// A 2d row-major grid containers for cells of type `T`
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
    /// # use cell_grid::Grid;
    /// let grid: Grid<i32> = Grid::new(2, 2);
    /// assert_eq!(grid.get(0, 0), Some(&0));
    /// assert_eq!(grid.get(1, 0), Some(&0));
    /// assert_eq!(grid.get(0, 1), Some(&0));
    /// assert_eq!(grid.get(1, 1), Some(&0));
    /// ```
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with(width, height, |_, _| T::default())
    }
}

impl<T> Grid<T> {
    /// The maximum width of a grid
    pub const MAX_WIDTH: usize = i32::MAX as usize;
    /// The maximum height of a grid
    pub const MAX_HEIGHT: usize = i32::MAX as usize;

    /// Create a grid using `init_cell` function for each cell
    ///
    ///
    /// # Panics
    ///
    /// Panics if the `width` or `height` is bigger than [`Self::MAX_WIDTH`] or [`Self::MAX_HEIGHT`] respectively.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use cell_grid::Grid;
    /// let grid = Grid::new_with(2, 2, |x, y| (x, y));
    /// assert_eq!(grid.get(0, 0), Some(&(0, 0)));
    /// assert_eq!(grid.get(1, 0), Some(&(1, 0)));
    /// assert_eq!(grid.get(0, 1), Some(&(0, 1)));
    /// assert_eq!(grid.get(1, 1), Some(&(1, 1)));
    /// ```
    #[must_use]
    pub fn new_with(width: usize, height: usize, mut init_cell: impl FnMut(i32, i32) -> T) -> Self {
        assert!(
            height < Self::MAX_HEIGHT,
            "Height cannot be greater than  {}",
            Self::MAX_HEIGHT
        );
        Self::from_row_major_iter(
            width,
            (0..(width * height)).map(|i| {
                let (x, y) = index_to_coord(i, width);
                init_cell(x, y)
            }),
        )
    }

    /// Create a grid using the row-major `iter`
    ///
    ///
    /// # Panics
    ///
    /// Panics if the `width` or `height` is bigger than [`Self::MAX_WIDTH`] or [`Self::MAX_HEIGHT`] respectively.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use cell_grid::Grid;
    /// let grid = Grid::from_row_major_iter(2, [1, 2, 3, 4]);
    /// assert_eq!(grid.get(0, 0), Some(&1));
    /// assert_eq!(grid.get(1, 0), Some(&2));
    /// assert_eq!(grid.get(0, 1), Some(&3));
    /// assert_eq!(grid.get(1, 1), Some(&4));
    /// ```
    #[must_use]
    pub fn from_row_major_iter(width: usize, iter: impl IntoIterator<Item = T>) -> Self {
        assert!(
            width < Self::MAX_WIDTH,
            "Height cannot be greater than  {}",
            Self::MAX_WIDTH
        );
        let cells = iter.into_iter().take(width * Self::MAX_HEIGHT).collect();
        Self { cells, width }
    }

    /// Access the cell at row `x` and column `y`
    ///
    /// Returns `None` if the `coord` is out of bounds
    #[must_use]
    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        self.index(x, y).and_then(|i| self.cells.get(i))
    }

    /// Access mutably the cell at row `x` and column `y`
    ///
    /// Returns `None` if the `coord` is out of bounds
    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        self.index(x, y).and_then(|i| self.cells.get_mut(i))
    }

    /// Set the value of the cell at row `x` and column `y`
    ///
    /// # Panics
    ///
    /// Panics if the `coord` is out-of-bounds
    ///
    /// For a non-panicking version, use `get_mut`
    pub fn set(&mut self, x: i32, y: i32, value: T) {
        let cell = self.get_mut(x, y).expect("coordinate is out-of-bounds");
        *cell = value;
    }

    /// Iterator over all cells
    pub fn cells(&self) -> impl Iterator<Item = &T> {
        self.cells.iter()
    }

    /// Iterator over all cells mutably
    pub fn cells_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.cells.iter_mut()
    }

    /// Iterator over the cells associated with their coordinates
    pub fn cells_with_coords(&self) -> impl Iterator<Item = ((i32, i32), &T)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, cell)| (index_to_coord(i, self.width), cell))
    }

    /// Iterator over the mutable cells associated with their coordinates
    pub fn cells_with_coords_mut(&mut self) -> impl Iterator<Item = ((i32, i32), &mut T)> {
        self.cells
            .iter_mut()
            .enumerate()
            .map(|(i, cell)| (index_to_coord(i, self.width), cell))
    }

    /// Iterator over the cells in the rectangle
    pub fn cells_in_rect(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> impl Iterator<Item = &T> {
        (y..(y + height))
            .filter_map(move |y| {
                let from = self.index(x, y)?;
                let to = self.index(x + width, y)?;
                self.cells.get(from..to)
            })
            .flatten()
    }

    fn index(&self, x: i32, y: i32) -> Option<usize> {
        let x: usize = x.try_into().ok()?;
        if x >= self.width {
            return None;
        }
        let y: usize = y.try_into().ok()?;
        Some(y * self.width + x)
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn index_to_coord(i: usize, width: usize) -> (i32, i32) {
    ((i % width) as i32, (i / width) as i32)
}
