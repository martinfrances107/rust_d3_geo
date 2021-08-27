/// Holds the clip circle interpolate function.
pub mod interpolate;
/// Holds the clip circle line function.
pub mod line;
/// Holds the clip circle point visible function.
pub mod pv;

mod intersect;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;

use interpolate::generate as generate_interpolate;
use line::Line;
use pv::PV;

pub(crate) fn gen_clip_factory_circle<PR, SINK, T>(
    radius: T,
) -> StreamNodeClipFactory<Line<T>, PR, PV<T>, SINK, T>
where
    PR: ProjectionRaw<T>,
    SINK: Stream<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    StreamNodeClipFactory::new(
        generate_interpolate(radius),
        Line::new(radius),
        PV::new(radius),
    )
}
