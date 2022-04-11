use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::ClipExtentAdjust;
use crate::stream::Stream;
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

	fn clip_extent_adjust(mut self, extent: &[Coordinate<T>; 2]) -> Self {
		self.x0 = Some(extent[0].x);
		self.y0 = Some(extent[0].y);
		self.x1 = Some(extent[1].x);
		self.y1 = Some(extent[1].y);
		// out.reset()
		self
	}
}

// impl<DRAIN, I, LB, LC, LU, PR, T> ClipExtentAdjust
// 	for Builder<
// 		DRAIN,
// 		I,
// 		LB,
// 		LC,
// 		LU,
// 		NoClipC<DRAIN, T>,
// 		NoClipU<DRAIN, T>,
// 		PR,
// 		PVAntimeridian<T>,
// 		ResampleNoClipC<DRAIN, PR, T>,
// 		ResampleNoClipU<DRAIN, PR, T>,
// 		T,
// 	> where
// 	DRAIN: 'static + Default + Stream<EP = DRAIN, T = T>,
// 	PR: Clone + Transform<T = T>,
// 	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
// {
// 	type T = T;

// 	fn clip_extent_adjust(self, extent: &[Coordinate<T>; 2]) -> Self {
// 		self.x0 = Some(extent[0].x);
// 		self.y0 = Some(extent[0].y);
// 		self.x1 = Some(extent[1].x);
// 		self.y1 = Some(extent[1].y);
// 		// out.reset()
// 		self
// 	}
// }
