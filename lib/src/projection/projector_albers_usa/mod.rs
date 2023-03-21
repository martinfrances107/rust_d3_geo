use std::fmt::Debug;
use std::marker::PhantomData;

use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;

use super::stream_transform_radians::StreamTransformRadians;
use super::Projector as ProjectorTrait;

pub mod multiplex;
/// Builder shorthand notations.
pub mod types;

type CacheState<CLIP, DRAIN, T> = Option<(
    DRAIN,
    StreamTransformRadians<Connected<RotatorRadians<Connected<CLIP>, T>>>,
)>;

/// Projection output of projection/Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<CC, DRAIN, MULTIPLEX>
where
    DRAIN: Clone,
{
    phantom_cc: PhantomData<CC>,
    phantom_drain: PhantomData<DRAIN>,
    pub(crate) multiplex: MULTIPLEX,
}

type ProjectionStream<CLIP, T> =
    StreamTransformRadians<Connected<RotatorRadians<Connected<CLIP>, T>>>;

impl<CC, DRAIN, MULTIPLEX> ProjectorTrait for Projector<CC, DRAIN, MULTIPLEX>
where
    CC: Clone + Stream<EP = DRAIN, T = f64>,
    DRAIN: Clone + PartialEq,
    MULTIPLEX: Clone + Connectable,
    // PCNC: Clone,
    // RU: Clone + Connectable<Output<PCNC> = RC>,
    // RC: Clone,
{
    type DRAIN = DRAIN;
    // type Transformer = ProjectionStream<CC, f64>;
    type Transformer = <MULTIPLEX as Connectable>::Output<DRAIN>;
    /// Connects a DRAIN to the projection.
    ///
    /// The Projection Stream Pipeline :-
    ///
    /// `StreamTransformRadians` -> `StreamTransform` -> `Preclip` -> `Resample` -> `Postclip` -> `DRAIN`
    ///
    fn stream(&mut self, drain: &DRAIN) -> Self::Transformer {
        self.multiplex.clone().connect(drain.clone())
    }
}

// impl<CLIPC, DRAIN, PCNU, PR, RC, RU, T> Transform for Projector<CLIPC, DRAIN, PR, RC, RU, T>
// where
//     CLIPC: Clone,

//     PR: Transform<T = T>,
//     T: CoordFloat + FloatConst,
// {
//     /// f32 or f64
//     type T = T;

//     fn transform(&self, p: &Coord<T>) -> Coord<T> {
//         todo!();
//         // let r = Coord {
//         //     x: p.x.to_radians(),
//         //     y: p.y.to_radians(),
//         // };
//         // self.project_rotate_transform.transform(&r)
//     }
//     fn invert(&self, p: &Coord<T>) -> Coord<T> {
//         todo!();
//         // let d = self.project_rotate_transform.invert(p);
//         // Coord {
//         //     x: d.x.to_degrees(),
//         //     y: d.y.to_degrees(),
//         // }
//     }
// }
