use geo::CoordFloat;

use num_traits::FloatConst;

use crate::projection::builder::Builder;
use crate::projection::AngleGet;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> AngleGet
	for Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
	T: CoordFloat + FloatConst,
{
	/// f64 or f32.
	type T = T;

	/// Returns the projection’s post-projection planar rotation angle.
	/// defaults to 0°.
	#[inline]
	fn get_angle(&self) -> Self::T {
		self.alpha.to_degrees()
	}
}
