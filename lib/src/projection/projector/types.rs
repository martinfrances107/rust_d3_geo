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
use crate::projection::Projector;

pub type ProjectorAntimeridianResampleNoneNoClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianC<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU<DRAIN>,
    PR,
    ResampleNoneNoPCNC<DRAIN, PR, T>,
    ResampleNoneNoPCNU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorAntimeridianResampleNoneClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianC<ResampleNonePCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<DRAIN, T>,
    PR,
    ResampleNonePCNC<DRAIN, PR, T>,
    ResampleNonePCNU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorAntimeridianResampleNoClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianC<ResamplePCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU<DRAIN>,
    PR,
    ResampleNoPCNC<DRAIN, PR, T>,
    ResampleNoPCNU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorAntimeridianResampleClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianC<ResamplePCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<DRAIN, T>,
    PR,
    ResamplePCNC<DRAIN, PR, T>,
    ResamplePCNU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorCircleResampleNoneNoClip<DRAIN, PR, T> = Projector<
    ClipCircleC<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU<DRAIN>,
    PR,
    ResampleNoneNoPCNC<DRAIN, PR, T>,
    ResampleNoneNoPCNU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorCircleResampleNoneClip<DRAIN, PR, T> = Projector<
    ClipCircleC<ResampleNonePCNC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<DRAIN, T>,
    PR,
    ResampleNonePCNC<DRAIN, PR, T>,
    ResampleNonePCNU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorCircleResampleNoClip<DRAIN, PR, T> = Projector<
    ClipCircleC<ResampleNoPCNC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU<DRAIN>,
    PR,
    ResampleNoPCNC<DRAIN, PR, T>,
    ResampleNoPCNU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorCircleResampleClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianC<ResamplePCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<DRAIN, T>,
    PR,
    ResamplePCNC<DRAIN, PR, T>,
    ResamplePCNU<DRAIN, PR, T>,
    T,
>;
