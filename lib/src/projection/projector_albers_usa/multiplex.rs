use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::albers_usa::AlbersUsa;
use crate::projection::projector_albers_usa::AlbersUsaMultiTransformer;
use crate::projection::Build;
use crate::projection::Projector as ProjectoTait;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::multitransformer::MultiTransformer;

/// When connected the state changes to hold the connected Projectors.
#[derive(Debug)]
pub struct Connected<const N: usize, TRANSFORM> {
    /// A collections of sub transforms.
    /// TODO can this be simplified once workings.
    pub store: [TRANSFORM; N],
}

/// A projection stream pipeline stage which holds a collection of
/// Projectors, in the case of `AlbersUSA` one for every region.
/// `lower_48`, `alaaska`, `hawaii`.
#[derive(Clone, Debug)]
pub struct Multiplex<PR, STATE, T> {
    p_t: PhantomData<T>,
    pr: PR,
    /// The State is Connected or Unconnected.
    /// TODO Once things are working consider simplifying here
    /// by removing this wrapper.
    pub state: STATE,
}

impl<PR, T> Default for Multiplex<PR, Unconnected, T>
where
    PR: Default,
{
    fn default() -> Self {
        Self {
            p_t: PhantomData::<T>,
            pr: PR::default(),
            state: Unconnected,
        }
    }
}

/// Hardcode type for now until things are generic
impl<PR, T> Multiplex<PR, Unconnected, T>
where
    T: CoordFloat + Default + FloatConst,
{
    /// Connects the next stage in the stream pipline.
    #[inline]
    pub fn connect<SD>(&self, sink: &SD) -> AlbersUsaMultiTransformer<3, SD, T>
    where
        T: Debug,
        SD: Clone + Default + PartialEq + Stream<EP = SD, T = T>,
    {
        let pr = AlbersUsa::<SD, T>::default();
        // The order of objects in the store is important for performance.
        // The earlier a point is found the better,
        // so the lower_48 is searched first, and the smallest land area last.
        let store = [
            pr.lower_48.build().stream(&sink.clone()),
            pr.alaska.build().stream(&sink.clone()),
            pr.hawaii.build().stream(&sink.clone()),
        ];
        MultiTransformer::new(store)
    }
}

impl<DRAIN, const N: usize, T> Transform
    for Multiplex<AlbersUsa<DRAIN, T>, Connected<N, AlbersUsaMultiTransformer<N, DRAIN, T>>, T>
where
    DRAIN: Clone + Stream<EP = DRAIN, T = T>,
    T: 'static + CoordFloat + Default + FloatConst,
{
    /// f32 or f64
    type T = T;

    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        self.pr.transform(p)
    }
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        self.pr.invert(p)
    }
}
