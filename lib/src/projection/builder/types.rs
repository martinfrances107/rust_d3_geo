use crate::clip::antimeridian::ClipAntimeridianU;

use crate::clip::circle::ClipCircleU;
use crate::clip::rectangle::Rectangle;
use crate::identity::Identity;
use crate::projection::builder::template::ResampleNoneNoPCNC;
use crate::projection::builder::template::ResampleNoneNoPCNU;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::ResampleNoPCNC;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::builder::ResamplePCNC;
use crate::projection::builder::ResamplePCNU;
use crate::stream::Unconnected;

use super::Builder;

/// A common projection builder with a Antimerdian clipping strategy, no resampling and no post clip node.
pub type BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Identity<Unconnected>,
    PR,
    ResampleNoneNoPCNU<PR, T>,
    T,
>;

/// A common projection builder with a Antimerdian clipping strategy, no resampling and a post clip node.
pub type BuilderAntimeridianResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Rectangle<Unconnected, T>,
    PR,
    ResampleNonePCNU<PR, T>,
    T,
>;

/// A common projection builder with a Antimerdian clipping strategy, resampling and no post clip node.
pub type BuilderAntimeridianResampleNoClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResampleNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Identity<Unconnected>,
    PR,
    ResampleNoPCNU<PR, T>,
    T,
>;

/// A common projection builder with a Antimerdian clipping strategy, resampling and a post clip node.
pub type BuilderAntimeridianResampleClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Rectangle<Unconnected, T>,
    PR,
    ResamplePCNU<PR, T>,
    T,
>;

/// A common projection builder with a circle clipping strategy, resampling and no post clip node.
pub type BuilderCircleResampleNoClip<DRAIN, PR, T> = Builder<
    ClipCircleU<ResampleNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Identity<Unconnected>,
    PR,
    ResampleNoPCNU<PR, T>,
    T,
>;

/// A common projection builder with a circle clipping strategy, no resampling and no post clip node.
pub type BuilderCircleResampleNoneNoClip<DRAIN, PR, T> = Builder<
    ClipCircleU<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Identity<Unconnected>,
    PR,
    ResampleNoneNoPCNU<PR, T>,
    T,
>;

/// A common projection builder with a circle clipping strategy, resampling and post clip node.
pub type BuilderCircleResampleClip<DRAIN, PR, T> = Builder<
    ClipCircleU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Rectangle<Unconnected, T>,
    PR,
    ResamplePCNU<PR, T>,
    T,
>;

/// A common projection builder with a circle clipping strategy, no resampling and a post clip node.
pub type BuilderCircleResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipCircleU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    Rectangle<Unconnected, T>,
    PR,
    ResampleNonePCNU<PR, T>,
    T,
>;
