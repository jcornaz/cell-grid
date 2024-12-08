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

pub mod dynamic;
mod legacy;

pub use legacy::{Coord, Grid, Rect};
