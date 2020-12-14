use geo::Point;
use num_traits::Float;

use crate::Transform;

use super::scale_translate::ScaleTranslate;

#[derive(Debug)]
pub struct ScaleTranslateRotate<T: Float> {
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

impl<T: Float + 'static> ScaleTranslateRotate<T> {
    pub fn new(k: T, dx: T, dy: T, sx: T, sy: T, alpha: T) -> Box<dyn Transform<T>> {
        if alpha.is_zero() {
            return ScaleTranslate::new(k, dx, dy, sx, sy);
        } else {
            let cos_alpha = alpha.cos();
            let sin_alpha = alpha.sin();
            return Box::new(ScaleTranslateRotate {
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
            });
        }
    }
}

impl<T: Float> Transform<T> for ScaleTranslateRotate<T> {
    fn transform(&self, p: &Point<T>) -> Point<T> {
        let x = p.x() * self.sx;
        let y = p.y() * self.sy;
        return Point::new(
            self.a * x - self.b * y + self.dx,
            self.dy - self.b * x - self.a * y,
        );
    }

    fn invert(&self, p: &Point<T>) -> Point<T> {
        return Point::new(
            self.sx * (self.ai * p.x() - self.bi * p.y() + self.ci),
            self.sy * (self.fi - self.bi * p.x() - self.ai * p.y()),
        );
    }
}
