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

// use geo::CoordFloat;
// use num_traits::FloatConst;

// use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
// use crate::projection::ProjectionRawBase;
// use crate::stream::Stream;

// use crate::projection::resampler::none::None;
// use crate::stream::Connected;
use crate::clip::Buffer;
use crate::stream::Connected;
use crate::Transform;
use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use interpolate::Interpolate;
use line::Line;
use num_traits::FloatConst;
use pv::PV;
// use template::Default as DefaultCircle;

// use crate::projection::builder::template::ResampleNoneNoClipU;
// use crate::projection::ProjectionRawBase;
// use crate::stream::Connected;
use crate::stream::Unconnected;

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
	// PR: ProjectionRawBase<T>,
	RC: Clone + Debug,
	RU: Clone + Debug,
	PR: Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	let cr = radius.cos();
	let small_radius = cr > T::zero();
	let start = if small_radius {
		[T::zero(), -radius]
	} else {
		[-T::PI(), radius - T::PI()]
	};

	let out: Clip<
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
	> = Clip::new(
		Interpolate::new(radius),
		Line::new(radius),
		PV::new(radius),
		start.into(),
	);

	out
}
