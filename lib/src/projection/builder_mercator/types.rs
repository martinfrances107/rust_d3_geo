use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleU;
use crate::clip::rectangle::Rectangle;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder_mercator::Builder;
use crate::stream::Unconnected;

/// A mercator builder with a Antimerdian clipping strategy, resampling and a post clip node.
pub type BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Rectangle<Unconnected, T>,
    PR,
    ResamplePCNU<PR, T>,
    T,
>;

/// A mercator builder with a Antimerdian clipping strategy, no resampling and a post clip node.
pub type BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Rectangle<Unconnected, T>,
    PR,
    ResampleNonePCNU<PR, T>,
    T,
>;

/// A mercator builder with a circle clipping strategy, resampling and a post clip node.
pub type BuilderMercatorCircleResampleClip<DRAIN, PR, T> = Builder<
    ClipCircleU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Rectangle<Unconnected, T>,
    PR,
    ResamplePCNU<PR, T>,
    T,
>;

/// A mercator builder with a circle clipping strategy, no resampling and a post clip node.
pub type BuilderMercatorCircleResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipCircleU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Rectangle<Unconnected, T>,
    PR,
    ResampleNonePCNU<PR, T>,
    T,
>;
