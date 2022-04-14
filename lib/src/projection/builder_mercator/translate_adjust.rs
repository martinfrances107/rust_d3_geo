use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder_mercator::Reclip;
use std::fmt::Debug;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::rectangle::Rectangle;
use crate::clip::Bufferable;
use crate::clip::Clean;
use crate::clip::Interpolator;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::builder_mercator::Buffer;
use crate::projection::builder_mercator::ReclipAdjust;
use crate::projection::builder_mercator::ResampleNoClipC;
use crate::projection::builder_mercator::ResampleNoClipU;
use crate::projection::resampler::none::None as ResampleNone;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::TranslateAdjust;
use crate::projection::TranslateSet;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;

impl<DRAIN, I, LB, LC, LU, PR, T> TranslateAdjust
	for Builder<
		DRAIN,
		// InterpolateAntimeridian<T>,
		// LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		// LineAntimeridian<
		// 	DRAIN,
		// 	ResampleNoClipC<DRAIN, PR, T>,
		// 	Connected<ResampleClipC<DRAIN, PR, T>>,
		// 	T,
		// >,
		// LineAntimeridian<DRAIN, ResampleClipC<DRAIN, PR, T>, Unconnected, T>,
		I,
		LB,
		LC,
		LU,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleClipC<DRAIN, PR, T>,
		ResampleClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: 'static + Clone + Default + Debug + Stream<EP = DRAIN, T = T>,
	I: Clone + Interpolator<T = T>,
	LB: Clone + Debug + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
	LC: Clone + LineConnected<SC = ResampleClipC<DRAIN, PR, T>> + Stream<EP = DRAIN, T = T>,
	LU: Clone
		+ Connectable<Output = LC, SC = ResampleClipC<DRAIN, PR, T>>
		+ Bufferable<Output = LB, T = T>
		+ Debug,
	PR: Clone + Debug + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn translate_adjust(mut self, t: &Coordinate<T>) -> Self {
		// types are changing rebuild base.

		self.base = self.base.translate_adjust(t);
		self.reclip_adjust()
		// let out = Self::Output {
		// 	pr: self.pr,
		// 	base,
		// 	x0: self.x0,
		// 	y0: self.y0,
		// 	x1: self.x1,
		// 	y1: self.y1, // post-clip extent
		// };
		// self.reclip()
	}
}

impl<DRAIN, I, LB, LC, LU, PR, PV, T> TranslateAdjust
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
		ResampleNoneClipC<DRAIN, PR, T>,
		ResampleNoneClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: 'static + Clone + Default + Debug + Stream<EP = DRAIN, T = T>,
	I: Clone,
	LB: Clone + Debug,
	LC: Clone,
	LU: Clone,
	PR: Clone + Debug + Transform<T = T>,
	PV: Clone,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn translate_adjust(mut self, t: &Coordinate<T>) -> Self {
		self.base = self.base.translate_adjust(t);
		self.reclip_adjust()
	}
}
