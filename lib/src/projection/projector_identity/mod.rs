use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::Projector as ProjectorTrait;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use transformer::Transformer;

/// Unit tests.
pub mod tests;
/// A stream pipeline stage.
pub mod transformer;
/// Builder shorthand notations.
pub mod types;

type CacheState<DRAIN, PCNC> = Option<(DRAIN, PCNC)>;

/// Projection output of projection/Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<DRAIN, PCNC, PCNU, T>
where
    PCNC: Clone,
    T: CoordFloat,
{
    pub(crate) p_pcnc: PhantomData<PCNC>,
    // Must be public as there is a implicit copy.
    pub(crate) postclip: PCNU,
    pub(crate) transform: Transformer<DRAIN, Unconnected, T>,
    pub(crate) cache: CacheState<DRAIN, Transformer<DRAIN, Connected<PCNC>, T>>,
}

impl<DRAIN, PCNC, PCNU, T> ProjectorTrait for Projector<DRAIN, PCNC, PCNU, T>
where
    DRAIN: Clone + PartialEq,
    PCNC: Clone,
    PCNU: Clone + Connectable<Output<DRAIN> = PCNC>,
    T: CoordFloat + FloatConst,
{
    /// Connects a DRAIN to the projection.
    ///
    /// The Projection Stream Pipeline :-
    ///
    ///  Transformer -> postclip -> DRAIN
    ///

    type Transformer = Transformer<DRAIN, Connected<PCNC>, T>;

    type EP = DRAIN;

    fn stream(&mut self, drain: &DRAIN) -> Self::Transformer {
        if let Some((cache_drain, output)) = &self.cache {
            if *cache_drain == *drain {
                return (*output).clone();
            }
        }

        let pcn = self.postclip.connect(drain.clone());

        let out = self.transform.connect(pcn);

        // Populate cache.
        self.cache = Some((drain.clone(), out.clone()));

        // Output stage is a transform_radians node.
        out
    }
}

impl<DRAIN, PCNC, PCNU, T> Transform for Projector<DRAIN, PCNC, PCNU, T>
where
    PCNC: Clone,
    T: CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        self.transform.transform(p)
    }
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        self.transform.invert(p)
    }
}
