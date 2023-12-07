/// A coordinate of a grid
#[allow(missing_docs, clippy::exhaustive_structs)]
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    #[must_use]
    #[allow(missing_docs)]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn from_index(grid_width: usize, index: usize) -> Option<Self> {
        Some(Self {
            x: (index % grid_width).try_into().ok()?,
            y: (index / grid_width).try_into().ok()?,
        })
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn into_index(self, grid_width: usize) -> Option<usize> {
        let x: usize = self.x.try_into().ok()?;
        let y: usize = self.y.try_into().ok()?;
        if x >= grid_width {
            return None;
        }
        Some(y * grid_width + x)
    }
}

impl From<[i32; 2]> for Coord {
    fn from([x, y]: [i32; 2]) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use alloc::collections::BTreeSet;
    use rstest::rstest;

    #[rstest]
    #[allow(clippy::cast_sign_loss)]
    fn test_into_index_from_index(#[values([0, 0], [123, 456])] coord: impl Into<Coord>) {
        const WIDTH: usize = 150;
        let coord = coord.into();
        let index = coord.into_index(WIDTH).unwrap();
        assert_eq!(coord, Coord::from_index(WIDTH, index).unwrap());
    }

    #[test]
    fn ciirds_have_unique_index_in_grid_len() {
        let coords = [[0, 0], [0, 1], [1, 0], [1, 1]];
        let indices = coords
            .into_iter()
            .map(Coord::from)
            .filter_map(|c| c.into_index(2))
            .collect::<BTreeSet<usize>>();
        assert_eq!(indices.len(), coords.len());
        for index in indices {
            assert!(index < 4, "index {index} is out of grid length");
        }
    }

    #[rstest]
    fn into_index_should_return_none_if_out_of_bounds(
        #[values([-1, 0], [0, -1], [100, 0], [101, 0])] coord: impl Into<Coord>,
    ) {
        let coord = coord.into();
        let index: Option<usize> = coord.into_index(100);
        assert!(index.is_none(), "{index:?}");
    }
}
