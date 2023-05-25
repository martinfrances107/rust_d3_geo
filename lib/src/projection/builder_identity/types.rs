use crate::identity::Identity;
use crate::projection::builder::template::PCNU;
use crate::stream::Unconnected;

use super::Builder;

/// A simplified builder with a antimeridian clipping stratergy, with resampling and no post clip node.
pub type BuilderIdentityAntimeridianResampleNoneNoClip<T> = Builder<Identity<Unconnected>, T>;

/// A simplified builder with a antimeridian clipping stratergy, with no resampling and post clip node.
pub type BuilderIdentityAntimeridianResampleNoneClip<T> = Builder<PCNU<T>, T>;

/// A simplified builder with a antimeridian clipping stratergy, with resampling and no post clip node.
pub type BuilderIdentityAntimeridianResampleNoClip<T> = Builder<Identity<Unconnected>, T>;

/// A simplified builder with a antimeridian clipping stratergy, with resampling and post clip node.
pub type BuilderIdentityAntimeridianResampleClip<T> = Builder<PCNU<T>, T>;

/// A simplified builder with a circle clipping stratergy, with resampling and no post clip node.
pub type BuilderIdentityCircleResampleNoClip<T> = Builder<Identity<Unconnected>, T>;

/// A simplified builder with a circle clipping stratergy, with no resampling and no post clip node.
pub type BuilderIdentityCircleResampleNoneNoClip<T> = Builder<Identity<Unconnected>, T>;

/// A simplified builder with a circle clipping stratergy, with resampling and a post clip node.
pub type BuilderIdentityCircleResampleClip<T> = Builder<PCNU<T>, T>;

/// A simplified builder with a circle clipping stratergy, with no resampling and a post clip node.
pub type BuilderIdentityCircleResampleNoneClip<T> = Builder<PCNU<T>, T>;
