use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::builder_mercator::Builder;
use crate::projection::builder_mercator::NoClipU;
use crate::projection::builder_mercator::ResampleNoClipC;
use crate::projection::builder_mercator::ResampleNoClipU;
use crate::stream::Connected;
use crate::stream::Unconnected;

pub type BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T> = Builder<
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

pub type BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T> = Builder<
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

//keep
pub type BuilderMercatorAntimeridianResampleNoClip<DRAIN, PR, T> = Builder<
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

/// Visibility restricted to pub(super).
/// The concept of NoClip in the mercator context is only useful for initialization.
/// Once constructed all mutations end with a reclip() which can only result
/// in something ResampleXClip...
///
/// So there is no reson for it to appear on any public API.
pub(super) type BuilderMercatorAntimeridianResampleNoneNoClip<DRAIN, PR, T> = Builder<
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

pub type BuilderMercatorCircleResampleClip<DRAIN, PR, T> = Builder<
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

pub type BuilderMercatorCircleResampleNoneClip<DRAIN, PR, T> = Builder<
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
