use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::antimeridian::ClipAntimeridianU;

use crate::clip::circle::ClipCircleC;
use crate::clip::circle::ClipCircleU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::builder::ClipU;
use crate::projection::builder::NoClipU;
use crate::projection::builder::ResampleClipC;
use crate::projection::builder::ResampleClipU;
use crate::projection::builder::ResampleNoClipC;
use crate::projection::builder::ResampleNoClipU;

use super::Builder;

pub type BuilderAntimeridianResampleNoneNoClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleNoneNoClipC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNoneNoClipC<DRAIN, PR, T>, T>,
    DRAIN,
    NoClipU<DRAIN>,
    PR,
    ResampleNoneNoClipC<DRAIN, PR, T>,
    ResampleNoneNoClipU<DRAIN, PR, T>,
    T,
>;

pub type BuilderAntimeridianResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleNoneClipC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNoneClipC<DRAIN, PR, T>, T>,
    DRAIN,
    ClipU<DRAIN, T>,
    PR,
    ResampleNoneClipC<DRAIN, PR, T>,
    ResampleNoneClipU<DRAIN, PR, T>,
    T,
>;

pub type BuilderAntimeridianResampleNoClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleNoClipC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNoClipC<DRAIN, PR, T>, T>,
    DRAIN,
    NoClipU<DRAIN>,
    PR,
    ResampleNoClipC<DRAIN, PR, T>,
    ResampleNoClipU<DRAIN, PR, T>,
    T,
>;

pub type BuilderAntimeridianResampleClip<DRAIN, PR, T> = Builder<
    ClipAntimeridianC<ResampleClipC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleClipC<DRAIN, PR, T>, T>,
    DRAIN,
    ClipU<DRAIN, T>,
    PR,
    ResampleClipC<DRAIN, PR, T>,
    ResampleClipU<DRAIN, PR, T>,
    T,
>;

pub type BuilderCircleResampleNoClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResampleNoClipC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNoClipC<DRAIN, PR, T>, T>,
    DRAIN,
    NoClipU<DRAIN>,
    PR,
    ResampleNoClipC<DRAIN, PR, T>,
    ResampleNoClipU<DRAIN, PR, T>,
    T,
>;

pub type BuilderCircleResampleNoneNoClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResampleNoneNoClipC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNoneNoClipC<DRAIN, PR, T>, T>,
    DRAIN,
    NoClipU<DRAIN>,
    PR,
    ResampleNoneNoClipC<DRAIN, PR, T>,
    ResampleNoneNoClipU<DRAIN, PR, T>,
    T,
>;

pub type BuilderCircleResampleClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResampleClipC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleClipC<DRAIN, PR, T>, T>,
    DRAIN,
    ClipU<DRAIN, T>,
    PR,
    ResampleClipC<DRAIN, PR, T>,
    ResampleClipU<DRAIN, PR, T>,
    T,
>;

pub type BuilderCircleResampleNoneClip<DRAIN, PR, T> = Builder<
    ClipCircleC<ResampleNoneClipC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNoneClipC<DRAIN, PR, T>, T>,
    DRAIN,
    ClipU<DRAIN, T>,
    PR,
    ResampleNoneClipC<DRAIN, PR, T>,
    ResampleNoneClipU<DRAIN, PR, T>,
    T,
>;
