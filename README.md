# cell-grid

![rustc](https://img.shields.io/badge/rustc-1.60+-blue?logo=rust)
[![License](https://img.shields.io/crates/l/cell-grid)](#Unlicense)
[![Crates.io](https://img.shields.io/crates/v/cell-grid)](https://crates.io/crates/cell-grid)
[![Docs](https://docs.rs/cell-grid/badge.svg)](https://docs.rs/cell-grid)

A simple 2d grid container

```rust
use cell_grid::DynamicGrid;

// Create a new empty grid
let mut grid: DynamicGrid<i32> = DynamicGrid::new();

// Push rows
grid.push_row([1, 2]).unwrap();
grid.push_row([3, 4]).unwrap();

// Access by coordinate
assert_eq!(grid.get(0, 0), Some(&1));
assert_eq!(grid.get(1, 0), Some(&2));
assert_eq!(grid.get(0, 1), Some(&3));
assert_eq!(grid.get(1, 1), Some(&4));

// Iterate the content
assert_eq!(grid.cells().copied().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
assert_eq!(grid.rows().collect::<Vec<_>>(), vec![&[1, 2], &[3, 4]]);
```


## Feature flags

* `std`: enabled by default. must be disabled to compile to `no_std`


## MSRV

The minimum supported rust version is currently `1.60`.

It will be updated when required, and that will not be considered a breaking change (it can happen in a minor version).


## Unlicense

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
