use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder_mercator::Builder;
use crate::projection::builder_mercator::ResampleNoClipC;
use crate::projection::builder_mercator::ResampleNoClipU;
use crate::projection::AngleGet;
use crate::projection::AngleSet;
use crate::Transform;

impl<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T> AngleGet
	for Builder<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T>
where
	T: CoordFloat + FloatConst,
{
	type T = T;

	#[inline]
	fn get_angle(&self) -> T {
		self.base.get_angle()
	}
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> AngleSet
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
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type T = T;

	/// Sets the rotation angles as measured in degrees.
	#[inline]
	fn angle(self, angle: T) -> Self {
		Self {
			extent: self.extent, // post-clip extent
			pr: self.pr,
			base: self.base.angle(angle),
		}
	}
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> AngleSet
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
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type T = T;

	/// Sets the rotation angles as measured in degrees.
	#[inline]
	fn angle(self, angle: T) -> Self {
		Self {
			extent: self.extent,
			pr: self.pr,
			base: self.base.angle(angle),
		}
	}
}
