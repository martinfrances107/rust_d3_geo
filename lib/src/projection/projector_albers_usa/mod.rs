use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::Transform;

use super::stream_transform_radians::StreamTransformRadians;
use super::Projector as ProjectorTrait;

/// The multiplex is a collection of sub-projections.
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

impl<CC, DRAIN, MULTIPLEX> Projector<CC, DRAIN, MULTIPLEX>
where
    DRAIN: Clone,
{
    pub fn new(multiplex: MULTIPLEX) -> Self {
        Self {
            phantom_cc: PhantomData::<CC>,
            phantom_drain: PhantomData::<DRAIN>,
            multiplex,
        }
    }
}

impl<CC, DRAIN, MULTIPLEX> ProjectorTrait for Projector<CC, DRAIN, MULTIPLEX>
where
    CC: Clone + Stream<EP = DRAIN, T = f64>,
    DRAIN: Clone + PartialEq,
    MULTIPLEX: Clone + Connectable,
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

impl<CC, DRAIN, MULTIPLEX> Transform for Projector<CC, DRAIN, MULTIPLEX>
where
    DRAIN: Clone,
    MULTIPLEX: Transform<T = f64>,
{
    /// f32 or f64
    type T = f64;

    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
        self.multiplex.transform(p)
    }
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        self.multiplex.invert(p)
    }
}
