use crate::clip::antimeridian::ClipAntimeridianU;

use crate::clip::circle::ClipCircleU;
use crate::projection::builder::template::ResampleNoneNoPCNC;
use crate::projection::builder::template::ResampleNoneNoPCNU;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::NoPCNU;
use crate::projection::builder::ResampleNoPCNC;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::builder::ResamplePCNC;
use crate::projection::builder::ResamplePCNU;
use crate::projection::builder::PCNU;

use super::Builder;

/// A common builder with a Antimerdian clipping stratergy, no resampling and no post clip node.
pub type BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU,
    PR,
    ResampleNoneNoPCNU<PR, T>,
    T,
>;

/// A common builder with a Antimerdian clipping stratergy, no resampling and a post clip node.
pub type BuilderAntimeridianResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<T>,
    PR,
    ResampleNonePCNU<PR, T>,
    T,
>;

/// A common builder with a Antimerdian clipping stratergy, resampling and no post clip node.
pub type BuilderAntimeridianResampleNoClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResampleNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU,
    PR,
    ResampleNoPCNU<PR, T>,
    T,
>;

/// A common builder with a Antimerdian clipping stratergy, resampling and a post clip node.
pub type BuilderAntimeridianResampleClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<T>,
    PR,
    ResamplePCNU<PR, T>,
    T,
>;

/// A common builder with a circle clipping stratergy, resampling and no post clip node.
pub type BuilderCircleResampleNoClip<DRAIN, PR, T> = Builder<
    ClipCircleU<ResampleNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU,
    PR,
    ResampleNoPCNU<PR, T>,
    T,
>;

/// A common builder with a circle clipping stratergy, no resampling and no post clip node.
pub type BuilderCircleResampleNoneNoClip<DRAIN, PR, T> = Builder<
    ClipCircleU<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU,
    PR,
    ResampleNoneNoPCNU<PR, T>,
    T,
>;

/// A common builder with a circle clipping stratergy, resampling and post clip node.
pub type BuilderCircleResampleClip<DRAIN, PR, T> =
    Builder<ClipCircleU<ResamplePCNC<DRAIN, PR, T>, T>, DRAIN, PCNU<T>, PR, ResamplePCNU<PR, T>, T>;

/// A common builder with a circle clipping stratergy, no resampling and a post clip node.
pub type BuilderCircleResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipCircleU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<T>,
    PR,
    ResampleNonePCNU<PR, T>,
    T,
>;
