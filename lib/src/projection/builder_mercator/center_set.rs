use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::builder_mercator::ReclipAdjust;
use crate::projection::CenterSet;
use crate::projection::TransformExtent;
use crate::stream::Stream;
use crate::Transform;

impl<DRAIN, PR, T> CenterSet for BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>
where
	DRAIN: Clone + Stream<EP = DRAIN, T = T>,
	PR: Clone + Debug + Transform<T = T> + TransformExtent<T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + Debug + FloatConst,
{
	type T = T;

	fn center(mut self, center: &Coordinate<T>) -> Self {
		self.base = self.base.center(center);
		self.reclip_adjust()
	}
}
