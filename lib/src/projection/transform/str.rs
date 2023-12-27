use geo::CoordFloat;
use geo_types::Coord;

use crate::Transform;

/// An inner type of the [`ScaleTranslateRotate`](super::scale_translate_rotate::ScaleTranslateRotate).
///
/// Covers both translate and rotate.
#[derive(Clone, Debug, Default)]
pub struct Str<T> {
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

impl<T> Str<T>
where
    T: CoordFloat,
{
    /// Constructor.
    pub(crate) fn new(
        k: &T,
        dx: &T,
        dy: &T,
        sx: &T,
        sy: &T,
        alpha: &T,
    ) -> Self {
        let (sin_alpha, cos_alpha) = alpha.sin_cos();
        Self {
            a: cos_alpha * *k,
            b: sin_alpha * *k,
            ai: cos_alpha / *k,
            bi: sin_alpha / *k,
            ci: (sin_alpha * *dy - cos_alpha * *dx) / *k,
            fi: (sin_alpha * *dx + cos_alpha * *dy) / *k,
            dx: *dx,
            dy: *dy,
            sx: *sx,
            sy: *sy,
        }
    }
}

impl<T> Transform for Str<T>
where
    T: CoordFloat,
{
    type T = T;

    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        let x = p.x * self.sx;
        let y = p.y * self.sy;
        Coord {
            x: self.a * x - self.b * y + self.dx,
            y: self.dy - self.b * x - self.a * y,
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        Coord {
            x: self.sx * (self.ai * p.x - self.bi * p.y + self.ci),
            y: self.sy * (self.fi - self.bi * p.x - self.ai * p.y),
        }
    }
}
