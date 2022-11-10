use geo::CoordFloat;
use geo::Coordinate;

use crate::Transform;

/// An inner type of [`ScaleTranslateRotate`].
///
/// Simplification when only  translate is needed.
#[derive(Clone, Copy, Debug)]
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
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let x = p.x * self.sx;
        let y = p.y * self.sy;
        Coordinate {
            x: self.dx + self.k * x,
            y: self.dy - self.k * y,
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        Coordinate {
            x: (p.x - self.dx) / self.k * self.sx,
            y: (self.dy - p.y) / self.k * self.sy,
        }
    }
}
