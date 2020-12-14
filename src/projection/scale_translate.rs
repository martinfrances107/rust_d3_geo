use crate::Transform;
use geo::Point;
use num_traits::Float;

#[derive(Debug)]
pub struct ScaleTranslate<T> {
    k: T,
    dx: T,
    dy: T,
    sx: T,
    sy: T,
}

impl<T> ScaleTranslate<T> {
    pub fn new(k: T, dx: T, dy: T, sx: T, sy: T) -> Box<Self> {
        return Box::new(ScaleTranslate { k, dx, dy, sx, sy });
    }
}

impl<T: Float> Transform<T> for ScaleTranslate<T> {
    fn transform(&self, p: &Point<T>) -> Point<T> {
        let x = p.x() * self.sx;
        let y = p.y() * self.sy;
        // TODO the minus sign in the y-output component I think is a inconsistency/bug in the javascript.
        // it should be :-
        // self.dy + self.k * y
        // but that would mean a departure from the copy and would have to be adjusted elsewhere.
        return Point::new(self.dx + self.k * x, self.dy - self.k * y);
    }

    fn invert(&self, p: &Point<T>) -> Point<T> {
        return Point::new(
            (p.x() - self.dx) / self.k * self.sx,
            (self.dy - p.y()) / self.k * self.sy,
        );
    }
}
