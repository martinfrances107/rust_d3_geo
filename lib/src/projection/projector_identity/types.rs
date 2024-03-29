use crate::clip::rectangle::Rectangle;
use crate::identity::Identity;
use crate::projection::builder::template::NoPCNC;
use crate::projection::builder::template::PCNC;
use crate::stream::Unconnected;

use super::Projector;

/// A simplified projector with a antimeridian clipping strategy, no resampling and no post clip node.
pub type ProjectorIdentityAntimeridianResampleNoneNoClip<DRAIN, T> =
    Projector<DRAIN, NoPCNC<DRAIN>, Identity<Unconnected>, T>;

/// A simplified projector with a antimeridian clipping strategy, no resampling and a post clip node.
pub type ProjectorIdentityAntimeridianResampleNoneClip<DRAIN, T> =
    Projector<DRAIN, PCNC<DRAIN, T>, Rectangle<Unconnected, T>, T>;

/// A simplified projector with a antimeridian clipping strategy, resampling and no post clip node.
pub type ProjectorIdentityAntimeridianResampleNoClip<DRAIN, T> =
    Projector<DRAIN, NoPCNC<DRAIN>, Identity<Unconnected>, T>;

/// A simplified projector with a antimeridian clipping strategy, resampling and a post clip node.
pub type ProjectorIdentityAntimeridianResampleClip<DRAIN, T> =
    Projector<DRAIN, PCNC<DRAIN, T>, Rectangle<Unconnected, T>, T>;

/// A simplified projector with a circle clipping strategy, no resampling and no post clip node.
pub type ProjectorIdentityCircleResampleNoneNoClip<DRAIN, T> =
    Projector<DRAIN, Identity<Unconnected>, NoPCNC<DRAIN>, T>;

/// A simplified projector with a circle clipping strategy, no resampling and a post clip node.
pub type ProjectorIdentityCircleResampleNoneClip<DRAIN, T> =
    Projector<DRAIN, PCNC<DRAIN, T>, Rectangle<Unconnected, T>, T>;

/// A simplified projector with a circle clipping strategy, resampling and no post clip node.
pub type ProjectorIdentityCircleResampleNoClip<DRAIN, T> =
    Projector<DRAIN, NoPCNC<DRAIN>, Identity<Unconnected>, T>;

/// A simplified projector with a circle clipping strategy, resampling and a post clip node.
pub type ProjectorIdentityCircleResampleClip<DRAIN, T> =
    Projector<DRAIN, PCNC<DRAIN, T>, Rectangle<Unconnected, T>, T>;
