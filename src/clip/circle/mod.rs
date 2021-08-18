pub mod interpolate;
mod intersect;
pub mod line;
pub mod pv;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;
use crate::Transform;

use super::circle::interpolate::generate as generate_interpolate;
use super::circle::line::Line;
use super::circle::pv::PV;

pub(crate) fn gen_clip_factory_circle<PR, SINK, T>(
    radius: T,
) -> StreamNodeClipFactory<Line<T>, PR, PV<T>, SINK, T>
where
    PR: ProjectionRaw<T = T> + Transform<T = T>,
    SINK: Stream<T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    StreamNodeClipFactory::new(
        generate_interpolate(radius),
        Line::new(radius),
        // PV { cr: radius.cos() },
        PV::new(radius),
    )
}
