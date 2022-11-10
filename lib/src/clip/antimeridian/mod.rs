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

use super::buffer::Buffer;
use super::clip::Clip;
use super::clip::Connected as ConnectedClip;
use interpolate::Interpolate;

/// Connected clip type using antimerdian interpolator, `point_visble` function line handler.
pub type ClipAntimeridianC<RC, T> = Clip<
    Interpolate<T>,
    Line<Connected<RC>, T>,
    Line<Unconnected, T>,
    PV<T>,
    RC,
    ConnectedClip<Line<Connected<Buffer<T>>, T>, Line<Connected<RC>, T>, T>,
    T,
>;
/// Unconnected clip type using antimerdian interpolator, `point_visble` function line handler.
pub type ClipAntimeridianU<RC, T> =
    Clip<Interpolate<T>, Line<Connected<RC>, T>, Line<Unconnected, T>, PV<T>, RC, Unconnected, T>;

/// Returns a clip setup for antimeridian clipping.
#[inline]
#[must_use]
pub fn gen_clip<PCNU, RC, T>() -> ClipAntimeridianU<RC, T>
where
    RC: Clone,
    T: CoordFloat + Default + FloatConst,
{
    Clip::new(
        Interpolate::default(),
        Line::default(),
        PV::default(),
        [-T::PI(), -T::FRAC_PI_2()].into(),
    )
}
