use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::Scale;
use crate::projection::TransformExtent;
use crate::stream::Stream;
use crate::Transform;

use super::ReclipAdjust;

impl<DRAIN, PR, T> Scale for BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>
where
	DRAIN: Clone + Debug + Default + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T> + TransformExtent<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn scale(mut self, scale: T) -> Self {
		self.base = self.base.scale(scale);
		self.reclip_adjust()
	}
}
