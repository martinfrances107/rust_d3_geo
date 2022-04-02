use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::ReflectGet;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> ReflectGet
	for Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
	T: CoordFloat + FloatConst,
{
	type T = T;

	/// Is the projection builder set to invert the x-coordinate.
	#[inline]
	fn get_reflect_x(&self) -> bool {
		self.sx < T::zero()
	}

	/// Is the projection builder set to invert the y-coordinate.
	#[inline]
	fn get_reflect_y(&self) -> bool {
		self.sy < T::zero()
	}
}
