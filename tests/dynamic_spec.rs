#![allow(missing_docs)]

use core::mem;

use cell_grid::{DynamicGrid, IncompatibleRowSize};

#[test]
fn can_push_rows() {
    let mut grid: DynamicGrid<(i32, i32)> = DynamicGrid::new();
    for y in 0..5 {
        grid.push_row((0..5).map(|x| (x, y))).unwrap();
    }
    assert_eq!(grid.get(2, 3), Some(&(2, 3)));
}

#[test]
fn cannot_push_row_of_incompatible_width() {
    let mut grid: DynamicGrid<i32> = DynamicGrid::new();
    grid.push_row(0..10).unwrap();
    let _: IncompatibleRowSize = grid.push_row(0..11).unwrap_err();
    let _: IncompatibleRowSize = grid.push_row(0..9).unwrap_err();
    assert_eq!(grid.get(0, 1), None);
}

#[test]
fn can_create_from_size_and_init_function() {
    let grid = DynamicGrid::new_with(5, 5, |x, y| (x, y));
    assert_eq!(grid.get(2, 3), Some(&(2, 3)));
}

#[test]
fn get_return_null_when_x_is_out_of_bounds() {
    let grid = DynamicGrid::new_with(5, 5, |x, y| (x, y));
    assert_eq!(grid.get(5, 0), None);
}

#[test]
fn can_create_from_size_and_default() {
    let grid = DynamicGrid::<bool>::new_with_default(5, 5);
    assert_eq!(grid.get(2, 3), Some(&false));
}

#[test]
fn can_create_from_row_major_iter() {
    let grid = DynamicGrid::new_from_iter(2, [1, 2, 3, 4]).unwrap();
    assert_eq!(grid.get(0, 0), Some(&1));
    assert_eq!(grid.get(1, 0), Some(&2));
    assert_eq!(grid.get(0, 1), Some(&3));
    assert_eq!(grid.get(1, 1), Some(&4));
}

#[test]
fn cannot_create_from_invalid_row_major_iter() {
    let _: IncompatibleRowSize = DynamicGrid::new_from_iter(2, [1, 2, 3, 4, 5]).unwrap_err();
}

#[test]
fn cannot_create_from_invalid_row_major_iter_wth_invalid_width() {
    let _: IncompatibleRowSize = DynamicGrid::new_from_iter(0, [1]).unwrap_err();
}

#[test]
fn can_create_empty_grid_from_iter() {
    let grid = DynamicGrid::<i32>::new_from_iter(0, []).unwrap();
    assert!(grid.is_empty());
}

#[test]
fn can_mutate_cell() {
    let mut grid = DynamicGrid::new_with_default(2, 2);
    assert_eq!(grid.get(1, 0), Some(&0));
    *grid.get_mut(1, 0).unwrap() = 2;
    assert_eq!(grid.get(1, 0), Some(&2));
    assert_eq!(grid.set(1, 0, 3), Some(2));
    assert_eq!(grid.get(1, 0), Some(&3));
}

#[test]
fn can_iterate_cells() {
    let grid = DynamicGrid::new_with(2, 2, |x, y| (x, y));
    let rows: Vec<&(usize, usize)> = grid.cells().collect();
    assert_eq!(rows, [&(0, 0), &(1, 0), &(0, 1), &(1, 1)]);
}

#[test]
fn can_iterate_cell_mutably() {
    let mut grid = DynamicGrid::new_with(3, 3, |x, y| (x, y));
    for (x, y) in grid.cells_mut() {
        mem::swap(x, y);
    }
    assert_eq!(grid.get(1, 2), Some(&(2, 1)));
}

#[test]
fn can_iterate_rows() {
    let grid = DynamicGrid::new_with(2, 2, |x, y| (x, y));
    let rows: Vec<&[(usize, usize)]> = grid.rows().collect();
    assert_eq!(rows, [&[(0, 0), (1, 0)], &[(0, 1), (1, 1)],]);
}

#[test]
fn can_iterate_cells_overlaping_a_rectangle() {
    let grid = DynamicGrid::new_with(10, 10, |x, y| (x, y));
    let cells: Vec<&(usize, usize)> = grid.cells_in_rect(4, 5, 2, 3).collect();
    assert_eq!(
        cells,
        [&(4, 5), &(5, 5), &(4, 6), &(5, 6), &(4, 7), &(5, 7)]
    );
}

#[test]
fn out_of_bounds_are_ignored_when_iterating_overlaping_rect() {
    let grid = DynamicGrid::new_with(10, 10, |x, y| (x, y));
    let cells: Vec<&(usize, usize)> = grid.cells_in_rect(9, 9, 5, 5).collect();
    assert_eq!(cells, [&(9, 9)]);
}

#[test]
fn can_iterate_cells_with_coords() {
    let grid = DynamicGrid::<u8>::new_from_iter(2, 0..4).unwrap();
    let values: Vec<((usize, usize), &u8)> = grid.cells_with_coords().collect();
    assert_eq!(
        &values,
        &[((0, 0), &0), ((1, 0), &1), ((0, 1), &2), ((1, 1), &3),]
    );
}

#[test]
fn can_iterate_cells_with_coords_mutably() {
    let mut grid = DynamicGrid::<u8>::new_from_iter(2, 0..4).unwrap();
    let values: Vec<((usize, usize), &mut u8)> = grid.cells_with_coords_mut().collect();
    assert_eq!(
        &values,
        &[
            ((0, 0), &mut 0),
            ((1, 0), &mut 1),
            ((0, 1), &mut 2),
            ((1, 1), &mut 3),
        ]
    );
}

#[test]
fn grid_should_be_thread_safe() {
    assert_thread_safe::<DynamicGrid<i32>>();
}

#[test]
fn errors_should_be_thread_safe() {
    assert_thread_safe::<IncompatibleRowSize>();
}

/// Statically ensure a types implement all basic traits
fn assert_thread_safe<T>()
where
    T: Send + Sync + 'static,
{
}
