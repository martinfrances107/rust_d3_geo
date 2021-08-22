use geo::{CoordFloat, Coordinate};

use crate::Transform;

/// An inner type of the Enum ScaleTranslateRotate.
///
/// Simplification when only  translate is needed.
#[derive(Clone, Debug)]
pub struct St<T>
where
    T: CoordFloat,
{
    pub k: T,
    pub dx: T,
    pub dy: T,
    pub sx: T,
    pub sy: T,
}

/// Default is a identity tranform.
///
/// Unity scaling, zero translation.
impl<T> Default for St<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            k: T::one(),
            dx: T::zero(),
            dy: T::zero(),
            sx: T::one(),
            sy: T::one(),
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
        // TODO the minus sign in the y-output component I think is a inconsistency/bug in the javascript.
        // it should be :-
        // self.dy + self.k * y
        // but that would mean a departure from the copy and would have to be adjusted elsewhere.
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
