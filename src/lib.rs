#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple fixed-size cellular grid container suitable for `no_std` game development
//!
//! ## Features
//!
//! `std`: *(enabled by default)* enable use of the standard library. Must be disabled for `no_std` crates.

extern crate alloc;

mod coord;

use alloc::vec::Vec;

pub use coord::Coord;

/// A 2d fixed-size grid containers for cells of type `T`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
}
