use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleC;
use crate::clip::circle::ClipCircleU;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder_mercator::Builder;
use crate::projection::builder_mercator::NoClipU;
use crate::projection::builder_mercator::ResampleNoClipC;
use crate::projection::builder_mercator::ResampleNoClipU;

pub type BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleClipC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleClipC<DRAIN, PR, T>, T>,
    DRAIN,
    ClipU<DRAIN, T>,
    PR,
    ResampleClipC<DRAIN, PR, T>,
    ResampleClipU<DRAIN, PR, T>,
    T,
>;

pub type BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleNoneClipC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNoneClipC<DRAIN, PR, T>, T>,
    DRAIN,
    ClipU<DRAIN, T>,
    PR,
    ResampleNoneClipC<DRAIN, PR, T>,
    ResampleNoneClipU<DRAIN, PR, T>,
    T,
>;

/// Visibility restricted to pub(super).
/// The concept of NoClip in the mercator context is only useful for initialization.
/// Once constructed all mutations end with a reclip() which can only result
/// in something ResampleXClip...
///
/// So there is no reson for it to appear on any public API.
pub type BuilderMercatorAntimeridianResampleNoClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleNoClipC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNoClipC<DRAIN, PR, T>, T>,
    DRAIN,
    NoClipU<DRAIN>,
    PR,
    ResampleNoClipC<DRAIN, PR, T>,
    ResampleNoClipU<DRAIN, PR, T>,
    T,
>;

// pub(super) type BuilderMercatorAntimeridianResampleNoneNoClip<DRAIN, PR, T> = Builder<
//     DRAIN,
//     InterpolateAntimeridian<T>,
//     LineAntimeridian<Buffer<T>, Connected<Buffer<T>>, T>,
//     LineAntimeridian<
//         ResampleNoneNoClipC<DRAIN, PR, T>,
//         Connected<ResampleNoneNoClipC<DRAIN, PR, T>>,
//         T,
//     >,
//     LineAntimeridian<ResampleNoneNoClipC<DRAIN, PR, T>, Unconnected, T>,
//     NoClipU<DRAIN>,
//     PR,
//     PVAntimeridian<T>,
//     ResampleNoneNoClipC<DRAIN, PR, T>,
//     ResampleNoneNoClipU<DRAIN, PR, T>,
//     T,
// >;

pub type BuilderMercatorCircleResampleClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResampleClipC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleClipC<DRAIN, PR, T>, T>,
    DRAIN,
    ClipU<DRAIN, T>,
    PR,
    ResampleClipC<DRAIN, PR, T>,
    ResampleClipU<DRAIN, PR, T>,
    T,
>;

pub type BuilderMercatorCircleResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResampleNoneClipC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNoneClipC<DRAIN, PR, T>, T>,
    DRAIN,
    ClipU<DRAIN, T>,
    PR,
    ResampleNoneClipC<DRAIN, PR, T>,
    ResampleNoneClipU<DRAIN, PR, T>,
    T,
>;
