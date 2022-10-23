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
pub type NoPCNC<DRAIN> = Identity<DRAIN, Connected<DRAIN>>;
/// A unconnected pass through post clip node.
pub type NoPCNU<DRAIN> = Identity<DRAIN, Unconnected>;

/// A connected post clip node.
pub type PCNC<DRAIN, T> = Rectangle<DRAIN, Connected<DRAIN>, T>;
/// A unconnected post clip node.
pub type PCNU<DRAIN, T> = Rectangle<DRAIN, Unconnected, T>;

/// A connected resample node connected to a post clip node.
pub type ResamplePCNC<DRAIN, PR, T> =
    Resample<PR, PCNC<DRAIN, T>, ConnectedResample<PCNC<DRAIN, T>, T>, T>;

/// A unconnected resample node connected to a post clip node.
pub type ResamplePCNU<DRAIN, PR, T> = Resample<PR, PCNC<DRAIN, T>, Unconnected, T>;

/// A connected resample node, connected to pass through clip node.
pub type ResampleNoPCNC<DRAIN, PR, T> =
    Resample<PR, NoPCNC<DRAIN>, ConnectedResample<NoPCNC<DRAIN>, T>, T>;
/// A unconnected resample node, connected to pass through clip node.
pub type ResampleNoPCNU<DRAIN, PR, T> = Resample<PR, NoPCNC<DRAIN>, Unconnected, T>;

/// A connected resample pass through node, connected to a post clip node.
pub type ResampleNonePCNC<DRAIN, PR, T> = None<PR, PCNC<DRAIN, T>, Connected<PCNC<DRAIN, T>>, T>;

/// A unconnected resample pass through node, connected to a post clip node.
pub type ResampleNonePCNU<DRAIN, PR, T> = None<PR, PCNC<DRAIN, T>, Unconnected, T>;

/// A connected resample pass through node, connected to a pass through post clip node.
pub type ResampleNoneNoPCNC<DRAIN, PR, T> = None<PR, NoPCNC<DRAIN>, Connected<NoPCNC<DRAIN>>, T>;

/// A unconnected resample pass through node, connected to a pass through post clip node.
pub type ResampleNoneNoPCNU<DRAIN, PR, T> = None<PR, NoPCNC<DRAIN>, Unconnected, T>;

/// Default builder, no resampling, no Clipping.
pub type Default<CLIPC, CLIPU, DRAIN, PR, T> = Builder<
    CLIPC,
    CLIPU,
    DRAIN,
    NoPCNU<DRAIN>,
    PR,
    ResampleNoneNoPCNC<DRAIN, PR, T>,
    ResampleNoneNoPCNU<DRAIN, PR, T>,
    T,
>;
