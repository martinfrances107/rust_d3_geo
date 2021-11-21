/// Holds the clip circle interpolate function.
pub mod interpolate;
/// Holds the clip circle line function.
pub mod line;
/// Holds the clip circle point visible function.
pub mod pv;

/// Intersects the great circle between a and b with the clip circle.
pub mod intersect;

use std::fmt::Debug;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::line::Line;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;

use interpolate::generate as generate_interpolate;
use line::Line as LineCircle;
use pv::PV;

pub(crate) fn gen_clip_factory_circle<EP, PR, SINK, T>(
    radius: T,
) -> StreamNodeClipFactory<EP, PR, PV<T>, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    PR: ProjectionRaw<T>,
    SINK: Stream<EP = EP, T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    let cr = radius.cos();
    let small_radius = cr > T::zero();
    let start = if small_radius {
        [T::zero(), -radius]
    } else {
        [-T::PI(), radius - T::PI()]
    };
    StreamNodeClipFactory::new(
        PV::new(radius),
        Line::C(LineCircle::new(radius)),
        generate_interpolate(radius),
        start.into(),
    )
}
