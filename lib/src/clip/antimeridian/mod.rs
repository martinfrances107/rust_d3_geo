/// Holds the clip antimeridian interpolate function.
pub mod interpolate;
/// Holds the clip antimeridian line function.
pub mod line;

mod intersect;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::stream::Connected;
use crate::stream::Unconnected;
use line::Line;

use super::buffer::Buffer;
use super::clipper::Clipper;
use super::clipper::Connected as ConnectedClip;
use interpolate::Interpolate;

/// Connected clip type using antimerdian interpolator, `point_visble` function line handler.
pub(crate) type ClipAntimeridianC<RC, T> = Clipper<
    Interpolate<T>,
    Line<Unconnected, T>,
    RC,
    ConnectedClip<Line<Connected<Buffer<T>>, T>, Line<Connected<RC>, T>, T>,
    T,
>;
/// Unconnected clip type using antimerdian interpolator, `point_visble` function line handler.
pub(crate) type ClipAntimeridianU<RC, T> =
    Clipper<Interpolate<T>, Line<Unconnected, T>, RC, Unconnected, T>;

/// Returns a clip setup for antimeridian clipping.
#[inline]
#[must_use]
pub(crate) fn gen_clip<RC, T>() -> ClipAntimeridianU<RC, T>
where
    T: 'static + CoordFloat + Default + FloatConst,
{
    Clipper::new(
        Interpolate::default(),
        Line::default(),
        [-T::PI(), -T::FRAC_PI_2()].into(),
    )
}
