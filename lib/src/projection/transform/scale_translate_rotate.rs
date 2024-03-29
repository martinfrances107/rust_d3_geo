use geo::CoordFloat;
use geo_types::Coord;

use crate::Transform;

use super::st::St;
use super::str::Str;

/// The scale translate rotate transform can be optimiised into
/// a faster variant, just scale translate.
#[derive(Clone, Debug)]
pub enum ScaleTranslateRotate<T> {
    /// Scale Translate.
    ST(St<T>),
    /// Scale Translate Rotate.
    STR(Str<T>),
}

impl<T> Transform for ScaleTranslateRotate<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        match self {
            Self::ST(st) => st.transform(p),
            Self::STR(str) => str.transform(p),
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        match self {
            Self::ST(st) => st.invert(p),
            Self::STR(str) => str.invert(p),
        }
    }
}
