use core::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use super::Builder;
use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::gen_clip;
use crate::clip::circle::ClipCircleU;

use crate::projection::ClipAngleSet;

impl<DRAIN, PCNU, PR, RC, RU, T> ClipAngleSet
    for Builder<ClipAntimeridianU<RC, T>, DRAIN, PCNU, PR, RU, T>
where
    PCNU: Clone,
    PR: Clone,
    RU: Clone,
    T: 'static + CoordFloat + FloatConst,
{
    type Output = Builder<ClipCircleU<RC, T>, DRAIN, PCNU, PR, RU, T>;
    type T = T;

    // Given an angle in degrees. Sets the internal clip angle and returns a builder
    // which uses the clip circle stratergy.
    fn clip_angle_set(&self, angle: T) -> Self::Output {
        debug_assert!(angle != T::zero(), "must call clip_angle_reset() instead");

        let theta = angle.to_radians();
        let clip = gen_clip::<RC, T>(theta);
        // Copy, Mutate - updating only theta and preclip_factory.
        Self::Output {
            p_d: PhantomData::<DRAIN>,
            projection_raw: self.projection_raw.clone(),
            clip,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,
            x: self.x,
            y: self.y,
            t360: self.t360,

            delta2: self.delta2,
            lambda: self.lambda,
            phi: self.phi,

            alpha: self.alpha,
            k: self.k,

            theta: Some(theta),

            sx: self.sx,
            sy: self.sy,

            rotate: self.rotate.clone(),
            project_transform: self.project_transform.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            postclip: self.postclip.clone(),

            resample: self.resample.clone(),
            rotator: self.rotator.clone(),
        }
    }
}
