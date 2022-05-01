// use geo::Coordinate;

// use super::builder::template::ClipC;
// use super::builder::template::ClipU;
// use super::builder::template::NoClipC;
// use super::builder::template::NoClipU;

// use super::builder::Builder;

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

// fn fit_with_resampling_and_pcn<I, LB, LC, LU, PR, PV, T>(
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
// 		ResampleClipC<Bounds<T>, PR, T>,
// 		ResampleClipU<Bounds<T>, PR, T>,
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
// 		ResampleClipC<Bounds<T>, PR, T>,
// 		ResampleClipU<Bounds<T>, PR, T>,
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
// 	ResampleClipC<Bounds<T>, PR, T>,
// 	ResampleClipU<Bounds<T>, PR, T>,
// 	T,
// >
// where
// 	I: Clone + Interpolator<T = T>,
// 	LB: Clone + Debug + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
// 	LC: Clone + LineConnected<SC = ResampleClipC<Bounds<T>, PR, T>> + Stream<EP = Bounds<T>, T = T>,
// 	LU: Connectable<Output = LC, SC = ResampleClipC<Bounds<T>, PR, T>>
// 		+ Bufferable<Output = LB, T = T>
// 		+ Clone
// 		+ Debug,
// 	PR: Clone + Debug + Transform<T = T>,
// 	PV: Clone + PointVisible<T = T>,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	match builder.get_clip_extent() {
// 		Some(clip) => {
// 			let builder = builder
// 				.scale(T::from(150.0_f64).unwrap())
// 				.translate(&Coordinate {
// 					x: T::zero(),
// 					y: T::zero(),
// 				});
// 			// .clip_extent_clear();
// 			let bounds_stream = Bounds::default();
// 			let mut stream_in = builder.build().stream(bounds_stream);

// 			object.to_stream(&mut stream_in);
// 			let bounds = stream_in.get_endpoint().result();
// 			let builder = fit_bounds(bounds, builder);
// 			// let builder = builder.clip_extent_adjust(&clip);
// 		}
// 		None => {}
// 	};

// 	builder
// }

// fn fit_convert<I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, RCOut, RUOut, T>(
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
// 	fit_bounds: FitBoundsConvert<Bounds<T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, RC, RU, T>,
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
// 	RC,
// 	RU,
// 	T,
// >
// where
// 	// I: Clone,
// 	// LB: Clone + Debug,
// 	// LC: Clone + Debug,
// 	// LU: Clone + Debug,
// 	RU: Debug,
// 	PR: Transform<T = T>,
// 	// PV: Clone + Debug,
// 	// PCNU: Clone + Debug,
// 	PCNU: Debug,
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
// 		ClipC<Bounds<T>, T>,
// 		ClipU<Bounds<T>, T>,
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
// 	ClipC<Bounds<T>, T>,
// 	ClipU<Bounds<T>, T>,
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
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
// 	RU: Debug,
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
// 				ClipC<Bounds<T>, T>,
// 				ClipU<Bounds<T>, T>,
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

// pub(super) fn fit_extent<I, LB, LC, LU, PR, PV, RC, RU, RCOut, RUOut, T>(
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
// 	extent: [[T; 2]; 2],
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
// 	RC,
// 	RU,
// 	T,
// >
// where
// 	I: Clone,
// 	LB: Clone + Debug,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PV: Clone + Debug,
// 	RC: Debug,
// 	RU: Debug,
// 	PR: Transform<T = T>,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	let two = T::from(2.0_f64).unwrap();
// 	let one_five_zero = T::from(150_f64).unwrap();
// 	fit_extent(
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
// 	RC,
// 	RU,
// 	T,
// >
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

// pub(super) fn fit_size_convert<I, LB, LC, LU, PR, PV, RC, RU, T>(
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
// 	size: [T; 2],
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
// 	RC,
// 	RU,
// 	T,
// >
// where
// 	I: Clone,
// 	LB: Clone + Debug,
// 	LC: Clone + Debug,
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
// 	LU: Clone + Debug,
// 	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	fit_extent(builder, [[T::zero(), T::zero()], size], object)
// }

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
// 	RC,
// 	RU,
// 	T,
// >
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
// 		NoClipC<Bounds<T>, T>,
// 		NoClipU<Bounds<T>, T>,
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
// 	ClipC<Bounds<T>, T>,
// 	ClipU<Bounds<T>, T>,
// 	PR,
// 	PV,
// 	RC,
// 	RU,
// 	T,
// >
// where
// 	// I: Clone,
// 	// LB: Clone + Debug,
// 	// LC: Clone + Debug,
// 	// LU: Clone + Debug,
// 	RU: Debug,
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
// 				NoClipC<Bounds<T>, T>,
// 				NoClipU<Bounds<T>, T>,
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
// 	RC,
// 	RU,
// 	T,
// >
// where
// 	I: Clone,
// 	LB: Clone + Debug,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PR: Transform<T = T>,
// 	PV: Clone + Debug,
// 	RU: Debug,
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
