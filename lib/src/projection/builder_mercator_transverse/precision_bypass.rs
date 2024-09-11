use geo::CoordFloat;
use num_traits::FloatConst;

use super::types::BuilderMercatorTransverseAntimeridianResampleClip;
use super::types::BuilderMercatorTransverseAntimeridianResampleNoneClip;
use super::types::BuilderMercatorTransverseCircleResampleClip;
use super::types::BuilderMercatorTransverseCircleResampleNoneClip;
use crate::projection::PrecisionBypass;

impl<DRAIN, PR, T> PrecisionBypass
    for BuilderMercatorTransverseAntimeridianResampleClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + Default + FloatConst,
{
    type Output =
        BuilderMercatorTransverseAntimeridianResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    // Switch the builder into one without a resample node,
    fn precision_bypass(&self) -> Self::Output {
        let base = self.base.precision_bypass();
        Self::Output { base }
    }
}

impl<DRAIN, PR, T> PrecisionBypass
    for BuilderMercatorTransverseCircleResampleClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + FloatConst,
{
    type Output = BuilderMercatorTransverseCircleResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    #[inline]
    // Switch the builder into one without a resample node,
    fn precision_bypass(&self) -> Self::Output {
        Self::Output {
            base: self.base.precision_bypass(),
        }
    }
}
