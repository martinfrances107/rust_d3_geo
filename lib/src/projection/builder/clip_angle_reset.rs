use std::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip;
use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleC;
use crate::clip::circle::ClipCircleU;
use crate::projection::ClipAngleReset;

use super::Builder;

impl<PCNU, PR, RC, RU, T> ClipAngleReset
    for Builder<ClipCircleC<RC, T>, ClipCircleU<RC, T>, PCNU, PR, RU, T>
where
    RC: Clone,
    T: CoordFloat + Default + FloatConst,
{
    /// The resultant builder type.
    type Output = Builder<ClipAntimeridianC<RC, T>, ClipAntimeridianU<RC, T>, PCNU, PR, RU, T>;
    type T = T;

    // Set the internal clip angle (theta) to null and return a builder
    // which uses the antimeridian clipping stratergy.
    #[inline]
    fn clip_angle_reset(self) -> Self::Output {
        // update only theta and preclip_factory.
        Self::Output {
            p_clipc: PhantomData::<ClipAntimeridianC<RC, T>>,
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
