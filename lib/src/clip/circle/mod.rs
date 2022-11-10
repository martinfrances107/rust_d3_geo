/// Holds the clip circle interpolate function.
pub mod interpolate;
/// Intersects the great circle between a and b with the clip circle.
pub mod intersect;
/// Holds the clip circle line function.
pub mod line;
/// Holds the clip circle point visible function.
pub mod pv;

use geo::CoordFloat;
use geo::Coordinate;
use interpolate::Interpolate;
use line::Line;
use num_traits::FloatConst;

use crate::stream::Connected;
use crate::stream::Unconnected;
use pv::PV;

use super::buffer::Buffer;
use super::clip::Clip;
use super::clip::Connected as ConnectedClip;

/// Connected clip type using circle interpolator, `point_visble` function line handler.
pub type ClipCircleC<RC, T> = Clip<
    Interpolate<T>,
    Line<Connected<RC>, T>,
    Line<Unconnected, T>,
    PV<T>,
    RC,
    ConnectedClip<Line<Connected<Buffer<T>>, T>, Line<Connected<RC>, T>, T>,
    T,
>;

/// Unconnected clip type using circle interpolator, `point_visble` function line handler.
pub type ClipCircleU<RC, T> =
    Clip<Interpolate<T>, Line<Connected<RC>, T>, Line<Unconnected, T>, PV<T>, RC, Unconnected, T>;

/// Returns a clip setup for circle clipping.
pub fn gen_clip<DRAIN, PCNU, PR, RC, RU, T>(radius: T) -> ClipCircleU<RC, T>
where
    RC: Clone,
    T: CoordFloat + FloatConst,
{
    let cr = radius.cos();
    let small_radius = cr > T::zero();
    let start = if small_radius {
        Coordinate {
            x: T::zero(),
            y: -radius,
        }
    } else {
        Coordinate {
            x: -T::PI(),
            y: radius - T::PI(),
        }
    };

    Clip::new(
        Interpolate::new(radius),
        Line::new(radius),
        PV::new(radius),
        start,
    )
}
