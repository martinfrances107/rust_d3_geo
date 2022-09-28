use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::antimeridian::ClipAntimeridianU;

use crate::clip::circle::ClipCircleC;
use crate::clip::circle::ClipCircleU;
use crate::projection::builder::template::ResampleNoneNoPCNC;
use crate::projection::builder::template::ResampleNoneNoPCNU;
use crate::projection::builder::template::ResampleNonePCNC;
use crate::projection::builder::template::ResampleNonePCNU;
use crate::projection::builder::NoPCNU;
use crate::projection::builder::ResampleNoPCNC;
use crate::projection::builder::ResampleNoPCNU;
use crate::projection::builder::ResamplePCNC;
use crate::projection::builder::ResamplePCNU;
use crate::projection::builder::PCNU;

use super::Builder;

pub type BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU<DRAIN>,
    PR,
    ResampleNoneNoPCNC<DRAIN, PR, T>,
    ResampleNoneNoPCNU<DRAIN, PR, T>,
    T,
>;

pub type BuilderAntimeridianResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleNonePCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<DRAIN, T>,
    PR,
    ResampleNonePCNC<DRAIN, PR, T>,
    ResampleNonePCNU<DRAIN, PR, T>,
    T,
>;

pub type BuilderAntimeridianResampleNoClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleNoPCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU<DRAIN>,
    PR,
    ResampleNoPCNC<DRAIN, PR, T>,
    ResampleNoPCNU<DRAIN, PR, T>,
    T,
>;

pub type BuilderAntimeridianResampleClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResamplePCNC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<DRAIN, T>,
    PR,
    ResamplePCNC<DRAIN, PR, T>,
    ResamplePCNU<DRAIN, PR, T>,
    T,
>;

pub type BuilderCircleResampleNoClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResampleNoPCNC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU<DRAIN>,
    PR,
    ResampleNoPCNC<DRAIN, PR, T>,
    ResampleNoPCNU<DRAIN, PR, T>,
    T,
>;

pub type BuilderCircleResampleNoneNoClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNoneNoPCNC<DRAIN, PR, T>, T>,
    DRAIN,
    NoPCNU<DRAIN>,
    PR,
    ResampleNoneNoPCNC<DRAIN, PR, T>,
    ResampleNoneNoPCNU<DRAIN, PR, T>,
    T,
>;

pub type BuilderCircleResampleClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResamplePCNC<DRAIN, PR, T>, T>,
    ClipCircleU<ResamplePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<DRAIN, T>,
    PR,
    ResamplePCNC<DRAIN, PR, T>,
    ResamplePCNU<DRAIN, PR, T>,
    T,
>;

pub type BuilderCircleResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResampleNonePCNC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNonePCNC<DRAIN, PR, T>, T>,
    DRAIN,
    PCNU<DRAIN, T>,
    PR,
    ResampleNonePCNC<DRAIN, PR, T>,
    ResampleNonePCNU<DRAIN, PR, T>,
    T,
>;
