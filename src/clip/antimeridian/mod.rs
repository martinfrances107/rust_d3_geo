/// Holds the clip antimeridian interpolate function.
pub mod interpolate;
/// Holds the clip antimeridian line function.
pub mod line;
/// Holds the clip antimeridian point visible function.
pub mod pv;

mod intersect;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;

use interpolate::generate as gen_interpolate;
use line::Line;
use pv::PV;

/// Returns a clip factory setup for antimeridian clipping.
pub fn gen_clip_factory_antimeridian<PR, SINK, T>(
) -> StreamNodeClipFactory<Line<T>, PR, PV<T>, SINK, T>
where
    PR: ProjectionRaw<T>,
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    StreamNodeClipFactory::new(gen_interpolate::<SINK, T>(), Line::default(), PV::default())
}
