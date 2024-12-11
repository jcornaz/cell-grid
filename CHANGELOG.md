# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]

### Added

* `cells_with_coords` and `cells_with_coords_mut` iterator


## [0.1.4] - 2024-12-08


### Added

* Rename `dynamic::Grid` to `DynamicGrid` (the former remains as an alias)
* `DynamicGrid::new_from_iter`
* `DynamicGrid::width` and `DynamicGrid::height`
* `DynamicGrid::cells_in_rect` iterator


### Documentation

* Change crate root documentation to point to `DynamicGrid` instead of `Grid`


### Deprecation

* Deprecated `Grid`, `Coord` and `Rect` in favor of `DynamicGrid`
* The module `dynamic` is deprecated and hidden from documentation (its content is now in the crate root)


## [0.1.3] - 2024-12-08

### Added

This release adds a `dynamic` module with a new `Grid` struct.

This is essentially a second attempt at designing the `Grid` API, with two main differences:

* The grid size can change after creation and `push_row` is the primary way to build a `DynamicGrid`.
  I think this will improve the ergonomics of the API.
* The grid API doesn't depend on `Coord` and `Rect`, but take multiple integer arguments instead.
  This nullify the needs to add conversion from every possible vector type to `Coord`.
  And I think it might even make the API more ergonomics and simpler to understand.


## [0.1.2] - 2024-12-07

### Added 

* `aline` feature for compatiblity with the [`aline`](https://github.com/jcornaz/aline) crate (version `1.x.x`)
  It effectively replaces the `aline-v01` feature flag

### Dependencies

* Lower MSRV to 1.60


## [0.1.1] - 2023-12-09

### Added 

* `Rect::coords` iterator
* `Grid::cells` iterator
* `Grid::get_mut` and `Grid::set` method to mutate the content

### Documentation

* Minor documentation improvements

## [0.1.0] - 2023-12-08

* `Grid` struct
  * Create a new grid either with default values for cells, an iterator of cells, or using an `init` function for each cell
  * Access a cell by coordinate
  * Iterate over cells that are in a given rectangle
* `Coord` struct
* `Rect` struct
* `no_std` support (by disabling the `std` feature)
* `aline-v01` feature for compatiblity with the [`aline`](https://github.com/jcornaz/aline) crate

[Unreleased]: https://github.com/jcornaz/cell-grid/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/jcornaz/cell-grid/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/jcornaz/cell-grid/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jcornaz/cell-grid/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jcornaz/cell-grid/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jcornaz/cell-grid/compare/...v0.1.0
