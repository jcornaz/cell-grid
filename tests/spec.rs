use cell_grid::{Coord, Grid};

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
