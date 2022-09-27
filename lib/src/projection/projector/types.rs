use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::antimeridian::ClipAntimeridianU;
use crate::clip::circle::ClipCircleC;
use crate::clip::circle::ClipCircleU;
use crate::projection::builder::template::ClipU;
use crate::projection::builder::template::NoClipU;
use crate::projection::builder::template::ResampleClipC;
use crate::projection::builder::template::ResampleClipU;
use crate::projection::builder::template::ResampleNoClipC;
use crate::projection::builder::template::ResampleNoClipU;
use crate::projection::builder::template::ResampleNoneClipC;
use crate::projection::builder::template::ResampleNoneClipU;
use crate::projection::builder::template::ResampleNoneNoClipC;
use crate::projection::builder::template::ResampleNoneNoClipU;
use crate::projection::Projector;

pub type ProjectorAntimeridianResampleNoneNoClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianC<ResampleNoneNoClipC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNoneNoClipC<DRAIN, PR, T>, T>,
    DRAIN,
    NoClipU<DRAIN>,
    PR,
    ResampleNoneNoClipC<DRAIN, PR, T>,
    ResampleNoneNoClipU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorAntimeridianResampleNoneClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianC<ResampleNoneClipC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleNoneClipC<DRAIN, PR, T>, T>,
    DRAIN,
    ClipU<DRAIN, T>,
    PR,
    ResampleNoneClipC<DRAIN, PR, T>,
    ResampleNoneClipU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorAntimeridianResampleNoClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianC<ResampleClipC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleClipC<DRAIN, PR, T>, T>,
    DRAIN,
    NoClipU<DRAIN>,
    PR,
    ResampleNoClipC<DRAIN, PR, T>,
    ResampleNoClipU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorAntimeridianResampleClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianC<ResampleClipC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleClipC<DRAIN, PR, T>, T>,
    DRAIN,
    ClipU<DRAIN, T>,
    PR,
    ResampleClipC<DRAIN, PR, T>,
    ResampleClipU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorCircleResampleNoneNoClip<DRAIN, PR, T> = Projector<
    ClipCircleC<ResampleNoneNoClipC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNoneNoClipC<DRAIN, PR, T>, T>,
    DRAIN,
    NoClipU<DRAIN>,
    PR,
    ResampleNoneNoClipC<DRAIN, PR, T>,
    ResampleNoneNoClipU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorCircleResampleNoneClip<DRAIN, PR, T> = Projector<
    ClipCircleC<ResampleNoneClipC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNoneClipC<DRAIN, PR, T>, T>,
    DRAIN,
    ClipU<DRAIN, T>,
    PR,
    ResampleNoneClipC<DRAIN, PR, T>,
    ResampleNoneClipU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorCircleResampleNoClip<DRAIN, PR, T> = Projector<
    ClipCircleC<ResampleNoClipC<DRAIN, PR, T>, T>,
    ClipCircleU<ResampleNoClipC<DRAIN, PR, T>, T>,
    DRAIN,
    NoClipU<DRAIN>,
    PR,
    ResampleNoClipC<DRAIN, PR, T>,
    ResampleNoClipU<DRAIN, PR, T>,
    T,
>;

pub type ProjectorCircleResampleClip<DRAIN, PR, T> = Projector<
    ClipAntimeridianC<ResampleClipC<DRAIN, PR, T>, T>,
    ClipAntimeridianU<ResampleClipC<DRAIN, PR, T>, T>,
    DRAIN,
    ClipU<DRAIN, T>,
    PR,
    ResampleClipC<DRAIN, PR, T>,
    ResampleClipU<DRAIN, PR, T>,
    T,
>;
