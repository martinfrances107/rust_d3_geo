use crate::projection::builder::template::NoPCNC;
use crate::projection::builder::template::NoPCNU;
use crate::projection::builder::template::PCNC;
use crate::projection::builder::template::PCNU;

use super::Projector;

/// A simplified projector with a antimeridian clipping stratergy, no resampling and no post clip node.
pub type ProjectorIdentityAntimeridianResampleNoneNoClip<DRAIN, T> =
    Projector<DRAIN, NoPCNC<DRAIN>, NoPCNU, T>;

/// A simplified projector with a antimeridian clipping stratergy, no resampling and a post clip node.
pub type ProjectorIdentityAntimeridianResampleNoneClip<DRAIN, T> =
    Projector<DRAIN, PCNC<DRAIN, T>, PCNU<T>, T>;

/// A simplified projector with a antimeridian clipping stratergy, resampling and no post clip node.
pub type ProjectorIdentityAntimeridianResampleNoClip<DRAIN, T> =
    Projector<DRAIN, NoPCNC<DRAIN>, NoPCNU, T>;

/// A simplified projector with a antimeridian clipping stratergy, resampling and a post clip node.
pub type ProjectorIdentityAntimeridianResampleClip<DRAIN, T> =
    Projector<DRAIN, PCNC<DRAIN, T>, PCNU<T>, T>;

/// A simplified projector with a circle clipping stratergy, no resampling and no post clip node.
pub type ProjectorIdentityCircleResampleNoneNoClip<DRAIN, T> =
    Projector<DRAIN, NoPCNU, NoPCNC<DRAIN>, T>;

/// A simplified projector with a circle clipping stratergy, no resampling and a post clip node.
pub type ProjectorIdentityCircleResampleNoneClip<DRAIN, T> =
    Projector<DRAIN, PCNC<DRAIN, T>, PCNU<T>, T>;

/// A simplified projector with a circle clipping stratergy, resampling and no post clip node.
pub type ProjectorIdentityCircleResampleNoClip<DRAIN, T> =
    Projector<DRAIN, NoPCNC<DRAIN>, NoPCNU, T>;

/// A simplified projector with a circle clipping stratergy, resampling and a post clip node.
pub type ProjectorIdentityCircleResampleClip<DRAIN, T> =
    Projector<DRAIN, PCNC<DRAIN, T>, PCNU<T>, T>;
