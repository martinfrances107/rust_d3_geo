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

pub type NoPCNC<DRAIN> = Identity<DRAIN, Connected<DRAIN>>;
pub type NoPCNU<DRAIN> = Identity<DRAIN, Unconnected>;

pub type PCNC<DRAIN, T> = Rectangle<DRAIN, Connected<DRAIN>, T>;
pub type PCNU<DRAIN, T> = Rectangle<DRAIN, Unconnected, T>;

pub type ResamplePCNC<DRAIN, PR, T> =
    Resample<PR, PCNC<DRAIN, T>, ConnectedResample<PCNC<DRAIN, T>, T>, T>;

pub type ResamplePCNU<DRAIN, PR, T> = Resample<PR, PCNC<DRAIN, T>, Unconnected, T>;

// ----
pub type ResampleNoPCNC<DRAIN, PR, T> =
    Resample<PR, NoPCNC<DRAIN>, ConnectedResample<NoPCNC<DRAIN>, T>, T>;

pub type ResampleNoPCNU<DRAIN, PR, T> = Resample<PR, NoPCNC<DRAIN>, Unconnected, T>;

/// ------------

pub type ResampleNonePCNC<DRAIN, PR, T> = None<PR, PCNC<DRAIN, T>, Connected<PCNC<DRAIN, T>>, T>;

pub type ResampleNonePCNU<DRAIN, PR, T> = None<PR, PCNC<DRAIN, T>, Unconnected, T>;

/// ----

pub type ResampleNoneNoPCNC<DRAIN, PR, T> = None<PR, NoPCNC<DRAIN>, Connected<NoPCNC<DRAIN>>, T>;

pub type ResampleNoneNoPCNU<DRAIN, PR, T> = None<PR, NoPCNC<DRAIN>, Unconnected, T>;

// Default
// No resampling,
// No Clipping.
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
