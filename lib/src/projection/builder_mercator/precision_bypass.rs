use core::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use super::types::BuilderMercatorAntimeridianResampleClip;
use super::types::BuilderMercatorAntimeridianResampleNoneClip;
use super::types::BuilderMercatorCircleResampleClip;
use super::types::BuilderMercatorCircleResampleNoneClip;
use crate::projection::PrecisionBypass;

impl<DRAIN, PR, T> PrecisionBypass
    for BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + Default + FloatConst,
{
    type Output = BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    // Switch the builder into one without a resample node,
    fn precision_bypass(&self) -> Self::Output {
        let base = self.base.precision_bypass();
        Self::Output {
            p_d: PhantomData::<DRAIN>,
            extent: self.extent, // post-clip extent
            pr: self.pr.clone(),
            base,
        }
    }
}

impl<DRAIN, PR, T> PrecisionBypass
    for BuilderMercatorCircleResampleClip<DRAIN, PR, T>
where
    PR: Clone,
    T: 'static + CoordFloat + FloatConst,
{
    type Output = BuilderMercatorCircleResampleNoneClip<DRAIN, PR, T>;
    type T = T;

    #[inline]
    // Switch the builder into one without a resample node,
    fn precision_bypass(&self) -> Self::Output {
        Self::Output {
            p_d: PhantomData::<DRAIN>,
            extent: self.extent, // post-clip extent
            pr: self.pr.clone(),
            base: self.base.precision_bypass(),
        }
    }
}
