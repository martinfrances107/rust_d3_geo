use crate::stream::Stream;
use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::ClipExtentAdjust;
use crate::Transform;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> ClipExtentAdjust
	for Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
	DRAIN: 'static + Default + Stream<EP = DRAIN, T = T>,
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
			x0: Some(extent[0].x),
			y0: Some(extent[0].y),
			x1: Some(extent[1].x),
			y1: Some(extent[1].y),
		};
		// .reset();

		// out.reset()
		out
	}
}
