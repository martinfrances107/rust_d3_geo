/// Holds the clip circle interpolate function.
pub mod interpolate;
/// Intersects the great circle between a and b with the clip circle.
pub mod intersect;
/// Holds the clip circle line function.
pub mod line;
/// Holds the clip circle point visible function.
pub mod pv;

use geo::CoordFloat;
use geo_types::Coord;
use interpolate::Interpolate;
use line::Line;
use num_traits::FloatConst;

use crate::stream::Connected;
use crate::stream::Unconnected;
use pv::PV;

use super::buffer::Buffer;
use super::clipper::Clipper;
use super::clipper::Connected as ConnectedClip;

/// Connected clip type using circle interpolator, `point_visble` function line handler.
pub type ClipCircleC<RC, T> = Clipper<
    Interpolate<T>,
    Line<Unconnected, T>,
    PV<T>,
    RC,
    ConnectedClip<Line<Connected<Buffer<T>>, T>, Line<Connected<RC>, T>, T>,
    T,
>;

/// Unconnected clip type using circle interpolator, `point_visble` function line handler.
pub type ClipCircleU<RC, T> =
    Clipper<Interpolate<T>, Line<Unconnected, T>, PV<T>, RC, Unconnected, T>;

/// Returns a clip setup for circle clipping.
pub fn gen_clip<RC, T>(radius: T) -> ClipCircleU<RC, T>
where
    T: CoordFloat + FloatConst,
{
    let cr = radius.cos();
    let small_radius = cr > T::zero();
    let start = if small_radius {
        Coord {
            x: T::zero(),
            y: -radius,
        }
    } else {
        Coord {
            x: -T::PI(),
            y: radius - T::PI(),
        }
    };

    Clipper::new(
        Interpolate::new(radius),
        Line::new(radius),
        PV::new(radius),
        start,
    )
}
