use crate::Coord;

/// A rectangle of coordinates
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect {
    min: Coord,
    max: Coord,
}

impl Rect {
    /// Create rectangle from the `min` and `max` coordinates
    ///
    /// In a typical screen-space system (x = right, y = down)
    /// `min` is the top-left, and `max` is the bottom right
    pub fn from_min_max(min: impl Into<Coord>, max: impl Into<Coord>) -> Self {
        Self {
            min: min.into(),
            max: max.into(),
        }
    }

    pub(crate) fn coords(self) -> impl Iterator<Item = Coord> {
        (self.min.x..=self.max.x)
            .flat_map(move |x| (self.min.y..=self.max.y).map(move |y| Coord::new(x, y)))
    }
}
