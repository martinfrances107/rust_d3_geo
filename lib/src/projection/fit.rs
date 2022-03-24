// use std::fmt::Debug;

// use crate::Transform;
// use approx::AbsDiffEq;
// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::Float;
// use num_traits::FloatConst;

// use crate::clip::rectangle::Rectangle;
// // use crate::clip::Interpolate;
// use crate::identity::Identity;
// use crate::path::bounds::Bounds;
// // use crate::stream::Connectable;
// use crate::stream::Connected;
// // use crate::stream::Stream;
// use crate::stream::Streamable;
// use crate::stream::Unconnected;

// use super::builder::template::ClipC;
// use super::builder::template::ClipU;
// use super::builder::template::NoClipC;
// use super::builder::template::NoClipU;
// use super::builder::template::ResampleClipC;
// use super::builder::template::ResampleClipU;
// use super::builder::Builder;
// // use super::builder::PostClipNode;
// use super::resampler::resample::Connected as ConnectedResample;
// use super::resampler::resample::Resample;
// use super::resampler::Resampler;
// use super::ClipExtentBounded;
// use super::ClipExtentSet;
// // use super::PointVisible;
// use super::ProjectionRawBase;

// type FitBoundsAdjust<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> = Box<
// 	dyn Fn(
// 		[Coordinate<T>; 2],
// 		Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>,
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

// fn fit_with_resampling_ce<I, LB, LC, LU, PR, PV, T>(
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
// 		ResampleClipU<Bounds<T>, PR, T>,
// 		ResampleClipC<Bounds<T>, PR, T>,
// 		T,
// 	>,
// 	fit_bounds: FitBoundsAdjust<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		ClipC<Bounds<T>, T>,
// 		ClipU<Bounds<T>, T>,
// 		PR,
// 		PV,
// 		ResampleClipU<Bounds<T>, PR, T>,
// 		ResampleClipC<Bounds<T>, PR, T>,
// 		T,
// 	>,
// 	object: &impl Streamable<T = T>,
// ) -> Builder<
// 	Bounds<T>,
// 	I,
// 	LB,
// 	LC,
// 	LU,
// 	ClipC<Bounds<T>, T>,
// 	ClipU<Bounds<T>, T>,
// 	PR,
// 	PV,
// 	ResampleClipU<Bounds<T>, PR, T>,
// 	ResampleClipC<Bounds<T>, PR, T>,
// 	T,
// >
// where
// 	// I: Interpolate<T = T>,
// 	// PR: ProjectionRawBase<T>,
// 	// PV: PointVisible<T = T>,
// 	// PCNU: PostClipNode + Connectable<Output = PCNC, SC = Bounds<T>>,
// 	// PCNC: PostClipNode + Stream<EP = Bounds<T>, T = T>,
// 	I: Clone,
// 	LB: Clone,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	let clip = builder.get_clip_extent();
// 	let builder1 = builder
// 		.scale(T::from(150.0_f64).unwrap())
// 		.translate(&Coordinate {
// 			x: T::zero(),
// 			y: T::zero(),
// 		});

// 	let builder2 = match clip {
// 		Some(_) => builder1.clip_extent_clear(),
// 		None => builder1,
// 	};

// 	let bounds_stream = Bounds::default();
// 	let mut stream_in = builder2.build().stream(bounds_stream);

// 	object.to_stream(&mut stream_in);
// 	let bounds = stream_in.get_endpoint().result();
// 	let builder3 = fit_bounds(bounds, builder2);
// 	match clip {
// 		Some(extent) => builder3.clip_extent(&extent),
// 		None => builder3,
// 	}
// }

// fn fit_convert<I, LB, LC, LU, PCNC, PCNU, PR, PV, RCIn, RUIn, RCOut, RUOut, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		Identity<Bounds<T>, Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 		Identity<Bounds<T>, Bounds<T>, Bounds<T>, Unconnected, T>,
// 		PR,
// 		PV,
// 		RCIn,
// 		RUIn,
// 		T,
// 	>,
// 	fit_bounds: FitBoundsConvert<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		PCNC,
// 		PCNU,
// 		PR,
// 		PV,
// 		RCIn,
// 		RUIn,
// 		RCOut,
// 		RUOut,
// 		T,
// 	>,
// 	object: &impl Streamable<T = T>,
// ) -> Builder<
// 	Bounds<T>,
// 	I,
// 	LB,
// 	LC,
// 	LU,
// 	Rectangle<Bounds<T>, Bounds<T>, ConnectedResample<Bounds<T>, T>, T>,
// 	Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 	PR,
// 	PV,
// 	RCOut,
// 	RUOut,
// 	T,
// >
// where
// 	// I: Interpolate<T = T>,
// 	// PR: ProjectionRawBase<T>,
// 	// PV: PointVisible<T = T>,
// 	// RCOut: Resampler + Stream<EP = Bounds<T>>,
// 	// RUOut: Resampler
// 	// 	+ Connectable<Output = RCOut, SC = Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>>,
// 	// PCNC: PostClipNode,
// 	// PCNU: PostClipNode + Connectable<Output = PCNC, SC = Bounds<T>>,
// 	I: Clone,
// 	LB: Clone + Debug,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
// 	PCNU: Clone + Debug,
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

// pub(super) fn fit_extent_adjust<I, LB, LC, LU, PR, PV, RC, RU, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 		Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>,
// 	extent: [[T; 2]; 2],
// 	object: &impl Streamable<T = T>,
// ) -> Builder<
// 	Bounds<T>,
// 	I,
// 	LB,
// 	LC,
// 	LU,
// 	Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 	Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 	PR,
// 	PV,
// 	RC,
// 	RU,
// 	T,
// >
// where
// 	// I: Interpolate<T = T>,
// 	// PR: ProjectionRawBase<T>,
// 	// PV: PointVisible<T = T>,
// 	// RC: Resampler + Stream<EP = Bounds<T>, T = T>,
// 	// RU: Resampler
// 	// 	+ Connectable<Output = RC, SC = Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>>,
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
// 	fit_extent_adjust(
// 		builder,
// 		Box::new(
// 			move |b: [Coordinate<T>; 2],
// 			      builder: Builder<
// 				Bounds<T>,
// 				I,
// 				LB,
// 				LC,
// 				LU,
// 				Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 				Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 				PR,
// 				PV,
// 				RC,
// 				RU,
// 				T,
// 			>| {
// 				let w = extent[1][0] - extent[0][0];
// 				let h = extent[1][1] - extent[0][1];
// 				let k = Float::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
// 				let x = extent[0][0] + (w - k * (b[1].x + b[0].x)) / two;
// 				let y = extent[0][1] + (h - k * (b[1].y + b[0].y)) / two;

// 				builder
// 					.scale(one_five_zero * k)
// 					.translate(&Coordinate { x, y })
// 			},
// 		),
// 		object,
// 	)
// }

// pub(super) fn fit_extent_convert<I, LB, LC, LU, PR, PV, RCIn, RUIn, RCOut, RUOut, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		Identity<Bounds<T>, Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 		Identity<Bounds<T>, Bounds<T>, Bounds<T>, Unconnected, T>,
// 		PR,
// 		PV,
// 		RCIn,
// 		RUIn,
// 		T,
// 	>,
// 	extent: [[T; 2]; 2],
// 	object: &impl Streamable<T = T>,
// ) -> Builder<
// 	Bounds<T>,
// 	I,
// 	LB,
// 	LC,
// 	LU,
// 	Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 	Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 	PR,
// 	PV,
// 	RCOut,
// 	RUOut,
// 	T,
// >
// where
// 	// I: Interpolate<T = T>,
// 	// PR: ProjectionRawBase<T>,
// 	// PV: PointVisible<T = T>,
// 	// RCOut: Resampler + Stream<EP = Bounds<T>, T = T>,
// 	// RUOut: Resampler
// 	// 	+ Connectable<Output = RCOut, SC = Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>>,
// 	I: Clone,
// 	LB: Clone + Debug,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PV: Clone + Debug,
// 	PR: Transform<T = T>,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	let two = T::from(2.0_f64).unwrap();
// 	let one_five_zero = T::from(150_f64).unwrap();
// 	fit_extent_convert(
// 		builder,
// 		Box::new(
// 			move |b: [Coordinate<T>; 2],
// 			      builder: Builder<
// 				Bounds<T>,
// 				I,
// 				LB,
// 				LC,
// 				LU,
// 				Identity<Bounds<T>, Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 				Identity<Bounds<T>, Bounds<T>, Bounds<T>, Unconnected, T>,
// 				PR,
// 				PV,
// 				RCIn,
// 				RUIn,
// 				T,
// 			>| {
// 				let w = extent[1][0] - extent[0][0];
// 				let h = extent[1][1] - extent[0][1];
// 				let k = Float::min(w / (b[1].x - b[0].x), h / (b[1].y - b[0].y));
// 				let x = extent[0][0] + (w - k * (b[1].x + b[0].x)) / two;
// 				let y = extent[0][1] + (h - k * (b[1].y + b[0].y)) / two;

// 				builder
// 					.scale(one_five_zero * k)
// 					.translate(&Coordinate { x, y })
// 			},
// 		),
// 		object,
// 	)
// }

// pub(super) fn fit_size_adjust<I, LB, LC, LU, PR, PV, RC, RU, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 		Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>,
// 	size: [T; 2],
// 	object: &impl Streamable<T = T>,
// ) -> Builder<
// 	Bounds<T>,
// 	I,
// 	LB,
// 	LC,
// 	LU,
// 	Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 	Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 	PR,
// 	PV,
// 	RC,
// 	RU,
// 	T,
// >
// where
// 	// I: Interpolate<T = T>,
// 	// PR: ProjectionRawBase<T>,
// 	// PV: PointVisible<T = T>,
// 	// RC: Resampler + Stream<EP = Bounds<T>, T = T>,
// 	// RU: Resampler
// 	// 	+ Connectable<Output = RC, SC = Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>>,
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

// pub(super) fn fit_size_convert<I, LB, LC, LU, PR, PV, RC, RU, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		Identity<Bounds<T>, Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 		Identity<Bounds<T>, Bounds<T>, Bounds<T>, Unconnected, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>,
// 	size: [T; 2],
// 	object: &impl Streamable<T = T>,
// ) -> Builder<
// 	Bounds<T>,
// 	I,
// 	LB,
// 	LC,
// 	LU,
// 	Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 	Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 	PR,
// 	PV,
// 	RC,
// 	RU,
// 	T,
// >
// where
// 	// I: Interpolate<T = T>,
// 	// PR: ProjectionRawBase<T>,
// 	// PV: PointVisible<T = T>,
// 	// RC: Resampler + Stream<EP = Bounds<T>, T = T>,
// 	// RU: Resampler
// 	// 	+ Connectable<Output = RC, SC = Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>>,
// 	I: Clone,
// 	LB: Clone + Debug,
// 	LC: Clone + Debug,
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
// 	LU: Clone + Debug,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	fit_extent_convert(builder, [[T::zero(), T::zero()], size], object)
// }

// pub(super) fn fit_width_adjust<I, LB, LC, LU, PR, PV, RC, RU, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 		Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 		PR,
// 		PV,
// 		RC,
// 		RU,
// 		T,
// 	>,
// 	w: T,
// 	object: &impl Streamable<T = T>,
// ) -> Builder<
// 	Bounds<T>,
// 	I,
// 	LB,
// 	LC,
// 	LU,
// 	Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 	Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 	PR,
// 	PV,
// 	RC,
// 	RU,
// 	T,
// >
// where
// 	// I: Interpolate<T = T>,
// 	// PR: ProjectionRawBase<T>,
// 	// PV: PointVisible<T = T>,
// 	// RC: Resampler + Stream<EP = Bounds<T>, T = T>,
// 	// RU: Resampler
// 	// 	+ Connectable<Output = RC, SC = Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>>,
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
// 				Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 				Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
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

// pub(super) fn fit_width_convert<I, LB, LC, LU, PR, PV, RCIn, RUIn, RCOut, RUOut, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		Identity<Bounds<T>, Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 		Identity<Bounds<T>, Bounds<T>, Bounds<T>, Unconnected, T>,
// 		PR,
// 		PV,
// 		RCIn,
// 		RUIn,
// 		T,
// 	>,
// 	w: T,
// 	object: &impl Streamable<T = T>,
// ) -> Builder<
// 	Bounds<T>,
// 	I,
// 	LB,
// 	LC,
// 	LU,
// 	Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 	Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 	PR,
// 	PV,
// 	RCOut,
// 	RUOut,
// 	T,
// >
// where
// 	// I: Interpolate<T = T>,
// 	// PR: ProjectionRawBase<T>,
// 	// PV: PointVisible<T = T>,
// 	// RCOut: Resampler + Stream<EP = Bounds<T>, T = T>,
// 	// RUOut: Resampler
// 	// 	+ Connectable<Outut = RCOut, SC = Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>>,
// 	I: Clone,
// 	LB: Clone + Debug,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
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
// 				Identity<Bounds<T>, Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 				Identity<Bounds<T>, Bounds<T>, Bounds<T>, Unconnected, T>,
// 				PR,
// 				PV,
// 				RCIn,
// 				RUIn,
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
// 		Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 		Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
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
// 	Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 	Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 	PR,
// 	PV,
// 	RC,
// 	RU,
// 	T,
// >
// where
// 	// I: Interpolate<T = T>,
// 	// PR: ProjectionRawBase<T>,
// 	// PV: PointVisible<T = T>,
// 	// RC: Resampler + Stream<EP = Bounds<T>, T = T>,
// 	// RU: Resampler
// 	// 	+ Connectable<Output = RC, SC = Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>>,
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
// 				Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 				Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
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

// pub(super) fn fit_height_convert<I, LB, LC, LU, PR, PV, RCIn, RUIn, RCOut, RUOut, T>(
// 	builder: Builder<
// 		Bounds<T>,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		Identity<Bounds<T>, Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 		Identity<Bounds<T>, Bounds<T>, Bounds<T>, Unconnected, T>,
// 		PR,
// 		PV,
// 		RCIn,
// 		RUIn,
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
// 	Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 	Rectangle<Bounds<T>, Bounds<T>, Unconnected, T>,
// 	PR,
// 	PV,
// 	RCOut,
// 	RUOut,
// 	T,
// >
// where
// 	// I: Interpolate<T = T>,
// 	// PR: ProjectionRawBase<T>,
// 	// PV: PointVisible<T = T>,
// 	// RCIn: Resampler + Stream<EP = Bounds<T>, T = T>,
// 	// RUIn: Resampler
// 	// 	+ Connectable<
// 	// 		Output = RCIn,
// 	// 		SC = Identity<Bounds<T>, Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 	// 	>,
// 	// RCOut: Resampler + Stream<EP = Bounds<T>, T = T>,
// 	// RUOut: Resampler
// 	// 	+ Connectable<Output = RCOut, SC = Rectangle<Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>>,
// 	I: Clone,
// 	LB: Clone + Debug,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
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
// 				Identity<Bounds<T>, Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
// 				Identity<Bounds<T>, Bounds<T>, Bounds<T>, Unconnected, T>,
// 				PR,
// 				PV,
// 				RCIn,
// 				RUIn,
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
