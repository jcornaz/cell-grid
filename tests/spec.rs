use std::collections::HashSet;

use cell_grid::{Coord, Grid, Rect};

#[test]
fn can_create_grid() {
    let grid = Grid::<i32>::new(10, 42);
    for x in 0..10 {
        for y in 0..42 {
            assert_eq!(*grid.get([x, y]).unwrap(), 0);
        }
    }
}

#[test]
fn can_crate_grid_with_cell_init_function() {
    let grid = Grid::new_with(10, 42, |coord| coord);
    for x in 0..10 {
        for y in 0..42 {
            assert_eq!(*grid.get([x, y]).unwrap(), Coord::new(x, y));
        }
    }
}

#[test]
fn can_iterate_cells_overlaping_a_rectangle() {
    let grid = Grid::new_with(10, 42, |coord| coord);
    let cells: HashSet<Coord> = grid
        .cells_in_rect(Rect::from_min_max([1, 2], [2, 3]))
        .copied()
        .collect();
    let expected_cells: HashSet<Coord> = [[1, 2], [1, 3], [2, 2], [2, 3]]
        .into_iter()
        .map(Coord::from)
        .collect();
    assert_eq!(cells, expected_cells);
}
