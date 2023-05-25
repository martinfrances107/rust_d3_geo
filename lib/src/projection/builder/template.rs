//! List of types need to construct the default Projection.
//!
//! Default is a build template :-
//! Has no resampling,
//! Has no clip bounds.
//!

use crate::clip::rectangle::Rectangle;
use crate::identity::Identity;
use crate::projection::resampler::none::None;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::stream::Connected;
use crate::stream::Unconnected;

use super::Builder;

/// A connected pass through post clip node.
pub type NoPCNC<DRAIN> = Identity<Connected<DRAIN>>;
/// A unconnected pass through post clip node.
type NoPCNU = Identity<Unconnected>;

/// A connected post clip node.
pub type PCNC<DRAIN, T> = Rectangle<Connected<DRAIN>, T>;
/// A unconnected post clip node.
pub type PCNU<T> = Rectangle<Unconnected, T>;

/// A connected resample node connected to a post clip node.
pub type ResamplePCNC<DRAIN, PR, T> = Resample<PR, ConnectedResample<PCNC<DRAIN, T>, T>, T>;

/// A unconnected resample node connected to a post clip node.
pub type ResamplePCNU<PR, T> = Resample<PR, Unconnected, T>;

/// A connected resample node, connected to pass through clip node.
pub type ResampleNoPCNC<DRAIN, PR, T> = Resample<PR, ConnectedResample<NoPCNC<DRAIN>, T>, T>;
/// A unconnected resample node, connected to pass through clip node.
pub type ResampleNoPCNU<PR, T> = Resample<PR, Unconnected, T>;

/// A connected resample pass through node, connected to a post clip node.
pub type ResampleNonePCNC<DRAIN, PR, T> = None<PR, Connected<PCNC<DRAIN, T>>, T>;

/// A unconnected resample pass through node, connected to a post clip node.
pub type ResampleNonePCNU<PR, T> = None<PR, Unconnected, T>;

/// A connected resample pass through node, connected to a pass through post clip node.
pub type ResampleNoneNoPCNC<DRAIN, PR, T> = None<PR, Connected<NoPCNC<DRAIN>>, T>;

/// A unconnected resample pass through node, connected to a pass through post clip node.
pub type ResampleNoneNoPCNU<PR, T> = None<PR, Unconnected, T>;

/// Default projection builder, no resampling, no Clipping.
pub type Default<CLIPU, DRAIN, PR, T> =
    Builder<CLIPU, DRAIN, NoPCNU, PR, ResampleNoneNoPCNU<PR, T>, T>;
