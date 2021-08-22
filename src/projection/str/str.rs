use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::Transform;

/// An inner type of the Enum ScaleTranslateRotate.
///
/// Cover both  translate and rotate.
#[derive(Clone, Copy, Debug, Default)]
pub struct Str<T: CoordFloat> {
    pub a: T,
    pub b: T,
    pub ai: T,
    pub bi: T,
    pub ci: T,
    pub fi: T,
    pub dx: T,
    pub dy: T,
    pub sx: T,
    pub sy: T,
}

impl<T> Transform for Str<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let x = p.x * self.sx;
        let y = p.y * self.sy;
        Coordinate {
            x: self.a * x - self.b * y + self.dx,
            y: self.dy - self.b * x - self.a * y,
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        Coordinate {
            x: self.sx * (self.ai * p.x - self.bi * p.y + self.ci),
            y: self.sy * (self.fi - self.bi * p.x - self.ai * p.y),
        }
    }
}
