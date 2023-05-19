use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleU;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;
use crate::projection::builder_mercator::Builder;

/// A mercator builder with a Antimerdian clipping stratergy, resampling and a post clip node.
pub type BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T> =
    Builder<ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>, PCNU<T>, PR, ResamplePCNU<PR, T>, T>;

/// A mercator builder with a Antimerdian clipping stratergy, no resampling and a post clip node.
pub type BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    PCNU<T>,
    PR,
    ResampleNonePCNU<PR, T>,
    T,
>;

/// A mercator builder with a circle clipping stratergy, resampling and a post clip node.
pub type BuilderMercatorCircleResampleClip<DRAIN, PR, T> =
    Builder<ClipCircleU<ResamplePCNC<DRAIN, PR, T>, T>, PCNU<T>, PR, ResamplePCNU<PR, T>, T>;

/// A mercator builder with a circle clipping stratergy, no resampling and a post clip node.
pub type BuilderMercatorCircleResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipCircleU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    PCNU<T>,
    PR,
    ResampleNonePCNU<PR, T>,
    T,
>;
