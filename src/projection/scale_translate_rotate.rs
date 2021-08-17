use std::fmt::Display;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::Transform;

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

#[derive(Clone, Debug)]
pub enum ScaleTranslateRotateEnum<T>
where
    T: CoordFloat + FloatConst,
{
    ST(ScaleTranslate<T>),
    STR(ScaleTranslateRotate<T>),
    Blank,
}

impl<T> Default for ScaleTranslateRotateEnum<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn default() -> Self {
        ScaleTranslateRotateEnum::Blank
    }
}

impl<T: CoordFloat> Transform for ScaleTranslateRotateEnum<T>
where
    T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            ScaleTranslateRotateEnum::ST(st) => st.transform(p),
            ScaleTranslateRotateEnum::STR(str) => str.transform(p),
            ScaleTranslateRotateEnum::Blank => {
                panic!("calling transform() on blank.")
            }
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            ScaleTranslateRotateEnum::ST(st) => st.invert(p),
            ScaleTranslateRotateEnum::STR(str) => str.invert(p),
            ScaleTranslateRotateEnum::Blank => {
                panic!("calling invert() on blank.")
            }
        }
    }
}

impl<T: CoordFloat + FloatConst> ScaleTranslateRotate<T> {
    #[inline]
    pub fn new(k: &T, dx: &T, dy: &T, sx: &T, sy: &T, alpha: &T) -> ScaleTranslateRotateEnum<T> {
        if alpha.is_zero() {
            ScaleTranslateRotateEnum::ST(ScaleTranslate {
                k: *k,
                dx: *dx,
                dy: *dy,
                sx: *sx,
                sy: *sy,
            })
        } else {
            let cos_alpha = alpha.cos();
            let sin_alpha = alpha.sin();
            ScaleTranslateRotateEnum::STR(ScaleTranslateRotate {
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
            })
        }
    }
}

impl<T> Transform for ScaleTranslateRotate<T>
where
    T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
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
