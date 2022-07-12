use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::builder::types::BuilderAntimeridianResampleClip;
use crate::projection::builder::Builder;
use crate::projection::builder::ClipC;
use crate::projection::builder::ClipU;
use crate::projection::builder::NoClipC;
use crate::projection::builder::NoClipU;
use crate::projection::builder::ResampleClipC;
use crate::projection::builder::ResampleClipU;
use crate::projection::builder::ResampleNoClipC;
use crate::projection::builder::ResampleNoClipU;
use crate::projection::projector::types::ProjectorAntimeridianResampleClip;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Build;
use crate::projection::Projector;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;

impl<DRAIN, I, LC, LB, LU, PR, PV, T> Build
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
	DRAIN: Clone + Stream<EP = DRAIN, T = T>,
	I: Clone,
	LB: Clone,
	LC: Clone,
	LU: Clone,
	PV: Clone,
	PR: Clone,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Drain = DRAIN;
	type I = I;
	type LB = LB;
	type LC = LC;
	type LU = LU;
	type PCNU = ClipU<DRAIN, T>;
	type PR = PR;
	type PV = PV;
	type RC = ResampleNoneClipC<DRAIN, PR, T>;
	type RU = ResampleNoneClipU<DRAIN, PR, T>;
	type T = T;
	/// Using the currently programmed state output a new projection.
	#[inline]
	fn build(
		&self,
	) -> Projector<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		ClipU<DRAIN, T>,
		PR,
		PV,
		ResampleNoneClipC<DRAIN, PR, T>,
		ResampleNoneClipU<DRAIN, PR, T>,
		T,
	> {
		Projector {
			cache: None,
			postclip: self.postclip.clone(),
			clip: self.clip.clone(),
			resample: self.resample.clone(),
			rotator: self.rotator.clone(),
			project_rotate_transform: self.project_rotate_transform.clone(),
			transform_radians: StreamTransformRadians(Unconnected),
		}
	}

	// fn reset(self) -> Self {
	//     // self.cache_stream = None;
	//     // self.cache = None;
	//     self
	// }
}

impl<DRAIN, I, LC, LB, LU, PR, PV, T> Build
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		NoClipC<DRAIN>,
		NoClipU<DRAIN>,
		PR,
		PV,
		ResampleNoneNoClipC<DRAIN, PR, T>,
		ResampleNoneNoClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Clone + Stream<EP = DRAIN, T = T>,
	I: Clone,
	LB: Clone,
	LC: Clone,
	LU: Clone,
	PV: Clone,
	PR: Clone,
	PR: Clone,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Drain = DRAIN;
	type I = I;
	type LB = LB;
	type LC = LC;
	type LU = LU;
	type PCNU = NoClipU<DRAIN>;
	type PR = PR;
	type PV = PV;
	type RC = ResampleNoneNoClipC<DRAIN, PR, T>;
	type RU = ResampleNoneNoClipU<DRAIN, PR, T>;
	type T = T;
	/// Using the currently programmed state output a new projection.
	#[inline]
	fn build(
		&self,
	) -> Projector<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		NoClipU<DRAIN>,
		PR,
		PV,
		ResampleNoneNoClipC<DRAIN, PR, T>,
		ResampleNoneNoClipU<DRAIN, PR, T>,
		T,
	> {
		Projector {
			cache: None,
			postclip: self.postclip.clone(),
			clip: self.clip.clone(),
			resample: self.resample.clone(),
			rotator: self.rotator.clone(),
			project_rotate_transform: self.project_rotate_transform.clone(),
			transform_radians: StreamTransformRadians(Unconnected),
		}
	}

	// fn reset(self) -> Self {
	//     // self.cache_stream = None;
	//     // self.cache = None;
	//     self
	// }
}

// impl<DRAIN, PR, T> Build for BuilderAntimeridianResampleNoClip<DRAIN, PR, T>
impl<DRAIN, I, LC, LB, LU, PR, PV, T> Build
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		NoClipC<DRAIN>,
		NoClipU<DRAIN>,
		PR,
		PV,
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Clone + Stream<EP = DRAIN, T = T>,
	I: Clone,
	LB: Clone,
	LC: Clone,
	LU: Clone,
	PV: Clone,
	PR: Clone,
	PR: Clone,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Drain = DRAIN;
	type I = I;
	type LB = LB;
	type LC = LC;
	type LU = LU;
	type PCNU = NoClipU<DRAIN>;
	type PR = PR;
	type PV = PV;
	type RC = ResampleNoClipC<DRAIN, PR, T>;
	type RU = ResampleNoClipU<DRAIN, PR, T>;
	type T = T;

	/// Using the currently programmed state output a new projection.
	#[inline]
	fn build(
		&self,
	) -> Projector<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		NoClipU<DRAIN>,
		PR,
		PV,
		ResampleNoClipC<DRAIN, PR, T>,
		ResampleNoClipU<DRAIN, PR, T>,
		T,
	> {
		Projector {
			cache: None,
			postclip: self.postclip.clone(),
			clip: self.clip.clone(),
			resample: self.resample.clone(),
			rotator: self.rotator.clone(),
			project_rotate_transform: self.project_rotate_transform.clone(),
			transform_radians: StreamTransformRadians(Unconnected),
		}
	}

	// fn reset(self) -> Self {
	//     // self.cache_stream = None;
	//     // self.cache = None;
	//     self
	// }
}

impl<DRAIN, PR, T> Build for BuilderAntimeridianResampleClip<DRAIN, PR, T>
where
	DRAIN: Clone + Stream<EP = DRAIN, T = T>,
	PR: Clone,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Drain = DRAIN;
	type I = InterpolateAntimeridian<T>;
	type LB = LineAntimeridian<Buffer<T>, Connected<Buffer<T>>, T>;
	type LC =
		LineAntimeridian<ResampleClipC<DRAIN, PR, T>, Connected<ResampleClipC<DRAIN, PR, T>>, T>;
	type LU = LineAntimeridian<ResampleClipC<DRAIN, PR, T>, Unconnected, T>;
	type PCNU = ClipU<DRAIN, T>;
	type PR = PR;
	type PV = PVAntimeridian<T>;
	type RC = ResampleClipC<DRAIN, PR, T>;
	type RU = ResampleClipU<DRAIN, PR, T>;
	type T = T;
	/// Using the currently programmed state output a new projection.
	#[inline]
	fn build(&self) -> ProjectorAntimeridianResampleClip<DRAIN, PR, T> {
		Projector {
			cache: None,
			postclip: self.postclip.clone(),
			clip: self.clip.clone(),
			resample: self.resample.clone(),
			rotator: self.rotator.clone(),
			project_rotate_transform: self.project_rotate_transform.clone(),
			transform_radians: StreamTransformRadians(Unconnected),
		}
	}

	// fn reset(self) -> Self {
	//     // self.cache_stream = None;
	//     // self.cache = None;
	//     self
	// }
}
