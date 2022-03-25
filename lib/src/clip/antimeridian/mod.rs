/// Holds the clip antimeridian interpolate function.
pub mod interpolate;
/// Holds the clip antimeridian line function.
pub mod line;
/// Holds the clip antimeridian point visible function.
pub mod pv;

mod intersect;
// mod template;
use crate::clip::Buffer;
use crate::stream::Connected;
use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::ProjectionRawBase;
use crate::stream::Unconnected;
use line::Line;
use pv::PV;

use super::clip::Clip;
use interpolate::Interpolate;
// // use template::Default;

// None<DRAIN, PR, NoClipC<DRAIN, T>, NoClipU<DRAIN, T>, Connected<NoClipC<DRAIN, T>>, T>

/// Returns a clip setup for antimeridian clipping.
pub fn gen_clip_antimeridian<DRAIN, PCNC, PCNU, PR, RC, RU, T>() -> Clip<
    DRAIN,
    Interpolate<DRAIN, RC, T>,
    Line<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
    Line<DRAIN, RC, Connected<RC>, T>,
    Line<DRAIN, RC, Unconnected, T>,
    PR,
    PV<T>,
    RC,
    RU,
    Unconnected,
    T,
>
where
    PR: ProjectionRawBase<T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    let interpolate = Interpolate::default();
    let clip_line: Line<DRAIN, RC, Unconnected, T> = Line::default();
    let pv = PV::default();
    let out: Clip<
        DRAIN,
        Interpolate<DRAIN, RC, T>,
        Line<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
        Line<DRAIN, RC, Connected<RC>, T>,
        Line<DRAIN, RC, Unconnected, T>,
        PR,
        PV<T>,
        RC,
        RU,
        Unconnected,
        T,
    > = Clip::new(
        interpolate,
        clip_line,
        pv,
        [-T::PI(), -T::FRAC_PI_2()].into(),
    );

    out
}
