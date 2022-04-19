use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::builder_mercator::ResampleNoClipC;
use crate::projection::builder_mercator::ResampleNoClipU;
use crate::projection::ClipExtentSet;
use crate::projection::TransformExtent;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Builder;
use super::Reclip;

// TOD must vary by ClipAntimeridian -- 2 more impl blocks

impl<DRAIN, PR, T> ClipExtentSet
	for Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			ResampleNoClipC<DRAIN, PR, T>,
			Connected<ResampleNoClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimeridian<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: 'static + Clone + Default + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T> + TransformExtent<T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	type OutputBounded = Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			ResampleClipC<DRAIN, PR, T>,
			Connected<ResampleClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimeridian<DRAIN, ResampleClipC<DRAIN, PR, T>, Unconnected, T>,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleClipC<DRAIN, PR, T>,
		ResampleClipU<DRAIN, PR, T>,
		T,
	>;

	fn clip_extent(mut self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
		self.extent = Some(*extent);
		self.reclip()
	}
}

impl<DRAIN, PR, T> ClipExtentSet
	for Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			ResampleNoneNoClipC<DRAIN, PR, T>,
			Connected<ResampleNoneNoClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimeridian<DRAIN, ResampleNoneNoClipC<DRAIN, PR, T>, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleNoneNoClipC<DRAIN, PR, T>,
		ResampleNoneNoClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: 'static + Clone + Default + Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T> + TransformExtent<T>,
	T: 'static + AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + FloatConst,
{
	type T = T;

	type OutputBounded = Builder<
		DRAIN,
		InterpolateAntimeridian<T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			DRAIN,
			ResampleNoneClipC<DRAIN, PR, T>,
			Connected<ResampleNoneClipC<DRAIN, PR, T>>,
			T,
		>,
		LineAntimeridian<DRAIN, ResampleNoneClipC<DRAIN, PR, T>, Unconnected, T>,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PVAntimeridian<T>,
		ResampleNoneClipC<DRAIN, PR, T>,
		ResampleNoneClipU<DRAIN, PR, T>,
		T,
	>;

	fn clip_extent(mut self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
		self.extent = Some(*extent);
		self.reclip()
	}
}

impl<DRAIN, PR, T> ClipExtentSet
	for Builder<
		DRAIN,
		InterpolateCircle<T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			DRAIN,
			ResampleNoClipC<DRAIN, PR, T>,
			Connected<ResampleNoClipC<DRAIN, PR, T>>,
			T,
		>,
		LineCircle<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PVCircle<T>,
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	PR: Clone + Transform<T = T>,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	type OutputBounded = Builder<
		DRAIN,
		InterpolateCircle<T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<DRAIN, ResampleClipC<DRAIN, PR, T>, Connected<ResampleClipC<DRAIN, PR, T>>, T>,
		LineCircle<DRAIN, ResampleClipC<DRAIN, PR, T>, Unconnected, T>,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PVCircle<T>,
		ResampleClipC<DRAIN, PR, T>,
		ResampleClipU<DRAIN, PR, T>,
		T,
	>;

	fn clip_extent(self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
		let base = self.base.clip_extent(extent);

		// Architecture Discussion:
		// CLIP is generic over <.. RC, RU,..>,
		// So a change in the resample type causes rebuilding of clip.
		let out = Self::OutputBounded {
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

impl<DRAIN, PR, T> ClipExtentSet
	for Builder<
		DRAIN,
		InterpolateCircle<T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			DRAIN,
			ResampleNoneNoClipC<DRAIN, PR, T>,
			Connected<ResampleNoneNoClipC<DRAIN, PR, T>>,
			T,
		>,
		LineCircle<DRAIN, ResampleNoneNoClipC<DRAIN, PR, T>, Unconnected, T>,
		NoClipC<DRAIN, T>,
		NoClipU<DRAIN, T>,
		PR,
		PVCircle<T>,
		ResampleNoneNoClipC<DRAIN, PR, T>,
		ResampleNoneNoClipU<DRAIN, PR, T>,
		T,
	> where
	PR: Clone,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	type OutputBounded = Builder<
		DRAIN,
		InterpolateCircle<T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<
			DRAIN,
			ResampleNoneClipC<DRAIN, PR, T>,
			Connected<ResampleNoneClipC<DRAIN, PR, T>>,
			T,
		>,
		LineCircle<DRAIN, ResampleNoneClipC<DRAIN, PR, T>, Unconnected, T>,
		ClipC<DRAIN, T>,
		ClipU<DRAIN, T>,
		PR,
		PVCircle<T>,
		ResampleNoneClipC<DRAIN, PR, T>,
		ResampleNoneClipU<DRAIN, PR, T>,
		T,
	>;

	fn clip_extent(self, extent: &[Coordinate<T>; 2]) -> Self::OutputBounded {
		let base = self.base.clip_extent(extent);

		// Architecture Discussion:
		// CLIP is generic over <.. RC, RU,..>,
		// So a change in the resample type causes rebuilding of clip.
		let out = Self::OutputBounded {
			// p_pcnc: PhantomData::<ClipC<DRAIN, T>>,
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
