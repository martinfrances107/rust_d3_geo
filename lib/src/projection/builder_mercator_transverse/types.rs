use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleU;
use crate::clip::rectangle::Rectangle;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder_mercator_transverse::Builder;
use crate::stream::Unconnected;

/// A mercator transverse builder with a antimeridian clipping strategy, with resampling and post clip node.
pub type BuilderMercatorTransverseAntimeridianResampleClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Rectangle<Unconnected, T>,
    PR,
    ResamplePCNU<PR, T>,
    T,
>;

/// A mercator transverse builder with a antimeridian clipping strategy, with no resampling and post clip node.
pub type BuilderMercatorTransverseAntimeridianResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Rectangle<Unconnected, T>,
    PR,
    ResampleNonePCNU<PR, T>,
    T,
>;

/// A mercator transverse builder with a circle clipping strategy, with  resampling and a post clip node.
pub type BuilderMercatorTransverseCircleResampleClip<DRAIN, PR, T> = Builder<
    ClipCircleU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Rectangle<Unconnected, T>,
    PR,
    ResamplePCNU<PR, T>,
    T,
>;

/// A mercator transverse builder with a circle clipping strategy, with no resampling and a post clip node.
pub type BuilderMercatorTransverseCircleResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipCircleU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Rectangle<Unconnected, T>,
    PR,
    ResampleNonePCNU<PR, T>,
    T,
>;
