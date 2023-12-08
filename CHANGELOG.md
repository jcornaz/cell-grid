# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]

* `Grid` struct
  * Create a new grid either with default values for cells, an iterator of cells, or using an `init` function for each cell
  * Access a cell by coordinate
  * Iterate over cells that are in a given rectangle
* `Coord` struct
* `Rect` struct
* `no_std` support (by disabling the `std` feature)
* `aline-v01` feature for compatiblity with the [`aline`](https://github.com/jcornaz/aline) crate

[Unreleased]: https://github.com/jcornaz/cell-grid/compare/...HEAD
