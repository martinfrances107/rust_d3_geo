/// Holds the clip antimeridian interpolate function.
pub mod interpolate;
/// Holds the clip antimeridian line function.
pub mod line;
/// Holds the clip antimeridian point visible function.
pub mod pv;

mod intersect;

use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::line::Line;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;
use interpolate::generate as gen_interpolate;
use line::Line as LineAntimeridian;

use pv::PV;

/// Returns a clip factory setup for antimeridian clipping.
pub fn gen_clip_factory_antimeridian<EP, PR, SINK, T>(
) -> StreamNodeClipFactory<EP, PR, PV<T>, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    PR: ProjectionRaw<T>,
    SINK: Stream<EP = EP, T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    StreamNodeClipFactory::new(
        PV::default(),
        Line::A(LineAntimeridian::default()),
        gen_interpolate::<SINK, T>(),
        [-T::PI(), -T::FRAC_PI_2()].into(),
    )
}
