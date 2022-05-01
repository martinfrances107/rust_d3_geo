use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleNoneClip;
use crate::projection::ScaleAdjust;
use crate::projection::TransformExtent;
use crate::stream::Stream;
use crate::Transform;

use super::ReclipAdjust;

// TODO: Must vary by :-
// ResampleNoClipC/U,
// ResampleNoneNocClipC/U

impl<DRAIN, PR, T> ScaleAdjust for BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>
where
	DRAIN: Clone + Debug + Default + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T> + TransformExtent<T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn scale(mut self, scale: T) -> Self {
		self.base = self.base.scale(scale);
		self.reclip_adjust()
	}
}

impl<DRAIN, PR, T> ScaleAdjust for BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T>
where
	DRAIN: Clone + Default + Debug + Stream<EP = DRAIN, T = T>,
	PR: Clone + Debug + Transform<T = T> + TransformExtent<T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn scale(mut self, scale: T) -> Self {
		self.base = self.base.scale(scale);
		self.reclip_adjust()
	}
}
