use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coord;

use crate::stream::Connectable;
use crate::stream::Unconnected;
use crate::Transform;

use self::multiplex::AlbersTransformer;
use self::multiplex::Multiplex;
use self::multitransformer::MultiTransformer;

use super::Projector as ProjectorTrait;

/// The multiplex is a collection of sub-projections.
pub mod multiplex;
/// Transformer defined as more than more proejction.
pub mod multitransformer;
/// Builder shorthand notations.
pub mod types;

/// Projection output of projection/Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<DRAIN, MULTIPLEX> {
    phantom_drain: PhantomData<DRAIN>,
    /// The internals stages of the pipeline
    pub multiplex: MULTIPLEX,
}

impl<DRAIN, MULTIPLEX> Default for Projector<DRAIN, MULTIPLEX>
where
    DRAIN: Clone,
    MULTIPLEX: Default,
{
    fn default() -> Self {
        Self {
            phantom_drain: PhantomData::<DRAIN>,
            multiplex: MULTIPLEX::default(),
        }
    }
}

impl<DRAIN> ProjectorTrait for Projector<DRAIN, Multiplex<Unconnected>>
where
    DRAIN: Clone,
{
    type EP = DRAIN;

    type Transformer = MultiTransformer<DRAIN, f64, AlbersTransformer<DRAIN>>;
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
