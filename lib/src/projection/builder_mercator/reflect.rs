use crate::Transform;
use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::PointVisible;
use crate::projection::builder_mercator::builder::Builder;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedRsample;
use crate::projection::resampler::resample::Resample;
use crate::projection::ProjectionRawBase;
use crate::projection::Reflect;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;

impl<DRAIN, INTERPOLATE, LB, LC, LU, PCNC, PCNU, PR, PV, T> Reflect
	for Builder<
		DRAIN,
		INTERPOLATE,
		LB,
		LC,
		LU,
		PCNC,
		PCNU,
		PR,
		PV,
		Resample<DRAIN, PR, PCNC, PCNU, ConnectedRsample<PCNC, T>, T>,
		Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
		T,
	> where
	// INTERPOLATE: Interpolate<T = T>,
	// DRAIN: Stream<EP = DRAIN, T = T> + Default,
	// PR: ProjectionRawBase<T>,
	PR: Clone + Transform<T = T>,
	// PV: PointVisible<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	/// Is the projection builder set to invert the x-coordinate.
	#[inline]
	fn get_reflect_x(&self) -> bool {
		self.base.sx < T::zero()
	}

	/// Set the projection builder to invert the x-coordinate.
	fn reflect_x(mut self, reflect: bool) -> Self {
		if reflect {
			self.base.sx = T::from(-1.0_f64).unwrap();
		} else {
			self.base.sx = T::one();
		}
		let base = self.base.recenter_with_resampling();

		Self {
			pr: self.pr,
			base,
			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1, // post-clip extent
		}
	}

	/// Is the projection builder set to invert the y-coordinate.
	#[inline]
	fn get_reflect_y(&self) -> bool {
		self.base.sy < T::zero()
	}

	/// Set the projection builder to invert the y-coordinate.
	#[inline]
	fn reflect_y(mut self, reflect: bool) -> Self {
		if reflect {
			self.base.sy = T::from(-1.0_f64).unwrap();
		} else {
			self.base.sy = T::one();
		}
		let base = self.base.recenter_with_resampling();
		Self {
			pr: self.pr,
			base,
			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1, // post-clip extent
		}
	}
}

impl<DRAIN, INTERPOLATE, LB, LC, LU, PCNC, PCNU, PR, PV, T> Reflect
	for Builder<
		DRAIN,
		INTERPOLATE,
		LB,
		LC,
		LU,
		PCNC,
		PCNU,
		PR,
		PV,
		ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
		ResampleNone<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
		T,
	> where
	// INTERPOLATE: Interpolate<T = T>,
	// DRAIN: Stream<EP = DRAIN, T = T> + Default,
	// PR: ProjectionRawBase<T>,
	PR: Clone + Transform<T = T>,
	// PV: PointVisible<T = T>,
	T: 'static + AsPrimitive<T> + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	/// Is the projection builder set to invert the x-coordinate.
	#[inline]
	fn get_reflect_x(&self) -> bool {
		self.base.sx < T::zero()
	}

	/// Set the projection builder to invert the x-coordinate.
	fn reflect_x(mut self, reflect: bool) -> Self {
		if reflect {
			self.base.sx = T::from(-1.0_f64).unwrap();
		} else {
			self.base.sx = T::one();
		}
		let base = self.base.recenter_no_resampling();
		Self {
			pr: self.pr,
			base,
			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1, // post-clip extent
		}
	}

	/// Is the projection builder set to invert the y-coordinate.
	#[inline]
	fn get_reflect_y(&self) -> bool {
		self.base.sy < T::zero()
	}

	/// Set the projection builder to invert the y-coordinate.
	#[inline]
	fn reflect_y(mut self, reflect: bool) -> Self {
		if reflect {
			self.base.sy = T::from(-1.0_f64).unwrap();
		} else {
			self.base.sy = T::one();
		}
		let base = self.base.recenter_no_resampling();
		Self {
			pr: self.pr,
			base,
			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1, // post-clip extent
		}
	}
}
