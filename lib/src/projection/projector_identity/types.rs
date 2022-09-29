use crate::projection::builder::template::NoPCNC;
use crate::projection::builder::template::NoPCNU;
use crate::projection::builder::template::PCNC;
use crate::projection::builder::template::PCNU;

use super::Projector;

pub type ProjectorIdentityAntimeridianResampleNoneNoClip<DRAIN, T> =
    Projector<DRAIN, NoPCNC<DRAIN>, NoPCNU<DRAIN>, T>;

pub type ProjectorIdentityAntimeridianResampleNoneClip<DRAIN, T> =
    Projector<DRAIN, PCNC<DRAIN, T>, PCNU<DRAIN, T>, T>;

pub type ProjectorIdentityAntimeridianResampleNoClip<DRAIN, T> =
    Projector<DRAIN, NoPCNC<DRAIN>, NoPCNU<DRAIN>, T>;

pub type ProjectorIdentityAntimeridianResampleClip<DRAIN, T> =
    Projector<DRAIN, PCNC<DRAIN, T>, PCNU<DRAIN, T>, T>;

pub type ProjectorIdentityCircleResampleNoneNoClip<DRAIN, T> =
    Projector<DRAIN, NoPCNU<DRAIN>, NoPCNC<DRAIN>, T>;

pub type ProjectorIdentityCircleResampleNoneClip<DRAIN, T> =
    Projector<DRAIN, PCNC<DRAIN, T>, PCNU<DRAIN, T>, T>;

pub type ProjectorIdentityCircleResampleNoClip<DRAIN, T> =
    Projector<DRAIN, NoPCNC<DRAIN>, NoPCNU<DRAIN>, T>;

pub type ProjectorIdentityCircleResampleClip<DRAIN, T> =
    Projector<DRAIN, PCNC<DRAIN, T>, PCNU<DRAIN, T>, T>;
