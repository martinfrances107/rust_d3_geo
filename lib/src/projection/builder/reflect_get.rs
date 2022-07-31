use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::ReflectGet;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T> ReflectGet
	for Builder<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T>
where
	T: CoordFloat + FloatConst,
{
	type T = T;

	/// Is the projection builder set to invert the x-coordinate.
	#[inline]
	fn is_x_reflected(&self) -> bool {
		self.sx < T::zero()
	}

	/// Is the projection builder set to invert the y-coordinate.
	#[inline]
	fn is_y_reflected(&self) -> bool {
		self.sy < T::zero()
	}
}
