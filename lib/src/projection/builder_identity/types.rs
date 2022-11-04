use crate::projection::builder::template::NoPCNU;

use crate::projection::builder::template::PCNU;

use super::Builder;

/// A simplified builder with a antimeridian clipping stratergy, with resampling and no post clip node.
pub type BuilderIdentityAntimeridianResampleNoneNoClip<DRAIN, T> = Builder<DRAIN, NoPCNU, T>;

/// A simplified builder with a antimeridian clipping stratergy, with no resampling and post clip node.
pub type BuilderIdentityAntimeridianResampleNoneClip<DRAIN, T> = Builder<DRAIN, PCNU<T>, T>;

/// A simplified builder with a antimeridian clipping stratergy, with resampling and no post clip node.
pub type BuilderIdentityAntimeridianResampleNoClip<DRAIN, T> = Builder<DRAIN, NoPCNU, T>;

/// A simplified builder with a antimeridian clipping stratergy, with resampling and post clip node.
pub type BuilderIdentityAntimeridianResampleClip<DRAIN, T> = Builder<DRAIN, PCNU<T>, T>;

/// A simplified builder with a circle clipping stratergy, with resampling and no post clip node.
pub type BuilderIdentityCircleResampleNoClip<DRAIN, T> = Builder<DRAIN, NoPCNU, T>;

/// A simplified builder with a circle clipping stratergy, with no resampling and no post clip node.
pub type BuilderIdentityCircleResampleNoneNoClip<DRAIN, T> = Builder<DRAIN, NoPCNU, T>;

/// A simplified builder with a circle clipping stratergy, with resampling and a post clip node.
pub type BuilderIdentityCircleResampleClip<DRAIN, T> = Builder<DRAIN, PCNU<T>, T>;

/// A simplified builder with a circle clipping stratergy, with no resampling and a post clip node.
pub type BuilderIdentityCircleResampleNoneClip<DRAIN, T> = Builder<DRAIN, PCNU<T>, T>;
