/// Holds the clip circle interpolate function.
pub mod interpolate;
/// Intersects the great circle between a and b with the clip circle.
pub mod intersect;
/// Holds the clip circle line function.
pub mod line;
/// Holds the clip circle point visible function.
pub mod pv;

use crate::stream::Stream;
use approx::AbsDiffEq;
use geo::CoordFloat;
use interpolate::Interpolate;
use line::Line;
use num_traits::FloatConst;

use crate::clip::Buffer;
use crate::stream::Connected;
use crate::stream::Unconnected;
use pv::PV;

use super::clip::Clip;

type ClipCircleU<PR, RC, RU, T> = Clip<
	Interpolate<T>,
	Line<Buffer<T>, Connected<Buffer<T>>, T>,
	Line<RC, Connected<RC>, T>,
	Line<RC, Unconnected, T>,
	PR,
	PV<T>,
	RC,
	RU,
	Unconnected,
	T,
>;

/// Returns a clip setup for circle clipping.
pub fn gen_clip_circle<DRAIN, PCNC, PCNU, PR, RC, RU, T>(radius: T) -> ClipCircleU<PR, RC, RU, T>
where
	DRAIN: Stream<EP = DRAIN, T = T>,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	let cr = radius.cos();
	let small_radius = cr > T::zero();
	let start = if small_radius {
		[T::zero(), -radius]
	} else {
		[-T::PI(), radius - T::PI()]
	};

	Clip::new(
		Interpolate::new(radius),
		Line::new(radius),
		PV::new(radius),
		start.into(),
	)
}
