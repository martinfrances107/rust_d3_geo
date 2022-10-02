use std::marker::PhantomData;

use super::types::BuilderMercatorAntimeridianResampleClip;
use super::types::BuilderMercatorAntimeridianResampleNoneClip;
use super::types::BuilderMercatorCircleResampleClip;
use super::types::BuilderMercatorCircleResampleNoneClip;
use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::circle::ClipCircleC;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::PrecisionBypass;
use geo::CoordFloat;
use num_traits::FloatConst;

impl<DRAIN, PR, T> PrecisionBypass for BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + Default + FloatConst,
{
    type Output = BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    fn precision_bypass(&self) -> Self::Output {
        let base = self.base.precision_bypass();
        Self::Output {
            p_clipc: PhantomData::<ClipAntimeridianC<ResampleNonePCNC<DRAIN, PR, T>, T>>,
            p_drain: PhantomData::<DRAIN>,
            p_rc: PhantomData::<ResampleNonePCNC<DRAIN, PR, T>>,
            extent: self.extent, // post-clip extent
            pr: self.pr.clone(),
            base,
        }
    }
}

impl<DRAIN, PR, T> PrecisionBypass for BuilderMercatorCircleResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + FloatConst,
{
    type Output = BuilderMercatorCircleResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    #[inline]
    fn precision_bypass(&self) -> Self::Output {
        Self::Output {
            p_clipc: PhantomData::<ClipCircleC<ResampleNonePCNC<DRAIN, PR, T>, T>>,
            p_drain: PhantomData::<DRAIN>,
            p_rc: PhantomData::<ResampleNonePCNC<DRAIN, PR, T>>,
            extent: self.extent, // post-clip extent
            pr: self.pr.clone(),
            base: self.base.precision_bypass(),
        }
    }
}
