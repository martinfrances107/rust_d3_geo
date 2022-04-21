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
