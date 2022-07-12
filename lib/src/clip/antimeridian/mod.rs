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
pub fn gen_clip_antimeridian<PCNU, RC, T>() -> ClipAntimeridianU<RC, T>
where
    T: CoordFloat + FloatConst,
{
    let interpolate = Interpolate::default();
    let clip_line: Line<RC, Unconnected, T> = Line::default();
    let pv = PV::default();
    let out = Clip::new(
        interpolate,
        clip_line,
        pv,
        [-T::PI(), -T::FRAC_PI_2()].into(),
    );

    out
}
