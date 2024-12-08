#![allow(missing_docs)]

use core::mem;

use cell_grid::dynamic::{Grid, IncompatibleRowSize};

#[test]
fn can_push_rows() {
    let mut grid: Grid<(i32, i32)> = Grid::new();
    for y in 0..5 {
        grid.push_row((0..5).map(|x| (x, y))).unwrap();
    }
    assert_eq!(grid.get(2, 3), Some(&(2, 3)));
}

#[test]
fn cannot_push_row_of_incompatible_width() {
    let mut grid: Grid<i32> = Grid::new();
    grid.push_row(0..10).unwrap();
    let _: IncompatibleRowSize = grid.push_row(0..11).unwrap_err();
    let _: IncompatibleRowSize = grid.push_row(0..9).unwrap_err();
    assert_eq!(grid.get(0, 1), None);
}

#[test]
fn can_create_from_size_and_init_function() {
    let grid = Grid::new_with(5, 5, |x, y| (x, y));
    assert_eq!(grid.get(2, 3), Some(&(2, 3)));
}

#[test]
fn can_create_from_size_and_default() {
    let grid = Grid::<bool>::new_with_default(5, 5);
    assert_eq!(grid.get(2, 3), Some(&false));
}

#[test]
fn can_mutate_cell() {
    let mut grid = Grid::new_with_default(2, 2);
    assert_eq!(grid.get(1, 0), Some(&0));
    *grid.get_mut(1, 0).unwrap() = 2;
    assert_eq!(grid.get(1, 0), Some(&2));
    assert_eq!(grid.set(1, 0, 3), Some(2));
    assert_eq!(grid.get(1, 0), Some(&3));
}

#[test]
fn can_iterate_cells() {
    let grid = Grid::new_with(2, 2, |x, y| (x, y));
    let rows: Vec<&(usize, usize)> = grid.cells().collect();
    assert_eq!(rows, [&(0, 0), &(1, 0), &(0, 1), &(1, 1)]);
}

#[test]
fn can_iterate_cell_mutably() {
    let mut grid = Grid::new_with(3, 3, |x, y| (x, y));
    for (x, y) in grid.cells_mut() {
        mem::swap(x, y);
    }
    assert_eq!(grid.get(1, 2), Some(&(2, 1)));
}

#[test]
fn can_iterate_rows() {
    let grid = Grid::new_with(2, 2, |x, y| (x, y));
    let rows: Vec<&[(usize, usize)]> = grid.rows().collect();
    assert_eq!(rows, [&[(0, 0), (1, 0)], &[(0, 1), (1, 1)],]);
}
