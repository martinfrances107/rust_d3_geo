use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::Transform;

use super::st::St;
use super::str::Str;

#[derive(Clone, Copy, Debug)]
pub enum ScaleTranslateRotate<T>
where
    T: CoordFloat + FloatConst,
{
    ST(St<T>),
    STR(Str<T>),
}

impl<T> Default for ScaleTranslateRotate<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn default() -> Self {
        ScaleTranslateRotate::ST(St::default())
    }
}

impl<T: CoordFloat> Transform for ScaleTranslateRotate<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            ScaleTranslateRotate::ST(st) => st.transform(p),
            ScaleTranslateRotate::STR(str) => str.transform(p),
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        match self {
            ScaleTranslateRotate::ST(st) => st.invert(p),
            ScaleTranslateRotate::STR(str) => str.invert(p),
        }
    }
}
