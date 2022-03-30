use geo::CoordFloat;
use geo::Coordinate;

use crate::projection::TranslateGet;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> TranslateGet
	for Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
	T: CoordFloat,
{
	type T = T;

	#[inline]
	fn get_translate(&self) -> Coordinate<T> {
		self.base.get_translate()
	}
}
