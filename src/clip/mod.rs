pub mod antimeridian;
pub mod buffer;
pub mod circle;
pub mod clip;
pub mod clip_base;
pub mod clip_raw;
pub mod clip_sink_enum;
pub mod line_enum;
pub mod line_sink_enum;

mod rejoin;

use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::stream::Stream;

use buffer::ClipBuffer;

pub trait ClipTraitRaw<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    type SctC;
    type SctOC;
    type SctT: CoordFloat + FloatConst;
    type SctCi;

    fn point_visible(&self, _p: &Self::SctC, _z: Option<u8>) -> bool;

    // Intersections are sorted along the clip edge. For both antimeridian cutting
    // and circle clipPIng, the same comparison is used.
    fn compare_intersection(&self, _a: Self::SctCi, _b: Self::SctCi) -> Self::SctT {
        // let a_dashed = a.x;
        // let part1 = match a_dashed.x < Self::SctT::zero() {
        //     true => a_dashed.y - Self::SctT::FRAC_PI_2() - Self::SctT::epsilon(),
        //     false => Self::SctT::FRAC_PI_2() - a_dashed.y,
        // };
        // let b_dashed = b.x;
        // let part2 = match b_dashed.x < Self::SctT::zero() {
        //     true => b_dashed.y - Self::SctT::FRAC_PI_2() - Self::SctT::epsilon(),
        //     false => Self::SctT::FRAC_PI_2() - b_dashed.y,
        // };

        // return part1 - part2;
        panic!("why is this called.");
    }

    fn interpolate(
        &self,
        from: Self::SctOC,
        to: Self::SctOC,
        direction: Self::SctT,
        stream: &mut impl Stream<T, C = Coordinate<T>>,
    );
}
