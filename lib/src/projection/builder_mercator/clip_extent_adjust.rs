use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::ClipExtentAdjust;
use crate::Transform;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentAdjust
	for Builder<DRAIN, I, LB, LC, LU, ClipC<DRAIN, T>, ClipU<DRAIN, T>, PR, PV, RC, RU, T>
where
	PR: Clone + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn clip_extent_adjust(self, extent: &[Coordinate<T>; 2]) -> Self {
		let base = self.base.clip_extent_adjust(extent);

		let out = Self {
			base,
			pr: self.pr,
			// Mutate stage
			extent: Some(*extent),
		};
		// .reset();

		// out.reset()
		out
	}
}
