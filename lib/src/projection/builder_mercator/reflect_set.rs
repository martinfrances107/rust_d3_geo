use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoClipC;
use crate::projection::builder::template::ResampleNoClipU;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::ReflectSet;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<DRAIN, INTERPOLATE, LB, LC, LU, PR, PV, T> ReflectSet
	for Builder<
		DRAIN,
		INTERPOLATE,
		LB,
		LC,
		LU,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PV,
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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
			extent: self.extent,
			pr: self.pr,
			base,
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
			extent: self.extent,
			pr: self.pr,
			base,
		}
	}
}

impl<DRAIN, INTERPOLATE, LB, LC, LU, PR, PV, T> ReflectSet
	for Builder<
		DRAIN,
		INTERPOLATE,
		LB,
		LC,
		LU,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PV,
		ResampleClipC<DRAIN, PR, T>,
		ResampleClipU<DRAIN, PR, T>,
		T,
	> where
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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
			extent: self.extent,
			pr: self.pr,
			base,
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
			extent: self.extent,
			pr: self.pr,
			base,
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
	T: AsPrimitive<T> + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
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
			extent: self.extent,
			pr: self.pr,
			base,
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
			extent: self.extent,
			pr: self.pr,
			base,
		}
	}
}
