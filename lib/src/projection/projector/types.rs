use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::builder::template::ClipC;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipC;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoClipC;
use crate::projection::builder::template::ResampleNoClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::projector::Buffer;
use crate::projection::Projector;
use crate::stream::Connected;
use crate::stream::Unconnected;

pub type ProjectorAntimeridianResampleNoneNoClip<DRAIN, PR, T> = Projector<
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

pub type ProjectorAntimeridianResampleNoneClip<DRAIN, PR, T> = Projector<
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

pub type ProjectorAntimeridianResampleNoClip<DRAIN, PR, T> = Projector<
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

pub type ProjectorAntimeridianResampleClip<DRAIN, PR, T> = Projector<
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

pub type ProjectorCircleResampleNoneNoClip<DRAIN, PR, T> = Projector<
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

pub type ProjectorCircleResampleNoneClip<DRAIN, PR, T> = Projector<
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

pub type ProjectorCircleResampleNoClip<DRAIN, PR, T> = Projector<
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

pub type ProjectorCircleResampleClip<DRAIN, PR, T> = Projector<
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
