// use crate::clip::buffer::Buffer;
// use crate::clip::clip::Clip;
// use crate::projection::builder::template::NoClipC;
// use crate::projection::builder::template::NoClipU;
// use crate::projection::builder::template::ResampleNoneNoClipC;
// use crate::projection::builder::template::ResampleNoneNoClipU;
// use crate::stream::Connected;
// use crate::stream::Unconnected;

// use super::interpolate::Interpolate;
// use super::line::Line;
// use super::pv::PV;

// Used by clip generators in the building the default Clip.
// pub type Default<DRAIN, PR, T> = Clip<
// 	DRAIN,
// 	Interpolate<DRAIN, ResampleNoneNoClipC<DRAIN, PR, T>, T>,
// 	Line<DRAIN, ResampleNoneNoClipU<DRAIN, PR, T>, Unconnected, T>,
// 	PR,
// 	PV<T>,
// 	ResampleNoneNoClipC<DRAIN, PR, T>,
// 	ResampleNoneNoClipU<DRAIN, PR, T>,
// 	Unconnected,
// 	T,
// >;
