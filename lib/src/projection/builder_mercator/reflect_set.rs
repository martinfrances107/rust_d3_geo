use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoClipC;
use crate::projection::builder::template::ResampleNoClipU;
use crate::projection::RecenterWithResampling;
use crate::projection::ReflectSet;
use crate::Transform;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> ReflectSet
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		NoClipU<DRAIN>,
		PR,
		PV,
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	PR: Clone + Transform<T = T>,
	Self: RecenterWithResampling,
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

impl<DRAIN, I, LB, LC, LU, PR, PV, T> ReflectSet
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
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
