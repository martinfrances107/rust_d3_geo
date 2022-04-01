use std::fmt::Debug;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::RotateGet;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> RotateGet
	for Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
	PCNU: Debug,
	RU: Debug,
	T: CoordFloat + FloatConst,
{
	type T = T;

	#[inline]
	fn get_rotate(&self) -> [T; 3] {
		[
			self.delta_lambda.to_degrees(),
			self.delta_phi.to_degrees(),
			self.delta_lambda.to_degrees(),
		]
	}
}
