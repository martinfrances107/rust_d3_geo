use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridianC;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::TransformExtent;
use crate::projection::TranslateSet;
use crate::Transform;

use super::types::BuilderMercatorAntimeridianResampleClip;
use super::types::BuilderMercatorAntimeridianResampleNoneClip;
use super::Reclip;

impl<DRAIN, PR, T> TranslateSet for BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn translate_set(&mut self, t: &Coord<T>) -> &mut Self {
        self.base.translate_set(t);
        self.reclip::<ClipAntimeridianC<ResamplePCNC<DRAIN, PR, T>, T>>()
    }
}

impl<DRAIN, PR, T> TranslateSet for BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone + Transform<T = T> + TransformExtent<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn translate_set(&mut self, t: &Coord<T>) -> &mut Self {
        self.base.translate_set(t);
        self.reclip::<ClipAntimeridianC<ResampleNonePCNC<DRAIN, PR, T>, T>>()
    }
}
