use crate::projection::builder::template::NoPCNU;

use crate::projection::builder::template::PCNU;

use super::Builder;

pub type BuilderIdentityAntimeridianResampleNoneNoClip<DRAIN, T> = Builder<DRAIN, NoPCNU<DRAIN>, T>;

pub type BuilderIdentityAntimeridianResampleNoneClip<DRAIN, T> = Builder<DRAIN, PCNU<DRAIN, T>, T>;

pub type BuilderIdentityAntimeridianResampleNoClip<DRAIN, T> = Builder<DRAIN, NoPCNU<DRAIN>, T>;

pub type BuilderIdentityAntimeridianResampleClip<DRAIN, T> = Builder<DRAIN, PCNU<DRAIN, T>, T>;

pub type BuilderIdentityCircleResampleNoClip<DRAIN, T> = Builder<DRAIN, NoPCNU<DRAIN>, T>;

pub type BuilderIdentityCircleResampleNoneNoClip<DRAIN, T> = Builder<DRAIN, NoPCNU<DRAIN>, T>;

pub type BuilderIdentityCircleResampleClip<DRAIN, T> = Builder<DRAIN, PCNU<DRAIN, T>, T>;

pub type BuilderIdentityCircleResampleNoneClip<DRAIN, T> = Builder<DRAIN, PCNU<DRAIN, T>, T>;
