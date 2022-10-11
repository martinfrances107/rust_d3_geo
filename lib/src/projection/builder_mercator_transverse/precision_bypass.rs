use std::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use super::types::BuilderMercatorTransverseAntimeridianResampleClip;
use super::types::BuilderMercatorTransverseAntimeridianResampleNoneClip;
use super::types::BuilderMercatorTransverseCircleResampleClip;
use super::types::BuilderMercatorTransverseCircleResampleNoneClip;
use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::circle::ClipCircleC;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::PrecisionBypass;

impl<DRAIN, PR, T> PrecisionBypass
    for BuilderMercatorTransverseAntimeridianResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + Default + FloatConst,
{
    type Output = BuilderMercatorTransverseAntimeridianResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    fn precision_bypass(&self) -> Self::Output {
        let base = self.base.precision_bypass();
        Self::Output {
            p_clipc: PhantomData::<ClipAntimeridianC<ResampleNonePCNC<DRAIN, PR, T>, T>>,
            p_drain: PhantomData::<DRAIN>,
            p_rc: PhantomData::<ResampleNonePCNC<DRAIN, PR, T>>,
            base,
        }
    }
}

impl<DRAIN, PR, T> PrecisionBypass for BuilderMercatorTransverseCircleResampleClip<DRAIN, PR, T>
where
    DRAIN: Clone,
    PR: Clone,
    T: CoordFloat + FloatConst,
{
    type Output = BuilderMercatorTransverseCircleResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    #[inline]
    fn precision_bypass(&self) -> Self::Output {
        Self::Output {
            p_clipc: PhantomData::<ClipCircleC<ResampleNonePCNC<DRAIN, PR, T>, T>>,
            p_drain: PhantomData::<DRAIN>,
            p_rc: PhantomData::<ResampleNonePCNC<DRAIN, PR, T>>,
            base: self.base.precision_bypass(),
        }
    }
}
