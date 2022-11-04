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

/// A mercator builder with a Antimerdian clipping stratergy, resampling and a post clip node.
pub type BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResamplePCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<T>,
    PR,
    ResamplePCNC<DRAIN, PR, T>,
    ResamplePCNU<PR, T>,
    T,
>;

/// A mercator builder with a Antimerdian clipping stratergy, no resampling and a post clip node.
pub type BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleNonePCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<T>,
    PR,
    ResampleNonePCNC<DRAIN, PR, T>,
    ResampleNonePCNU<PR, T>,
    T,
>;

/// A mercator builder with a circle clipping stratergy, resampling and a post clip node.
pub type BuilderMercatorCircleResampleClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResamplePCNC<DRAIN, PR, T>, T>,
    ClipCircleU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<T>,
    PR,
    ResamplePCNC<DRAIN, PR, T>,
    ResamplePCNU<PR, T>,
    T,
>;

/// A mercator builder with a circle clipping stratergy, no resampling and a post clip node.
pub type BuilderMercatorCircleResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResampleNonePCNC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<T>,
    PR,
    ResampleNonePCNC<DRAIN, PR, T>,
    ResampleNonePCNU<PR, T>,
    T,
>;
