use crate::clip::PointVisible;
use crate::projection::builder::template::ClipU;
use crate::projection::builder_mercator::Builder;
use crate::projection::builder_mercator::NoClipU;
use crate::projection::AsPrimitive;
use crate::projection::ClipExtentGet;
use crate::projection::TransformExtent;
use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

impl<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, T> ClipExtentGet
	for Builder<DRAIN, I, LB, LC, LU, ClipU<DRAIN, T>, PR, PV, RC, RU, T>
where
	PR: TransformExtent<T = T>,
	PV: PointVisible<T = T>,
	T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	/// f64 or f32.
	type T = T;

	/// Returns a bounding box.
	#[inline]
	fn get_clip_extent(&self) -> Option<[Coordinate<Self::T>; 2]> {
		self.extent
	}
}
