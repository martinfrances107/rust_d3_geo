use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::TranslateSet;
use crate::Transform;

use super::Builder;

impl<DRAIN, PR, T> TranslateSet for Builder<BuilderAntimeridianResampleNoClip<DRAIN, PR, T>, PR, T>
where
    DRAIN: Clone,
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn translate_set(&mut self, t: &Coord<T>) -> &mut Self {
        self.base.translate_set(t);
        self
    }
}
