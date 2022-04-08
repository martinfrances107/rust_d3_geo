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

pub type NoClipC<DRAIN, T> = Identity<DRAIN, DRAIN, Connected<DRAIN>, T>;
pub type NoClipU<DRAIN, T> = Identity<DRAIN, DRAIN, Unconnected, T>;

pub type ClipC<DRAIN, T> = Rectangle<DRAIN, DRAIN, Connected<DRAIN>, T>;
pub type ClipU<DRAIN, T> = Rectangle<DRAIN, DRAIN, Unconnected, T>;

pub type ResampleClipC<DRAIN, PR, T> =
    Resample<DRAIN, PR, ClipC<DRAIN, T>, ClipU<DRAIN, T>, ConnectedResample<ClipC<DRAIN, T>, T>, T>;

pub type ResampleClipU<DRAIN, PR, T> =
    Resample<DRAIN, PR, ClipC<DRAIN, T>, ClipU<DRAIN, T>, Unconnected, T>;

// ----
pub type ResampleNoClipC<DRAIN, PR, T> = Resample<
    DRAIN,
    PR,
    NoClipC<DRAIN, T>,
    NoClipU<DRAIN, T>,
    ConnectedResample<NoClipC<DRAIN, T>, T>,
    T,
>;

pub type ResampleNoClipU<DRAIN, PR, T> =
    Resample<DRAIN, PR, NoClipC<DRAIN, T>, NoClipU<DRAIN, T>, Unconnected, T>;

/// ------------

pub type ResampleNoneClipC<DRAIN, PR, T> =
    None<DRAIN, PR, ClipC<DRAIN, T>, ClipU<DRAIN, T>, Connected<ClipC<DRAIN, T>>, T>;

pub type ResampleNoneClipU<DRAIN, PR, T> =
    None<DRAIN, PR, ClipC<DRAIN, T>, ClipU<DRAIN, T>, Unconnected, T>;

/// ----

pub type ResampleNoneNoClipC<DRAIN, PR, T> =
    None<DRAIN, PR, NoClipC<DRAIN, T>, NoClipU<DRAIN, T>, Connected<NoClipC<DRAIN, T>>, T>;

pub type ResampleNoneNoClipU<DRAIN, PR, T> =
    None<DRAIN, PR, NoClipC<DRAIN, T>, NoClipU<DRAIN, T>, Unconnected, T>;

// Default
// No resampling,
// No Clipping.
pub type Default<DRAIN, I, LB, LC, LU, PR, PV, T> = Builder<
    DRAIN,
    I,
    LB,
    LC,
    LU,
    NoClipC<DRAIN, T>,
    NoClipU<DRAIN, T>,
    PR,
    PV,
    ResampleNoneNoClipC<DRAIN, PR, T>,
    ResampleNoneNoClipU<DRAIN, PR, T>,
    T,
>;
