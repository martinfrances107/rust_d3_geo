use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleC;
use crate::clip::circle::ClipCircleU;
use crate::projection::builder::template::NoPCNU;
use crate::projection::builder::template::ResampleNoPCNC;
use crate::projection::builder::template::ResampleNoPCNU;
use crate::projection::builder::template::ResampleNoneNoPCNC;
use crate::projection::builder::template::ResampleNoneNoPCNU;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::template::ResamplePCNC;
use crate::projection::builder::template::ResamplePCNU;
use crate::projection::builder::template::PCNU;

use super::Projector;
use super::Source;

/// A projector with a antimeridian clipping stratergy, with resampling and no post clip node.
pub type ProjectorAntimeridianResampleNoneNoClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianU<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU,
    PR,
    ResampleNoneNoPCNU<PR, T>,
    Source<ClipAntimeridianC<ResampleNoneNoPCNC<DRAIN, PR, T>, T>, T>,
    T,
>;

/// A projector with a antimeridian clipping stratergy, with no resampling and a post clip node.
pub type ProjectorAntimeridianResampleNoneClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<T>,
    PR,
    ResampleNonePCNU<PR, T>,
    Source<ClipAntimeridianC<ResampleNonePCNC<DRAIN, PR, T>, T>, T>,
    T,
>;

/// A projector with a antimeridian clipping stratergy, with resampling and no post clip node.
pub type ProjectorAntimeridianResampleNoClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianU<ResampleNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU,
    PR,
    ResampleNoPCNU<PR, T>,
    Source<ClipAntimeridianC<ResampleNoPCNC<DRAIN, PR, T>, T>, T>,
    T,
>;

/// A projector with a antimeridian clipping stratergy, with resampling and a post clip node.
pub type ProjectorAntimeridianResampleClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<T>,
    PR,
    ResamplePCNU<PR, T>,
    Source<ClipAntimeridianC<ResamplePCNC<DRAIN, PR, T>, T>, T>,
    T,
>;

/// A projector with a circle clipping stratergy, with no resampling and no post clip node.
pub type ProjectorCircleResampleNoneNoClip<DRAIN, PR, T> = Projector<
    ClipCircleU<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU,
    PR,
    ResampleNoneNoPCNU<PR, T>,
    Source<ClipCircleC<ResampleNoneNoPCNC<DRAIN, PR, T>, T>, T>,
    T,
>;

/// A projector with a circle clipping stratergy, with no resampling and post clip node.
pub type ProjectorCircleResampleNoneClip<DRAIN, PR, T> = Projector<
    ClipCircleU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<T>,
    PR,
    ResampleNonePCNU<PR, T>,
    Source<ClipCircleC<ResampleNonePCNC<DRAIN, PR, T>, T>, T>,
    T,
>;

/// A projector with a circle clipping stratergy, with resampling and post clip node.
pub type ProjectorCircleResampleNoClip<DRAIN, PR, T> = Projector<
    ClipCircleU<ResampleNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU,
    PR,
    ResampleNoPCNU<PR, T>,
    Source<ClipCircleC<ResampleNoPCNC<DRAIN, PR, T>, T>, T>,
    T,
>;

/// A projector with a circle clipping stratergy, with resampling and post clip node.
pub type ProjectorCircleResampleClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<T>,
    PR,
    ResamplePCNU<PR, T>,
    Source<ClipAntimeridianC<ResamplePCNC<DRAIN, PR, T>, T>, T>,
    T,
>;
