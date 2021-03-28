use crate::Transform;
use geo::{CoordFloat, Coordinate};

#[derive(Clone, Debug, Default)]
pub struct ScaleTranslate<T> {
    pub k: T,
    pub dx: T,
    pub dy: T,
    pub sx: T,
    pub sy: T,
}

// impl<T> ScaleTranslate<T> {
//     #[inline]
//     pub fn new(k: &T, dx: &T, dy: &T, sx: &T, sy: &T) -> Self {
//         ScaleTranslate { k, dx, dy, sx, sy }
//     }
// }

// impl<'a, T: CoordFloat> TransformClone<'a> for ScaleTranslate<T> {
//     fn box_clone(&'a self) -> Box<dyn TransformClone<'a, TcC = Self::TcC>> {
//         Box::new(self.clone())
//     }
// }

impl<T: CoordFloat> Transform for ScaleTranslate<T> {
    type TcC = Coordinate<T>;
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
