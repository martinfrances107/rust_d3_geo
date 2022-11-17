use std::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::TransformExtent;
use crate::projection::TranslateSet;
use crate::stream::Stream;
use crate::Transform;

use super::types::BuilderMercatorAntimeridianResampleClip;
use super::types::BuilderMercatorAntimeridianResampleNoneClip;
use super::Reclip;

impl<DRAIN, PR, T> TranslateSet for BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn translate_set(&mut self, t: &Coord<T>) -> &mut Self {
        self.base.translate_set(t);
        self.reclip()
    }
}

impl<DRAIN, PR, T> TranslateSet for BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T>
where
    DRAIN: Clone + Default + Debug + Stream<EP = DRAIN, T = T>,
    PR: Clone + Debug + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn translate_set(&mut self, t: &Coord<T>) -> &mut Self {
        self.base.translate_set(t);
        self.reclip()
    }
}
