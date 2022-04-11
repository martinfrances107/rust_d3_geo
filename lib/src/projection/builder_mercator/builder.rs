// impl<I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> Fit
// 	for MercatorBuilder<Bounds<T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
// 	I: Clone + Debug,
// 	LB: Clone + Debug,
// 	LC: Clone + Debug,
// 	LU: Clone + Debug,
// 	PCNC: Clone + Debug,
// 	PCNU: Clone + Debug,
// 	PR: TransformExtent<T>,
// 	PV: PointVisible<T = T>,
// 	RC: Clone + Debug,
// 	RU: Clone + Debug,
// 	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
// {
// 	type T = T;

// 	#[inline]
// 	fn fit_extent(mut self, extent: [[T; 2]; 2], object: &impl Streamable<T = Self::T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		self.base = self.base.fit_extent(extent, object);
// 		self
// 	}

// 	#[inline]
// 	fn fit_size(mut self, size: [T; 2], object: &impl Streamable<T = T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		self.base = self.base.fit_size(size, object);
// 		self
// 	}

// 	#[inline]
// 	fn fit_width(mut self, w: T, object: &impl Streamable<T = T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		self.base = self.base.fit_width(w, object);
// 		self
// 	}

// 	/// Similar to fit_size where the width is automatically chosen from
// 	/// the aspect ratio of object and the given constraint on height.
// 	#[inline]
// 	fn fit_height(mut self, h: T, object: &impl Streamable<T = T>) -> Self
// 	where
// 		Self::T: AsPrimitive<T> + CoordFloat,
// 	{
// 		self.base = self.base.fit_height(h, object);
// 		self
// 	}
// }

// impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> Reflect
// 	for MercatorBuilder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
// where
// 	DRAIN: Default + Stream<EP = DRAIN, T = T>,
// 	PV: PointVisible<T = T>,

// 	T: 'static
// 		+ AbsDiffEq<Epsilon = T>
// 		+ std::ops::AddAssign
// 		+ AsPrimitive<T>
// 		+ CoordFloat
// 		// + Display
// 		+ FloatConst,
// {
// 	type T = T;

// 	/// Is the projection builder set to invert the x-coordinate.
// 	#[inline]
// 	fn get_reflect_x(&self) -> bool {
// 		self.base.get_reflect_x()
// 	}

// 	/// Set the projection builder to invert the x-coordinate.
// 	fn reflect_x(mut self, reflect: bool) -> Self {
// 		self.base = self.base.reflect_x(reflect);
// 		self
// 	}

// 	/// Is the projection builder set to invert the y-coordinate.
// 	#[inline]
// 	fn get_reflect_y(&self) -> bool {
// 		self.base.get_reflect_y()
// 	}

// 	/// Set the projection builder to invert the y-coordinate.
// 	#[inline]
// 	fn reflect_y(mut self, reflect: bool) -> Self {
// 		self.base = self.base.reflect_y(reflect);
// 		self
// 	}
// }
