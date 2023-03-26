use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coord;

use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::Transform;

use self::multiplex::Multiplex;

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
pub struct Projector<DRAIN, MULTIPLEX> {
    phantom_drain: PhantomData<DRAIN>,
    pub(crate) multiplex: MULTIPLEX,
}

impl<DRAIN, MULTIPLEX> Default for Projector<DRAIN, MULTIPLEX>
where
    DRAIN: Clone + PartialEq,
    MULTIPLEX: Default,
    // MULTIPLEX: Clone + Connectable,
{
    fn default() -> Self {
        Self {
            phantom_drain: PhantomData::<DRAIN>,
            multiplex: MULTIPLEX::default(),
        }
    }
}

impl<DRAIN, MULTIPLEX> ProjectorTrait for Projector<DRAIN, MULTIPLEX>
where
    DRAIN: Clone + PartialEq,
    MULTIPLEX: Clone + Connectable,
{
    type DRAIN = DRAIN;

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

impl<DRAIN, MULTIPLEX> Transform for Projector<DRAIN, MULTIPLEX>
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
