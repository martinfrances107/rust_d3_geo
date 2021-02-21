use crate::Transform;
use geo::{CoordFloat, Coordinate};

#[derive(Debug)]
pub struct ScaleTranslate<T> {
    k: T,
    dx: T,
    dy: T,
    sx: T,
    sy: T,
}

impl<T> ScaleTranslate<T> {
    #[inline]
    pub fn new(k: T, dx: T, dy: T, sx: T, sy: T) -> Box<Self> {
        Box::new(ScaleTranslate { k, dx, dy, sx, sy })
    }
}

impl<T: CoordFloat> Transform for ScaleTranslate<T> {
    type C = Coordinate<T>;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let x = p.x * self.sx;
        let y = p.y * self.sy;
        // TODO the minus sign in the y-output component I think is a inconsistency/bug in the javascript.
        // it should be :-
        // self.dy + self.k * y
        // but that would mean a departure from the copy and would have to be adjusted elsewhere.
        return Coordinate {
            x: self.dx + self.k * x,
            y: self.dy - self.k * y,
        };
    }

    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        return Coordinate {
            x: (p.x - self.dx) / self.k * self.sx,
            y: (self.dy - p.y) / self.k * self.sy,
        };
    }
}
