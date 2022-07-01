//! # fit_no_rectangle.
//!
//! 1) No-op: No rectange to remove.
//! 2) Perform operations.
//! 3) SWAP -  NoClip for Clip
//!
//! # Elsewhere in fit_no_rectangle.
//!
//! 1) Removed Post Clip Rectangle.
//! 2) Perform operations.
//! 3) Restore Post Clip Rectangle
//!

use crate::path::bounds::Bounds;
use crate::path::Result;
use crate::projection::builder::types::BuilderAntimeridianResampleClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::builder::types::BuilderCircleResampleClip;
use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::builder::types::BuilderCircleResampleNoneClip;
use crate::projection::builder::types::BuilderCircleResampleNoneNoClip;
use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::projection::ClipExtentSet;
use crate::projection::ScaleAdjust;
use crate::projection::TranslateAdjust;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

use super::builder::Builder;

// type FitBoundsAdjust<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> = Box<
// 	dyn Fn(
// 		[Coordinate<T>; 2],
// 		Builder<DRAIN, I, LB, LC, LU, ClipC<DRAIN, T>, ClipU<DRAIN, T>, PR, PV, RC, RU, T>,
// 	) -> Builder<DRAIN, I, LB, LC, LU, ClipC<DRAIN, T>, ClipU<DRAIN, T>, PR, PV, RC, RU, T>,
// >;

// type FitBoundsConvert<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RCIn, RUIn, RCOut, RUOut, T> = Box<
// 	dyn Fn(
// 		[Coordinate<T>; 2],
// 		Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RCIn, RUIn, T>,
// 	) -> Builder<
// 		DRAIN,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		NoClipC<DRAIN, T>,
// 		NoClipU<DRAIN, T>,
// 		PR,
// 		PV,
// 		RCOut,
// 		RUOut,
// 		T,
// 	>,
// >;

fn fit_antimeridian<PR, T>(
	builder: BuilderAntimeridianResampleNoClip<Bounds<T>, PR, T>,
	fit_bounds: Box<
		dyn Fn(
			[Coordinate<T>; 2],
			BuilderAntimeridianResampleNoClip<Bounds<T>, PR, T>,
		) -> BuilderAntimeridianResampleNoClip<Bounds<T>, PR, T>,
	>,
	object: &impl Streamable<T = T>,
) -> BuilderAntimeridianResampleClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	let builder = builder
		.scale(T::from(150.0_f64).unwrap())
		.translate(&Coordinate {
			x: T::zero(),
			y: T::zero(),
		});
	let bounds_stream = Bounds::<T>::default();
	let mut stream_in = builder.build().stream(&bounds_stream);

	object.to_stream(&mut stream_in);
	let bounds = stream_in.get_endpoint().result();
	let builder = fit_bounds(bounds, builder);

	builder.clip_extent(&bounds)
}

fn fit_circle_resample_none_no_clip<PR, T>(
	builder: BuilderCircleResampleNoneNoClip<Bounds<T>, PR, T>,
	fit_bounds: Box<
		dyn Fn(
			[Coordinate<T>; 2],
			BuilderCircleResampleNoneNoClip<Bounds<T>, PR, T>,
		) -> BuilderCircleResampleNoneNoClip<Bounds<T>, PR, T>,
	>,
	object: &impl Streamable<T = T>,
) -> BuilderCircleResampleNoneClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	let builder = builder
		.scale(T::from(150.0_f64).unwrap())
		.translate(&Coordinate {
			x: T::zero(),
			y: T::zero(),
		});
	let bounds_stream = Bounds::<T>::default();
	let mut stream_in = builder.build().stream(&bounds_stream);

	object.to_stream(&mut stream_in);
	let bounds = stream_in.get_endpoint().result();
	let builder = fit_bounds(bounds, builder);

	builder.clip_extent(&bounds)
}

fn fit_circle_resample_no_clip<PR, T>(
	builder: BuilderCircleResampleNoClip<Bounds<T>, PR, T>,
	fit_bounds: Box<
		dyn Fn(
			[Coordinate<T>; 2],
			BuilderCircleResampleNoClip<Bounds<T>, PR, T>,
		) -> BuilderCircleResampleNoClip<Bounds<T>, PR, T>,
	>,
	object: &impl Streamable<T = T>,
) -> BuilderCircleResampleClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	let builder = builder
		.scale(T::from(150.0_f64).unwrap())
		.translate(&Coordinate {
			x: T::zero(),
			y: T::zero(),
		});
	let bounds_stream = Bounds::<T>::default();
	let mut stream_in = builder.build().stream(&bounds_stream);

	object.to_stream(&mut stream_in);
	let bounds = stream_in.get_endpoint().result();
	let builder = fit_bounds(bounds, builder);

	builder.clip_extent(&bounds)
}

fn fit_antimeridian_resample_no_clip<PR, T>(
	builder: BuilderAntimeridianResampleNoClip<Bounds<T>, PR, T>,
	fit_bounds: Box<
		dyn Fn(
			[Coordinate<T>; 2],
			BuilderAntimeridianResampleNoClip<Bounds<T>, PR, T>,
		) -> BuilderAntimeridianResampleNoClip<Bounds<T>, PR, T>,
	>,
	object: &impl Streamable<T = T>,
) -> BuilderAntimeridianResampleClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	let builder = builder
		.scale(T::from(150.0_f64).unwrap())
		.translate(&Coordinate {
			x: T::zero(),
			y: T::zero(),
		});
	let bounds_stream = Bounds::<T>::default();
	let mut stream_in = builder.build().stream(&bounds_stream);

	object.to_stream(&mut stream_in);
	let bounds = stream_in.get_endpoint().result();
	let builder = fit_bounds(bounds, builder);

	builder.clip_extent(&bounds)
}

// fn fit_convert<I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, RCOut, RUOut, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		ClipC<Bounds<T>, T>,
// 		ClipU<Bounds<T>, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>,
// 	fit_bounds: FitBoundsConvert<Bounds<T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, RC, RU, T>,
// 	object: &impl Streamable<T = T>,
// ) -> Builder<Bounds<T>, I, LB, LC, LU, ClipC<Bounds<T>, T>, ClipU<Bounds<T>, T>, PR, PV, RC, RU, T>
// where
// 	I: Clone,
// 	LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
// 	LC: Clone + LineConnected<SC = RC> + Stream<EP = Bounds<T>, T = T>,
// 	LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T> + Debug,
// 	PV: Clone + Debug,
// 	RC: Clone + Debug + Stream<EP = Bounds<T>, T = T>,
// 	RU: Clone + Debug + Connectable<Output = RC, SC = NoClipC<Bounds<T>, T>> + Debug,
// 	PR: Transform<T = T>,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	// let clip = builder.get_clip_extent();
// 	let builder1 = builder
// 		.scale(T::from(150.0_f64).unwrap())
// 		.translate(&Coordinate {
// 			x: T::zero(),
// 			y: T::zero(),
// 		});

// 	let builder2 = builder1;

// 	let bounds_stream = Bounds::default();
// 	let mut stream_in = builder2.build().stream(bounds_stream);

// 	object.to_stream(&mut stream_in);
// 	let bounds = stream_in.get_endpoint().result();
// 	fit_bounds(bounds, builder2)
// }

// InterpolateAntimeridian<T>,
// LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
// LineAntimeridian<
// 	DRAIN,
// 	ResampleClipC<DRAIN, Mercator<DRAIN, T>, T>,
// 	Connected<ResampleClipC<DRAIN, Mercator<DRAIN, T>, T>>,
// 	T,
// >,
// LineAntimeridian<DRAIN, ResampleClipC<DRAIN, Mercator<DRAIN, T>, T>, Unconnected, T>,

pub(super) fn fit_extent_antimerdian<PR, T>(
	builder: BuilderAntimeridianResampleNoClip<Bounds<T>, PR, T>,
	extent: [[T; 2]; 2],
	object: &impl Streamable<T = T>,
) -> BuilderAntimeridianResampleClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	let two = T::from(2.0_f64).unwrap();
	let one_five_zero = T::from(150_f64).unwrap();

	fit_antimeridian(
		builder,
		Box::new(
			move |b: [Coordinate<T>; 2],
			      builder: BuilderAntimeridianResampleNoClip<Bounds<T>, PR, T>| {
				let w = extent[1][0] - extent[0][0];
				let h = extent[1][1] - extent[0][1];
				let k = T::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
				let x = extent[0][0] + (w - k * (b[1].x + b[0].x)) / two;
				let y = extent[0][1] + (h - k * (b[1].y + b[0].y)) / two;

				builder
					.scale(one_five_zero * k)
					.translate(&Coordinate { x, y })
			},
		),
		object,
	)
}

pub(super) fn fit_extent_circle_resample_no_clip<PR, T>(
	builder: BuilderCircleResampleNoClip<Bounds<T>, PR, T>,
	extent: [[T; 2]; 2],
	object: &impl Streamable<T = T>,
) -> BuilderCircleResampleClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	let two = T::from(2.0_f64).unwrap();
	let one_five_zero = T::from(150_f64).unwrap();

	fit_circle_resample_no_clip(
		builder,
		Box::new(
			move |b: [Coordinate<T>; 2], builder: BuilderCircleResampleNoClip<Bounds<T>, PR, T>| {
				let w = extent[1][0] - extent[0][0];
				let h = extent[1][1] - extent[0][1];
				let k = T::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
				let x = extent[0][0] + (w - k * (b[1].x + b[0].x)) / two;
				let y = extent[0][1] + (h - k * (b[1].y + b[0].y)) / two;

				builder
					.scale(one_five_zero * k)
					.translate(&Coordinate { x, y })
			},
		),
		object,
	)
}

pub(super) fn fit_extent_circle_none_no_clip<PR, T>(
	builder: BuilderCircleResampleNoneNoClip<Bounds<T>, PR, T>,
	extent: [[T; 2]; 2],
	object: &impl Streamable<T = T>,
) -> BuilderCircleResampleNoneClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	let two = T::from(2.0_f64).unwrap();
	let one_five_zero = T::from(150_f64).unwrap();

	fit_circle_resample_none_no_clip(
		builder,
		Box::new(
			move |b: [Coordinate<T>; 2],
			      builder: BuilderCircleResampleNoneNoClip<Bounds<T>, PR, T>| {
				let w = extent[1][0] - extent[0][0];
				let h = extent[1][1] - extent[0][1];
				let k = T::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
				let x = extent[0][0] + (w - k * (b[1].x + b[0].x)) / two;
				let y = extent[0][1] + (h - k * (b[1].y + b[0].y)) / two;

				builder
					.scale(one_five_zero * k)
					.translate(&Coordinate { x, y })
			},
		),
		object,
	)
}

// pub(super) fn fit_size_adjust<I, LB, LC, LU, PR, PV, RC, RU, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		ClipC<Bounds<T>, T>,
// 		ClipU<Bounds<T>, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>,
// 	size: [T; 2],
// 	object: &impl Streamable<T = T>,
// ) -> Builder<Bounds<T>, I, LB, LC, LU, ClipC<Bounds<T>, T>, ClipU<Bounds<T>, T>, PR, PV, RC, RU, T>
// where
// 	I: Clone,
// 	LB: Clone,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	fit_extent_adjust(builder, [[T::zero(), T::zero()], size], object)
// }

pub(super) fn fit_size_circle_resample_no_clip<PR, T>(
	builder: BuilderCircleResampleNoClip<Bounds<T>, PR, T>,
	size: [T; 2],
	object: &impl Streamable<T = T>,
) -> BuilderCircleResampleClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	fit_extent_circle_resample_no_clip(builder, [[T::zero(), T::zero()], size], object)
}

pub(super) fn fit_width_antimerdian_resample_no_clip<PR, T>(
	builder: BuilderAntimeridianResampleNoClip<Bounds<T>, PR, T>,
	width: T,
	object: &impl Streamable<T = T>,
) -> BuilderAntimeridianResampleClip<Bounds<T>, PR, T>
where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	let two = T::from(2.0_f64).unwrap();
	let one_five_zero = T::from(150_f64).unwrap();

	fit_antimeridian_resample_no_clip(
		builder,
		Box::new(
			move |b: [Coordinate<T>; 2],
			      builder: BuilderAntimeridianResampleNoClip<Bounds<T>, PR, T>| {
				let w = width;
				let k = w / (b[1].x - b[0].x);
				let x = (w - k * (b[1].x + b[0].x)) / two;
				let y = -k * b[0].y;

				builder
					.scale(one_five_zero * k)
					.translate(&Coordinate { x, y })
			},
		),
		object,
	)
}

// pub(super) fn fit_width_adjust<I, LB, LC, LU, PR, PV, RC, RU, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		ClipC<Bounds<T>, T>,
// 		ClipU<Bounds<T>, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>,
// 	w: T,
// 	object: &impl Streamable<T = T>,
// ) -> Builder<Bounds<T>, I, LB, LC, LU, ClipC<Bounds<T>, T>, ClipU<Bounds<T>, T>, PR, PV, RC, RU, T>
// where
// 	I: Clone,
// 	LB: Clone,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	let two = T::from(2.0_f64).unwrap();
// 	let one_five_zero = T::from(150_f64).unwrap();
// 	fit_adjust(
// 		builder,
// 		Box::new(
// 			move |b: [Coordinate<T>; 2],
// 			      builder: Builder<
// 				Bounds<T>,
// 				I,
// 				LB,
// 				LC,
// 				LU,
// 				ClipC<Bounds<T>, T>,
// 				ClipU<Bounds<T>, T>,
// 				PR,
// 				PV,
// 				RC,
// 				RU,
// 				T,
// 			>| {
// 				let k = w / (b[1].x - b[0].x);
// 				let x = (w - k * (b[1].x - b[0].x)) / two;
// 				let y = -k * b[0].y;

// 				builder
// 					.scale(one_five_zero * k)
// 					.translate(&Coordinate { x, y })
// 			},
// 		),
// 		object,
// 	)
// }

// pub fn fit_width_convert<I, LB, LC, LU, PR, PV, RC, RU, RCOut, RUOut, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		ClipC<Bounds<T>, T>,
// 		ClipU<Bounds<T>, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>,
// 	w: T,
// 	object: &impl Streamable<T = T>,
// ) -> Builder<Bounds<T>, I, LB, LC, LU, ClipC<Bounds<T>, T>, ClipU<Bounds<T>, T>, PR, PV, RC, RU, T>
// where
// 	I: Clone,
// 	LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
// 	LC: Clone + LineConnected<SC = RC> + Stream<EP = Bounds<T>, T = T>,
// 	LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T> + Debug,
// 	PV: Clone + Debug,
// 	RC: Clone + Debug + Stream<EP = Bounds<T>, T = T>,
// 	RU: Clone + Debug + Connectable<Output = RC, SC = NoClipC<Bounds<T>, T>> + Debug,
// 	PR: Transform<T = T>,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	let two = T::from(2.0_f64).unwrap();
// 	let one_five_zero = T::from(150_f64).unwrap();
// 	fit_convert(
// 		builder,
// 		Box::new(
// 			move |b: [Coordinate<T>; 2],
// 			      builder: Builder<
// 				Bounds<T>,
// 				I,
// 				LB,
// 				LC,
// 				LU,
// 				ClipC<Bounds<T>, T>,
// 				ClipU<Bounds<T>, T>,
// 				PR,
// 				PV,
// 				RC,
// 				RU,
// 				T,
// 			>| {
// 				let k = w / (b[1].x - b[0].x);
// 				let x = (w - k * (b[1].x - b[0].x)) / two;
// 				let y = -k * b[0].y;

// 				builder
// 					.scale(one_five_zero * k)
// 					.translate(&Coordinate { x, y })
// 			},
// 		),
// 		object,
// 	)
// }

// pub(super) fn fit_height_adjust<I, LB, LC, LU, PR, PV, RC, RU, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		ClipC<Bounds<T>, T>,
// 		ClipU<Bounds<T>, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>,
// 	h: T,
// 	object: &impl Streamable<T = T>,
// ) -> Builder<
// 	Bounds<T>,
// 	I,
// 	LB,
// 	LC,
// 	LU,
// 	ClipC<Bounds<T>, T>,
// 	ClipU<Bounds<T>, T>,
// 	// ClipC<Bounds<T>, T>,
// 	// ClipU<Bounds<T>, T>,
// 	PR,
// 	PV,
// 	RC,
// 	RU,
// 	T,
// >
// where
// 	I: Clone,
// 	LB: Clone,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	RU: Debug,
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	let two = T::from(2.0_f64).unwrap();
// 	let one_five_zero = T::from(150_f64).unwrap();
// 	fit_adjust(
// 		builder,
// 		Box::new(
// 			move |b: [Coordinate<T>; 2],
// 			      builder: Builder<
// 				Bounds<T>,
// 				I,
// 				LB,
// 				LC,
// 				LU,
// 				ClipC<Bounds<T>, T>,
// 				ClipU<Bounds<T>, T>,
// 				// ClipC<Bounds<T>, T>,
// 				// ClipU<Bounds<T>, T>,
// 				PR,
// 				PV,
// 				RC,
// 				RU,
// 				T,
// 			>| {
// 				let k = h / (b[1].y - b[0].y);
// 				let x = -k * b[0].x;
// 				let y = (h - k * (b[1].y - b[0].y)) / two;
// 				builder
// 					.scale(one_five_zero * k)
// 					.translate(&Coordinate { x, y })
// 			},
// 		),
// 		object,
// 	)
// }

// pub(super) fn fit_height_convert<I, LB, LC, LU, PR, PV, RC, RU, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		NoClipC<Bounds<T>, T>,
// 		NoClipU<Bounds<T>, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>,
// 	h: T,
// 	object: &impl Streamable<T = T>,
// ) -> Builder<Bounds<T>, I, LB, LC, LU, ClipC<Bounds<T>, T>, ClipU<Bounds<T>, T>, PR, PV, RC, RU, T>
// where
// 	I: Clone,
// 	LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
// 	LC: Clone + LineConnected<SC = RC> + Stream<EP = Bounds<T>, T = T>,
// 	LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T> + Debug,
// 	PV: Clone + Debug,
// 	RC: Clone + Debug + Stream<EP = Bounds<T>, T = T>,
// 	RU: Clone + Debug + Connectable<Output = RC, SC = NoClipC<Bounds<T>, T>> + Debug,
// 	PR: Transform<T = T>,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	let two = T::from(2.0_f64).unwrap();
// 	let one_five_zero = T::from(150_f64).unwrap();
// 	fit_convert(
// 		builder,
// 		Box::new(
// 			move |b: [Coordinate<T>; 2],
// 			      builder: Builder<
// 				Bounds<T>,
// 				I,
// 				LB,
// 				LC,
// 				LU,
// 				NoClipC<Bounds<T>, T>,
// 				NoClipU<Bounds<T>, T>,
// 				PR,
// 				PV,
// 				RC,
// 				RU,
// 				T,
// 			>| {
// 				let k = h / (b[1].y - b[0].y);
// 				let x = -k * b[0].x;
// 				let y = (h - k * (b[1].y - b[0].y)) / two;
// 				builder
// 					.scale(one_five_zero * k)
// 					.translate(&Coordinate { x, y })
// 			},
// 		),
// 		object,
// 	)
// }
