use std::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::gen_clip_antimeridian;
use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleC;
use crate::clip::circle::ClipCircleU;

use crate::projection::ClipAngleReset;

use super::Builder;

impl<DRAIN, PCNU, PR, RC, RU, T> ClipAngleReset
    for Builder<ClipCircleC<RC, T>, ClipCircleU<RC, T>, DRAIN, PCNU, PR, RC, RU, T>
where
    RC: Clone,
    T: CoordFloat + Default + FloatConst,
{
    type Output =
        Builder<ClipAntimeridianC<RC, T>, ClipAntimeridianU<RC, T>, DRAIN, PCNU, PR, RC, RU, T>;
    type T = T;

    // Set the internal clip angle (theta) to null and return a builder
    // which uses the antimeridian clipping stratergy.
    #[inline]
    fn clip_angle_reset(self) -> Self::Output {
        // update only theta and preclip_factory.
        Self::Output {
            p_clipc: PhantomData::<ClipAntimeridianC<RC, T>>,
            p_rc: PhantomData::<RC>,
            p_drain: PhantomData::<DRAIN>,
            clip: gen_clip_antimeridian::<PCNU, RC, T>(),
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,
            rotator: self.rotator,
            projection_raw: self.projection_raw,
            postclip: self.postclip,
            x: self.x,
            y: self.y,
            resample: self.resample,
            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1,
            delta2: self.delta2,
            lambda: self.lambda,
            phi: self.phi,

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
