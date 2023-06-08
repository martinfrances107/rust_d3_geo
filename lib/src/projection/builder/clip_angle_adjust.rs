use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::circle::gen_clip;
use crate::clip::circle::ClipCircleU;
use crate::projection::ClipAngleAdjust;

use super::Builder;

impl<DRAIN, PCNU, PR, RC, RU, T> ClipAngleAdjust
    for Builder<ClipCircleU<RC, T>, DRAIN, PCNU, PR, RU, T>
where
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;

    fn clip_angle(&mut self, angle: T) -> &mut Self {
        debug_assert!(angle != T::zero(), "must call clip_angle_reset() instead");
        let theta = angle.to_radians();
        let clip = gen_clip::<RC, T>(theta);

        self.clip = clip;
        self.theta = Some(angle);

        self
    }
}
