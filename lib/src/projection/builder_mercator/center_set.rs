use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::Bufferable;
use crate::clip::Clean;
use crate::clip::Interpolator;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::CenterSet;
use crate::projection::TransformExtent;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::Transform;

use super::Builder;
use super::ReclipAdjust;

impl<DRAIN, I, LB, LC, LU, PR, PV, T> CenterSet
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PV,
		ResampleClipC<DRAIN, PR, T>,
		ResampleClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Clone,
	I: Clone + Interpolator<T = T>,
	LB: Clone + Debug + LineConnected<SC = Buffer<T>> + Clean + Stream<EP = Buffer<T>, T = T>,
	LC: Clone + LineConnected<SC = ResampleClipC<DRAIN, PR, T>> + Stream<EP = DRAIN, T = T>,
	LU: Clone
		+ Debug
		+ Connectable<Output = LC, SC = ResampleClipC<DRAIN, PR, T>>
		+ Bufferable<Output = LB, T = T>,
	PR: Clone + Debug + Transform<T = T> + TransformExtent<T>,
	PV: Clone + PointVisible<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + Debug + FloatConst,
{
	type T = T;

	fn center(mut self, center: &Coordinate<T>) -> Self {
		self.base = self.base.center(center);
		// self
		self.reclip_adjust()
	}
}
