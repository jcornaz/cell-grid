//! A 2d grid container that can grow after being created

use core::{fmt::Display, mem};

use alloc::vec::Vec;

/// A row-major 2d grid
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Grid<T>
where
    T: Default,
{
    /// Create a new grid of the given size, with each cells being initialized with the default value of `T`
    #[must_use]
    pub fn new_with_default(width: usize, height: usize) -> Self {
        Self::new_with(width, height, |_, _| T::default())
    }
}

impl<T> Grid<T> {
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

    /// Get a reference to the cell at col `x` and row `y`
    ///
    /// Returns `None` if `x` and `y` are out of bounds
    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.cells.get(self.index(x, y))
    }

    /// Get a mutable reference to the cell at col `x` and row `y`
    ///
    /// Returns `None` if `x` and `y` are out of bounds
    #[must_use]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let index = self.index(x, y);
        self.cells.get_mut(index)
    }

    /// Set the new value to the cell at col `x` and row `y` and return the old value.
    ///
    /// Returns `None` if `x` and `y` are out of bounds
    #[must_use]
    pub fn set(&mut self, x: usize, y: usize, mut new_value: T) -> Option<T> {
        let cell = self.get_mut(x, y)?;
        mem::swap(cell, &mut new_value);
        Some(new_value)
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn index_to_coord(index: usize, width: usize) -> (usize, usize) {
        (index % width, index / width)
    }
}

/// Error returned by [`Grid::push_row`] if the length of the row being pushed
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
