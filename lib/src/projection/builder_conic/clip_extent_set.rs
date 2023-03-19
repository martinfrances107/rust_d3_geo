use std::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip as gen_clip_antimeridian;
use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::circle::gen_clip as gen_clip_circle;
use crate::clip::circle::ClipCircleC;
use crate::clip::rectangle::Rectangle;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::builder::types::BuilderAntimeridianResampleClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::builder::types::BuilderCircleResampleClip;
use crate::projection::builder::types::BuilderCircleResampleNoneClip;

use crate::projection::ClipExtentSet;

use super::types::BuilderConicAntimeridianResampleClip;
use super::types::BuilderConicAntimeridianResampleNoClip;

// Code Repeated 2^2 times.
// Variantion over ClipAntimeridian/ClipCircle as Clip is rebuilt.
// Varariantion over Resample/None as Resample is rebuilt.
impl<DRAIN, PR, T> ClipExtentSet for BuilderConicAntimeridianResampleNoClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + Default + FloatConst,
{
    type Output = BuilderConicAntimeridianResampleClip<DRAIN, PR, T>;
    type T = T;

    #[inline]
    fn clip_extent_set(&self, extent: &[Coord<T>; 2]) -> Self::Output {
        Self::Output {
            base: self.base.clip_extent_set(extent),

            phi0: self.phi0,
            phi1: self.phi1,
        }
    }
}
