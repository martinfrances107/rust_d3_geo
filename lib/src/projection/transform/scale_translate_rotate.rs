use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::Transform;

use super::st::St;
use super::str::Str;
/// The scale translate rotate transform can be optimiised into
/// a faster variant, just scale translate.
#[derive(Clone, Copy, Debug)]
pub enum ScaleTranslateRotate<T> {
    /// Scale Translate.
    ST(St<T>),
    /// Scale Translate Rotate.
    STR(Str<T>),
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
