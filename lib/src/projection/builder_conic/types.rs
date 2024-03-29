use crate::projection::builder::types::BuilderAntimeridianResampleClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoneClip;
use crate::projection::builder::types::BuilderAntimeridianResampleNoneNoClip;
use crate::projection::builder::types::BuilderCircleResampleClip;
use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::builder::types::BuilderCircleResampleNoneClip;
use crate::projection::builder::types::BuilderCircleResampleNoneNoClip;

use super::Builder;

/// A common builder with a Antimerdian clipping strategy, no resampling and no post clip node.
pub type BuilderConicAntimeridianResampleNoneNoClip<DRAIN, PR, T> =
    Builder<BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T>, T>;

/// A common builder with a Antimerdian clipping strategy, no resampling and a post clip node.
pub type BuilderConicAntimeridianResampleNoneClip<DRAIN, PR, T> =
    Builder<BuilderAntimeridianResampleNoneClip<DRAIN, PR, T>, T>;

/// A common builder with a Antimerdian clipping strategy, resampling and no post clip node.
pub type BuilderConicAntimeridianResampleNoClip<DRAIN, PR, T> =
    Builder<BuilderAntimeridianResampleNoClip<DRAIN, PR, T>, T>;

/// A common builder with a Antimerdian clipping strategy, resampling and a post clip node.
pub type BuilderConicAntimeridianResampleClip<DRAIN, PR, T> =
    Builder<BuilderAntimeridianResampleClip<DRAIN, PR, T>, T>;

/// A common builder with a circle clipping strategy, resampling and no post clip node.
pub type BuilderConicCircleResampleNoClip<DRAIN, PR, T> =
    Builder<BuilderCircleResampleNoClip<DRAIN, PR, T>, T>;

/// A common builder with a circle clipping strategy, no resampling and no post clip node.
pub type BuilderConicCircleResampleNoneNoClip<DRAIN, PR, T> =
    Builder<BuilderCircleResampleNoneNoClip<DRAIN, PR, T>, T>;

/// A common builder with a circle clipping strategy, resampling and post clip node.
pub type BuilderConicCircleResampleClip<DRAIN, PR, T> =
    Builder<BuilderCircleResampleClip<DRAIN, PR, T>, T>;

/// A common builder with a circle clipping strategy, no resampling and a post clip node.
pub type BuilderConicCircleResampleNoneClip<DRAIN, PR, T> =
    Builder<BuilderCircleResampleNoneClip<DRAIN, PR, T>, T>;
