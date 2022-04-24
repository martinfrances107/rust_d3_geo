use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::rectangle::Rectangle;
use crate::projection::ClipExtentAdjust;
use crate::stream::Stream;
use crate::Transform;

use super::template::ClipC;
use super::template::ClipU;
use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentAdjust
	for Builder<DRAIN, I, LB, LC, LU, ClipC<DRAIN, T>, ClipU<DRAIN, T>, PR, PV, RC, RU, T>
where
	DRAIN: Default + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn clip_extent_adjust(mut self, extent: &[Coordinate<T>; 2]) -> Self {
		self.x0 = Some(extent[0].x);
		self.y0 = Some(extent[0].y);
		self.x1 = Some(extent[1].x);
		self.y1 = Some(extent[1].y);
		self.postclip = Rectangle::new(extent[0].x, extent[0].y, extent[1].x, extent[1].y);
		// out.reset()
		self
	}
}
