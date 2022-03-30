use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::projection::builder_mercator::Builder;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedRsample;
use crate::projection::resampler::resample::Resample;
use crate::projection::ReflectSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

impl<DRAIN, INTERPOLATE, LB, LC, LU, PCNC, PCNU, PR, PV, T> ReflectSet
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
	PR: Clone + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

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

impl<DRAIN, INTERPOLATE, LB, LC, LU, PCNC, PCNU, PR, PV, T> ReflectSet
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
	PR: Clone + Transform<T = T>,
	T: 'static + AsPrimitive<T> + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

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
