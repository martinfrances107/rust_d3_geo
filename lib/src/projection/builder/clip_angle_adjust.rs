use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::circle::gen_clip_circle;
use crate::clip::circle::ClipCircleC;
use crate::clip::circle::ClipCircleU;
use crate::projection::ClipAngleAdjust;

use super::Builder;

impl<DRAIN, PCNU, PR, RC, RU, T> ClipAngleAdjust
    for Builder<ClipCircleC<RC, T>, ClipCircleU<RC, T>, DRAIN, PCNU, PR, RC, RU, T>
where
    RC: Clone,
    T: CoordFloat + FloatConst,
{
    type T = T;

    fn clip_angle(&mut self, angle: T) -> &mut Self {
        if angle == T::zero() {
            panic!("must call clip_angle_reset() instead");
        }
        let theta = angle.to_radians();
        let clip = gen_clip_circle::<DRAIN, PCNU, PR, RC, RU, T>(theta);

        self.clip = clip;
        self.theta = Some(angle);

        self
    }
}
