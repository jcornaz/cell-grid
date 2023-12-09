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
fn can_create_from_row_major_iter() {
    let grid = Grid::from_row_major_iter(2, [1, 2, 3, 4]);
    assert_eq!(grid.get([0, 0]), Some(&1));
    assert_eq!(grid.get([1, 0]), Some(&2));
    assert_eq!(grid.get([0, 1]), Some(&3));
    assert_eq!(grid.get([1, 1]), Some(&4));
}

#[test]
fn can_mutate_cell() {
    let mut grid = Grid::new(2, 1);
    grid.set([0, 0], 1);
    assert_eq!(grid.get([0, 0]), Some(&1));
    *grid.get_mut([1, 0]).unwrap() = 2;
    assert_eq!(grid.get([1, 0]), Some(&2));
}

#[test]
#[should_panic(expected = "coordinate is out-of-bounds")]
fn set_cell_panic_if_out_of_bounds() {
    let mut grid = Grid::new(1, 1);
    grid.set([0, 1], 1);
}

#[test]
fn can_iterate_cells() {
    let grid = Grid::from_row_major_iter(2, [1, 2, 3, 4]);
    let cells: HashSet<i32> = grid.cells().copied().collect();
    let expected: HashSet<i32> = (1..=4).collect();
    assert_eq!(cells, expected);
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

#[test]
fn can_get_coords_from_rect() {
    let rect: HashSet<Coord> = Rect::from_min_max([0, 0], [1, 1]).coords().collect();
    let expected: HashSet<Coord> = [[0, 0], [0, 1], [1, 0], [1, 1]]
        .into_iter()
        .map(Coord::from)
        .collect();
    assert_eq!(rect, expected);
}
