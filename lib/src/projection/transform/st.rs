use geo::CoordFloat;
use geo_types::Coord;

use crate::Transform;

/// An inner type of [`ScaleTranslateRotate`](super::ScaleTranslateRotate).
///
/// Simplification when only  translate is needed.
#[derive(Clone, Debug)]
pub struct St<T> {
    k: T,
    dx: T,
    dy: T,
    sx: T,
    sy: T,
}

impl<T> St<T>
where
    T: Copy,
{
    /// Constructor.
    #[inline]
    pub const fn new(k: &T, dx: &T, dy: &T, sx: &T, sy: &T) -> Self {
        Self {
            k: *k,
            dx: *dx,
            dy: *dy,
            sx: *sx,
            sy: *sy,
        }
    }
}

impl<T> Transform for St<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        let x = p.x * self.sx;
        let y = p.y * self.sy;
        Coord {
            x: self.dx + self.k * x,
            y: self.dy - self.k * y,
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        Coord {
            x: (p.x - self.dx) / self.k * self.sx,
            y: (self.dy - p.y) / self.k * self.sy,
        }
    }
}
