use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::builder::ClipC;
use crate::projection::builder::ClipU;
use crate::projection::builder::NoClipC;
use crate::projection::builder::NoClipU;
use crate::projection::builder::ResampleClipC;
use crate::projection::builder::ResampleClipU;
use crate::projection::builder::ResampleNoClipC;
use crate::projection::builder::ResampleNoClipU;
use crate::stream::Connected;
use crate::stream::Unconnected;

use super::Builder;

pub type BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T> = Builder<
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
>;

pub type BuilderAntimeridianResampleNoneClip<DRAIN, PR, T> = Builder<
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

pub type BuilderAntimeridianResampleNoClip<DRAIN, PR, T> = Builder<
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
>;

pub type BuilderAntimeridianResampleClip<DRAIN, PR, T> = Builder<
	DRAIN,
	InterpolateAntimeridian<T>,
	LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
	LineAntimeridian<DRAIN, ResampleClipC<DRAIN, PR, T>, Connected<ResampleClipC<DRAIN, PR, T>>, T>,
	LineAntimeridian<DRAIN, ResampleClipC<DRAIN, PR, T>, Unconnected, T>,
	ClipC<DRAIN, T>,
	ClipU<DRAIN, T>,
	PR,
	PVAntimeridian<T>,
	ResampleClipC<DRAIN, PR, T>,
	ResampleClipU<DRAIN, PR, T>,
	T,
>;

pub type BuilderCircleResampleNoClip<DRAIN, PR, T> = Builder<
	DRAIN,
	InterpolateCircle<T>,
	LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
	LineCircle<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Connected<ResampleNoClipC<DRAIN, PR, T>>, T>,
	LineCircle<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
	NoClipC<DRAIN, T>,
	NoClipU<DRAIN, T>,
	PR,
	PVCircle<T>,
	ResampleNoClipC<DRAIN, PR, T>,
	ResampleNoClipU<DRAIN, PR, T>,
	T,
>;

pub type BuilderCircleResampleNoneNoClip<DRAIN, PR, T> = Builder<
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
>;

pub type BuilderCircleResampleClip<DRAIN, PR, T> = Builder<
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

pub type BuilderCircleResampleNoneClip<DRAIN, PR, T> = Builder<
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
