use geo::{CoordFloat, Coordinate};

use crate::Transform;
use crate::TransformClone;

use super::scale_translate::ScaleTranslate;

#[derive(Clone, Copy, Debug, Default)]
pub struct ScaleTranslateRotate<T: CoordFloat> {
    a: T,
    b: T,
    ai: T,
    bi: T,
    ci: T,
    fi: T,
    dx: T,
    dy: T,
    sx: T,
    sy: T,
}

impl<T: CoordFloat + 'static> ScaleTranslateRotate<T> {
    #[inline]
    pub fn new(
        k: T,
        dx: T,
        dy: T,
        sx: T,
        sy: T,
        alpha: T,
    ) -> Box<dyn Transform<TcC = Coordinate<T>>> {
        if alpha.is_zero() {
            ScaleTranslate::new(k, dx, dy, sx, sy)
        } else {
            let cos_alpha = alpha.cos();
            let sin_alpha = alpha.sin();
            Box::new(ScaleTranslateRotate {
                a: cos_alpha * k,
                b: sin_alpha * k,
                ai: cos_alpha / k,
                bi: sin_alpha / k,
                ci: (sin_alpha * dy - cos_alpha * dx) / k,
                fi: (sin_alpha * dx + cos_alpha * dy) / k,
                dx,
                dy,
                sx,
                sy,
            })
        }
    }
}

impl<T: CoordFloat + 'static> TransformClone for ScaleTranslateRotate<T> {
    type TcC = Coordinate<T>;
    fn clone_box(&self) -> Box<dyn Transform<TcC = Self::TcC>> {
        Box::new(self.clone())
    }
}

impl<T: CoordFloat + 'static> Transform for ScaleTranslateRotate<T> {
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
