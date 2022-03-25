/// Holds the clip circle interpolate function.
pub mod interpolate;
/// Intersects the great circle between a and b with the clip circle.
pub mod intersect;
/// Holds the clip circle line function.
pub mod line;
/// Holds the clip circle point visible function.
pub mod pv;
/// Type for default clip.
pub mod template;

use std::fmt::Debug;

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

/// Returns a clip setup for circle clipping.
pub fn gen_clip_circle<DRAIN, PCNC, PCNU, PR, RC, RU, T>(
	radius: T,
) -> Clip<
	DRAIN,
	Interpolate<DRAIN, RC, T>,
	Line<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
	Line<DRAIN, RC, Connected<RC>, T>,
	Line<DRAIN, RC, Unconnected, T>,
	PR,
	PV<T>,
	RC,
	RU,
	Unconnected,
	T,
>
where
	DRAIN: Clone + Debug,
	PCNC: Clone + Debug,
	PCNU: Clone + Debug,
	RC: Clone + Debug,
	RU: Clone + Debug,
	PR: Clone,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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
