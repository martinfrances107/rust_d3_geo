/// Holds the clip antimeridian interpolate function.
pub mod interpolate;
/// Holds the clip antimeridian line function.
pub mod line;
/// Holds the clip antimeridian point visible function.
pub mod pv;

mod intersect;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::stream::Connected;
use crate::stream::Unconnected;
use line::Line;
use pv::PV;

use super::clip::Clip;
use interpolate::Interpolate;

type ClipAntimeridianU<RC, T> = Clip<
    Interpolate<T>,
    Line<RC, Connected<RC>, T>,
    Line<RC, Unconnected, T>,
    PV<T>,
    RC,
    Unconnected,
    T,
>;

/// Returns a clip setup for antimeridian clipping.
#[inline]
pub fn gen_clip_antimeridian<PCNU, RC, T>() -> ClipAntimeridianU<RC, T>
where
    T: CoordFloat + FloatConst,
{
    Clip::new(
        Interpolate::default(),
        Line::default(),
        PV::default(),
        [-T::PI(), -T::FRAC_PI_2()].into(),
    )
}
