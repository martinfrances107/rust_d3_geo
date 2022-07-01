//! # fit_rectangle.
//!
//! 1) Removed Post Clip Rectangle.
//! 2) Perform operations.
//! 3) Restore Post Clip Rectangle
//!
//! # Elsewhere  in fit_no_rectangle.
//!
//! 1) No-op: No rectange to remove.
//! 2) Perform operations.
//! 3) SWAP -  implies inserting PostClip Rectangle.
//!

use approx::AbsDiffEq;
use num_traits::AsPrimitive;
use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;

use crate::clip::buffer::Buffer;
use crate::clip::Bufferable;
use crate::clip::Interpolator;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
use crate::path::bounds::Bounds;
use crate::path::Result;
use crate::projection::builder::template::NoClipC;
use crate::projection::ClipExtentBounded;
use crate::projection::ClipExtentSet;
use crate::projection::FitBounds;
use crate::projection::FloatConst;
use crate::projection::ScaleAdjust;
use crate::projection::TranslateAdjust;
use crate::stream::Connectable;
use crate::stream::Pipeline;
use crate::stream::Stream;
use crate::stream::Streamable;

pub(super) fn fit_rectangle<B, Bint, I, LB, LC, LU, PR, PV, RC, RU, T>(
	builder: B,
	fit_bounds: FitBounds<Bint, T>,
	object: &impl Streamable<T = T>,
) -> B
where
	B: ClipExtentBounded<OutputClear = Bint, T = T> + ScaleAdjust<T = T> + TranslateAdjust<T = T>,
	Bint: ClipExtentSet<OutputBounded = B, T = T>
		+ Pipeline<Bounds<T>, I, LB, LC, LU, PR, PV, RC, RU, T>,
	I: Clone + Interpolator<T = T>,
	LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
	LC: Clone + LineConnected<SC = RC> + Stream<EP = Bounds<T>, T = T>,
	LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T>,
	PV: PointVisible<T = T>,
	RC: Clone + Stream<EP = Bounds<T>, T = T>,
	RU: Clone + Connectable<Output = RC, SC = NoClipC<Bounds<T>, T>> + Debug,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + Debug + FloatConst,
{
	let clip = builder.get_clip_extent();
	let mut b_no_clip = builder
		.scale(T::from(150_f64).unwrap())
		.translate(&Coordinate {
			x: T::zero(),
			y: T::zero(),
		})
		.clip_extent_clear();

	let mut bounds_stream = Bounds::default();
	let mut stream_in = b_no_clip.stream(&bounds_stream);
	object.to_stream(&mut stream_in);
	let b_result = fit_bounds(bounds_stream.result(), b_no_clip);

	b_result.clip_extent(&clip.unwrap())
}

pub(super) fn fit_extent_adjust<B, Bint, I, LB, LC, LU, PR, PV, RC, RU, T>(
	builder: B,
	extent: [[T; 2]; 2],
	object: &impl Streamable<T = T>,
) -> B
where
	B: ClipExtentBounded<OutputClear = Bint, T = T> + ScaleAdjust<T = T> + TranslateAdjust<T = T>,
	Bint: ClipExtentSet<OutputBounded = B, T = T>
		+ Pipeline<Bounds<T>, I, LB, LC, LU, PR, PV, RC, RU, T>
		+ ScaleAdjust<T = T>
		+ TranslateAdjust<T = T>,
	I: Clone + Interpolator<T = T>,
	LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
	LC: Clone + LineConnected<SC = RC> + Stream<EP = Bounds<T>, T = T>,
	LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T>,
	PV: PointVisible<T = T>,
	RC: Clone + Stream<EP = Bounds<T>, T = T>,
	RU: Clone + Connectable<Output = RC, SC = NoClipC<Bounds<T>, T>> + Debug,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + Debug + FloatConst,
{
	fit_rectangle(
		builder,
		Box::new(move |b: [Coordinate<T>; 2], builder: Bint| -> Bint {
			let two = T::from(2.0_f64).unwrap();
			let one_five_zero = T::from(150_f64).unwrap();
			let w = extent[1][0] - extent[0][0];
			let h = extent[1][1] - extent[0][1];
			let k = T::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
			let x = extent[0][0] + (w - k * (b[1].x + b[0].x)) / two;
			let y = extent[0][1] + (h - k * (b[1].y + b[0].y)) / two;

			builder
				.scale(one_five_zero * k)
				.translate(&Coordinate { x, y })
		}),
		object,
	)
}
