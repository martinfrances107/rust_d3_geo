use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::builder::template::ClipU;
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
	LineAntimeridian<Buffer<T>, Connected<Buffer<T>>, T>,
	LineAntimeridian<
		ResampleNoneNoClipC<DRAIN, PR, T>,
		Connected<ResampleNoneNoClipC<DRAIN, PR, T>>,
		T,
	>,
	LineAntimeridian<ResampleNoneNoClipC<DRAIN, PR, T>, Unconnected, T>,
	NoClipU<DRAIN>,
	PR,
	PVAntimeridian<T>,
	ResampleNoneNoClipC<DRAIN, PR, T>,
	ResampleNoneNoClipU<DRAIN, PR, T>,
	T,
>;

pub type ProjectorAntimeridianResampleNoneClip<DRAIN, PR, T> = Projector<
	DRAIN,
	InterpolateAntimeridian<T>,
	LineAntimeridian<Buffer<T>, Connected<Buffer<T>>, T>,
	LineAntimeridian<
		ResampleNoneClipC<DRAIN, PR, T>,
		Connected<ResampleNoneClipC<DRAIN, PR, T>>,
		T,
	>,
	LineAntimeridian<ResampleNoneClipC<DRAIN, PR, T>, Unconnected, T>,
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
	LineAntimeridian<Buffer<T>, Connected<Buffer<T>>, T>,
	LineAntimeridian<ResampleNoClipC<DRAIN, PR, T>, Connected<ResampleNoClipC<DRAIN, PR, T>>, T>,
	LineAntimeridian<ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
	NoClipU<DRAIN>,
	PR,
	PVAntimeridian<T>,
	ResampleNoClipC<DRAIN, PR, T>,
	ResampleNoClipU<DRAIN, PR, T>,
	T,
>;

pub type ProjectorAntimeridianResampleClip<DRAIN, PR, T> = Projector<
	DRAIN,
	InterpolateAntimeridian<T>,
	LineAntimeridian<Buffer<T>, Connected<Buffer<T>>, T>,
	LineAntimeridian<ResampleClipC<DRAIN, PR, T>, Connected<ResampleClipC<DRAIN, PR, T>>, T>,
	LineAntimeridian<ResampleClipC<DRAIN, PR, T>, Unconnected, T>,
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
	LineCircle<Buffer<T>, Connected<Buffer<T>>, T>,
	LineCircle<ResampleNoneNoClipC<DRAIN, PR, T>, Connected<ResampleNoneNoClipC<DRAIN, PR, T>>, T>,
	LineCircle<ResampleNoneNoClipC<DRAIN, PR, T>, Unconnected, T>,
	NoClipU<DRAIN>,
	PR,
	PVCircle<T>,
	ResampleNoneNoClipC<DRAIN, PR, T>,
	ResampleNoneNoClipU<DRAIN, PR, T>,
	T,
>;

pub type ProjectorCircleResampleNoneClip<DRAIN, PR, T> = Projector<
	DRAIN,
	InterpolateCircle<T>,
	LineCircle<Buffer<T>, Connected<Buffer<T>>, T>,
	LineCircle<ResampleNoneClipC<DRAIN, PR, T>, Connected<ResampleNoneClipC<DRAIN, PR, T>>, T>,
	LineCircle<ResampleNoneClipC<DRAIN, PR, T>, Unconnected, T>,
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
	LineCircle<Buffer<T>, Connected<Buffer<T>>, T>,
	LineCircle<ResampleNoClipC<DRAIN, PR, T>, Connected<ResampleNoClipC<DRAIN, PR, T>>, T>,
	LineCircle<ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
	NoClipU<DRAIN>,
	PR,
	PVCircle<T>,
	ResampleNoClipC<DRAIN, PR, T>,
	ResampleNoClipU<DRAIN, PR, T>,
	T,
>;

pub type ProjectorCircleResampleClip<DRAIN, PR, T> = Projector<
	DRAIN,
	InterpolateCircle<T>,
	LineCircle<Buffer<T>, Connected<Buffer<T>>, T>,
	LineCircle<ResampleClipC<DRAIN, PR, T>, Connected<ResampleClipC<DRAIN, PR, T>>, T>,
	LineCircle<ResampleClipC<DRAIN, PR, T>, Unconnected, T>,
	ClipU<DRAIN, T>,
	PR,
	PVCircle<T>,
	ResampleClipC<DRAIN, PR, T>,
	ResampleClipU<DRAIN, PR, T>,
	T,
>;
