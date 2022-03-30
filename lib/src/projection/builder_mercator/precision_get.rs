use geo::CoordFloat;

use crate::projection::builder_mercator::builder::Builder;
use crate::projection::PrecisionGet;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> PrecisionGet
	for Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
	T: CoordFloat,
{
	type T = T;
	#[inline]
	fn get_precision(&self) -> T {
		self.base.get_precision()
	}
}
