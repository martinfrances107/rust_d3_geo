use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::stream::Connectable;
use crate::stream::Unconnected;
use crate::Transform;

// pub mod types;
mod tests;
pub mod transformer;

use transformer::Transformer;

type CacheState<DRAIN, PCNC> = Option<(DRAIN, PCNC)>;

/// Projection output of projection/Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<DRAIN, PCNC, PCNU, T>
where
    T: CoordFloat,
{
    // Must be public as there is a implicit copy.
    pub(crate) postclip: PCNU,
    pub(crate) transform: Transformer<PCNU, Unconnected, T>,
    pub(crate) cache: CacheState<DRAIN, PCNC>,
}

// type ProjectionStream<T> = StreamTransformRadians<
//     Connected<RotatorRadians<Connected<Clip<I, LC, LU, PV, RC, ConnectedClip<LB, LC, T>, T>>, T>>,
// >;

impl<DRAIN, PCNC, PCNU, T> Projector<DRAIN, PCNC, PCNU, T>
where
    DRAIN: Clone + PartialEq,
    PCNC: Clone,
    PCNU: Clone + Connectable<SC = DRAIN, Output = PCNC>,
    // PCNU: Clone,
    T: CoordFloat,
{
    /// Connects a DRAIN to the projection.
    ///
    /// The Projection Stream Pipeline :-
    ///
    /// StreamTransformRadians -> StreamTransform -> preclip -> resample -> postclip -> DRAIN
    ///
    pub fn stream(&mut self, drain: &DRAIN) -> PCNC {
        if let Some((cache_drain, output)) = &self.cache {
            if *cache_drain == *drain {
                return (*output).clone();
            }
        }

        // let transformer = self.transform.connect(drain.clone());

        // Build cache.
        // let out = self.postclip.clone().connect(transformer);

        // Populate cache.
        // self.cache = Some((drain.clone(), out.clone()));

        // Output stage is a transform_radians node.
        todo!();
        // out
    }
}

impl<DRAIN, PCNC, PCNU, T> Transform for Projector<DRAIN, PCNC, PCNU, T>
where
    T: CoordFloat + FloatConst,
{
    /// f32 or f64
    type T = T;

    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        self.transform.transform(p)
    }
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        self.transform.invert(p)
    }
}
