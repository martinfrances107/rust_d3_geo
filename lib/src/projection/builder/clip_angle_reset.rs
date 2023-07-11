use core::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip;
use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleU;
use crate::projection::ClipAngleReset;

use super::Builder;

impl<DRAIN, PCNU, PR, RC, RU, T> ClipAngleReset
    for Builder<ClipCircleU<RC, T>, DRAIN, PCNU, PR, RU, T>
where
    T: 'static + CoordFloat + Default + FloatConst,
{
    /// The resultant builder type.
    type Output = Builder<ClipAntimeridianU<RC, T>, DRAIN, PCNU, PR, RU, T>;
    type T = T;

    // Set the internal clip angle (theta) to null and return a builder
    // which uses the antimeridian clipping strategy.
    #[inline]
    fn clip_angle_reset(self) -> Self::Output {
        // update only theta and preclip_factory.
        Self::Output {
            p_d: PhantomData::<DRAIN>,
            clip: gen_clip::<RC, T>(),
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,
            rotator: self.rotator,
            projection_raw: self.projection_raw,
            postclip: self.postclip,
            x: self.x,
            y: self.y,
            resample: self.resample,
            delta2: self.delta2,
            lambda: self.lambda,
            phi: self.phi,
            t360: self.t360,

            alpha: self.alpha,
            k: self.k,
            theta: None,
            sx: self.sx,
            sy: self.sy,
            rotate: self.rotate,
            project_transform: self.project_transform,
            project_rotate_transform: self.project_rotate_transform,
        }
    }
}
