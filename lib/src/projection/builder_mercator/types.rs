use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleC;
use crate::clip::circle::ClipCircleU;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;
use crate::projection::builder_mercator::Builder;
use crate::projection::builder_mercator::NoPCNU;
use crate::projection::builder_mercator::ResampleNoPCNC;
use crate::projection::builder_mercator::ResampleNoPCNU;

pub type BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResamplePCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<DRAIN, T>,
    PR,
    ResamplePCNC<DRAIN, PR, T>,
    ResamplePCNU<DRAIN, PR, T>,
    T,
>;

pub type BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleNonePCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<DRAIN, T>,
    PR,
    ResampleNonePCNC<DRAIN, PR, T>,
    ResampleNonePCNU<DRAIN, PR, T>,
    T,
>;

/// Visibility restricted to pub(super).
/// The concept of NoClip in the mercator context is only useful for initialization.
/// Once constructed all mutations end with a reclip() which can only result
/// in something ResampleXClip...
///
/// So there is no reson for it to appear on any public API.
pub type BuilderMercatorAntimeridianResampleNoClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleNoPCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU<DRAIN>,
    PR,
    ResampleNoPCNC<DRAIN, PR, T>,
    ResampleNoPCNU<DRAIN, PR, T>,
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
    ClipCircleC<ResamplePCNC<DRAIN, PR, T>, T>,
    ClipCircleU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<DRAIN, T>,
    PR,
    ResamplePCNC<DRAIN, PR, T>,
    ResamplePCNU<DRAIN, PR, T>,
    T,
>;

pub type BuilderMercatorCircleResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResampleNonePCNC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<DRAIN, T>,
    PR,
    ResampleNonePCNC<DRAIN, PR, T>,
    ResampleNonePCNU<DRAIN, PR, T>,
    T,
>;
