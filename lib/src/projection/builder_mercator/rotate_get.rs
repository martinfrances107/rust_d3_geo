use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::RotateGet;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T> RotateGet
	for Builder<DRAIN, I, LB, LC, LU, PCNU, PR, PV, RC, RU, T>
where
	T: CoordFloat + FloatConst,
{
	type T = T;

	#[inline]
	fn rotate(&self) -> [T; 3] {
		self.base.rotate()
	}
}
