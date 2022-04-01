use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::circle::stream_fn::stream_fn;
// use crate::clip::InterpolateFn;
use crate::clip::Interpolator;
// use crate::clip::InterpolatorBuilder;
use crate::stream::Stream;
// use crate::stream::Unconnected;

// struct Builder<T> {
// 	radius: T,
// 	delta: T,
// }

/// Interpolate Circle.
#[derive(Clone, Debug)]
pub struct Interpolate<T> {
	// p_ep: PhantomData<EP>,
	// p_stream: PhantomData<STREAM>,
	radius: T,
	delta: T,
}

impl<T> Interpolate<T>
where
	T: CoordFloat + FloatConst,
{
	pub fn new(radius: T) -> Self {
		Self {
			// p_ep: PhantomData::<EP>,
			// p_stream: PhantomData::<STREAM>,
			radius,
			delta: T::from(6_f64).unwrap().to_radians(),
		}
	}
}

// impl<T> InterpolatorBuilder for Builder<T> {
// 	/// Sets up a clip circle interpolate function, for a given radius.
// 	fn build<EP, STREAM>(&self, stream: STREAM) -> Interpolate<EP, STREAM, T>
// 	where
// 		EP: Stream<EP = EP, T = T> + Default,
// 		STREAM: Stream<EP = EP, T = T>,
// 		T: 'static + CoordFloat + FloatConst,
// 	{
// 		Interpolate {
// 			p_ep: PhantomData::<EP>,
// 			p_stream: PhantomData::<STREAM>,
// 			radius: self.radius,
// 			delta: self.delta,
// 		}
// 	}
// }

impl<T> Interpolator for Interpolate<T>
where
	// STREAM: Stream<EP = EP, T = T>,
	T: CoordFloat + FloatConst,
{
	// type EP = EP;
	// type Stream = STREAM;
	type T = T;
	fn interpolate<EP, STREAM>(
		&mut self,
		from: Option<Coordinate<T>>,
		to: Option<Coordinate<T>>,
		direction: T,
		stream: &mut STREAM,
	) where
		STREAM: Stream<EP = EP, T = T>,
	{
		stream_fn(stream, self.radius, self.delta, direction, from, to);
	}
}
