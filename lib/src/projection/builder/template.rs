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

pub type NoClipC<DRAIN> = Identity<DRAIN, Connected<DRAIN>>;
pub type NoClipU<DRAIN> = Identity<DRAIN, Unconnected>;

pub type ClipC<DRAIN, T> = Rectangle<DRAIN, Connected<DRAIN>, T>;
pub type ClipU<DRAIN, T> = Rectangle<DRAIN, Unconnected, T>;

pub type ResampleClipC<DRAIN, PR, T> =
    Resample<PR, ClipC<DRAIN, T>, ConnectedResample<ClipC<DRAIN, T>, T>, T>;

pub type ResampleClipU<DRAIN, PR, T> = Resample<PR, ClipC<DRAIN, T>, Unconnected, T>;

// ----
pub type ResampleNoClipC<DRAIN, PR, T> =
    Resample<PR, NoClipC<DRAIN>, ConnectedResample<NoClipC<DRAIN>, T>, T>;

pub type ResampleNoClipU<DRAIN, PR, T> = Resample<PR, NoClipC<DRAIN>, Unconnected, T>;

/// ------------

pub type ResampleNoneClipC<DRAIN, PR, T> = None<PR, ClipC<DRAIN, T>, Connected<ClipC<DRAIN, T>>, T>;

pub type ResampleNoneClipU<DRAIN, PR, T> = None<PR, ClipC<DRAIN, T>, Unconnected, T>;

/// ----

pub type ResampleNoneNoClipC<DRAIN, PR, T> = None<PR, NoClipC<DRAIN>, Connected<NoClipC<DRAIN>>, T>;

pub type ResampleNoneNoClipU<DRAIN, PR, T> = None<PR, NoClipC<DRAIN>, Unconnected, T>;

// Default
// No resampling,
// No Clipping.
pub type Default<DRAIN, I, LB, LC, LU, PR, PV, T> = Builder<
    DRAIN,
    I,
    LB,
    LC,
    LU,
    NoClipU<DRAIN>,
    PR,
    PV,
    ResampleNoneNoClipC<DRAIN, PR, T>,
    ResampleNoneNoClipU<DRAIN, PR, T>,
    T,
>;
