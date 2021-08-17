use crate::Transform;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use std::fmt::Display;
use std::ops::AddAssign;

#[derive(Clone, Debug, Default)]
pub struct ScaleTranslate<T> {
    pub k: T,
    pub dx: T,
    pub dy: T,
    pub sx: T,
    pub sy: T,
}

impl<T> Transform for ScaleTranslate<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
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
