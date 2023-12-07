#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple fixed-size 2d grid container suitable for `no_std` game development
//!
//! ## Features
//!
//! `std`: *(enabled by default)* enable use of the standard library. Must be disabled for `no_std` crates.
//! `aline-v01`: add conversions between [`Coord`] and [aline 0.1](https://docs.rs/aline/0.1) vectors

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
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with(width, height, |_| T::default())
    }
}

impl<T> Grid<T> {
    /// Create a grid using `init_cell` function for each cell
    #[must_use]
    pub fn new_with(width: usize, height: usize, init_cell: impl FnMut(Coord) -> T) -> Self {
        let cells = (0..(width * height))
            .map(|i| Coord::from_index(width, i))
            .map(init_cell)
            .collect();
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

    /// Iterator over cells in `rect`
    pub fn cells_in_rect(&self, rect: impl Into<Rect>) -> impl Iterator<Item = &T> {
        rect.into().coords().filter_map(|c| self.get(c))
    }
}
