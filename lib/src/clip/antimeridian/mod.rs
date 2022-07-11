/// Holds the clip antimeridian interpolate function.
pub mod interpolate;
/// Holds the clip antimeridian line function.
pub mod line;
/// Holds the clip antimeridian point visible function.
pub mod pv;

mod intersect;

use crate::stream::Stream;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::Buffer;
use crate::stream::Connected;
use crate::stream::Unconnected;
use line::Line;
use pv::PV;

use super::clip::Clip;
use interpolate::Interpolate;

type ClipAntimeridianU<PR, RC, RU, T> = Clip<
    Interpolate<T>,
    Line<Buffer<T>, Connected<Buffer<T>>, T>,
    Line<RC, Connected<RC>, T>,
    Line<RC, Unconnected, T>,
    PR,
    PV<T>,
    RC,
    RU,
    Unconnected,
    T,
>;

/// Returns a clip setup for antimeridian clipping.
pub fn gen_clip_antimeridian<PCNC, PCNU, PR, RC, RU, T>() -> ClipAntimeridianU<PR, RC, RU, T>
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
