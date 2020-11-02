use delaunator::Point;

use crate::Transform;

#[derive(Debug)]
pub struct ScaleTranslate {
    k: f64,
    dx: f64,
    dy: f64,
    sx: f64,
    sy: f64,
}

impl ScaleTranslate {
    pub fn new(k: f64, dx: f64, dy: f64, sx: f64, sy: f64) -> Box<Self> {
        return Box::new(ScaleTranslate { k, dx, dy, sx, sy });
    }
}

impl Transform for ScaleTranslate {
    fn transform(&self, p: &Point) -> Point {
        let x = p.x * self.sx;
        let y = p.y * self.sy;
        // TODO the minus sign in the y-output component I think is a inconsistency/bug in the javascript.
        // it should be :-
        // self.dy + self.k * y
        // but that would mean a departure from the copy and would have to be adjusted elsewhere.
        return Point {
            x: self.dx + self.k * x,
            y: self.dy - self.k * y,
        };
    }

    fn invert(&self, p: &Point) -> Point {
        return Point {
            x: (p.x - self.dx) / self.k * self.sx,
            y: (self.dy - p.y) / self.k * self.sy,
        };
    }
}
