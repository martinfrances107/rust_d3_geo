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

use super::multidrain::Multidrain;
use super::multidrain::Unpopulated;
use super::multitransformer::MultiTransformer;
use super::AlbersTransformer;

/// When connected the state changes to hold the connected Projectors.
#[derive(Debug)]
pub struct Connected<const N: usize, SUBTRANS> {
    /// A collections of sub transforms.
    /// TODO can this be simplified once workings.
    pub store: [SUBTRANS; N],
}

/// A projection stream pipeline stage which holds a collection of
/// Projectors, in the case of `AlbersUSA` one for every region.
/// `lower_48`, `alaaska`, `hawaii`.
///
/// NB This is not just a wrapper around an array of transforms.
/// to implement Transform here we store PR which hold the
/// complexity.
#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub fn connect<SD>(
        &self,
        drain: &Multidrain<3, SD, Unpopulated>,
    ) -> MultiTransformer<3, SD, AlbersTransformer<SD, T>>
    where
        T: Debug,
        SD: Clone + Default + PartialEq + Stream<EP = SD, T = T>,
    {
        let pr: AlbersUsa<SD, T> = AlbersUsa::<SD, T>::default();
        let sd = &drain.sd;

        // The order of objects in the store is important for performance.
        // The earlier a point is found the better,
        // so the lower_48 is searched first, and the smallest land area last.
        let store = [
            pr.lower_48.build().stream(sd),
            pr.alaska.build().stream(sd),
            pr.hawaii.build().stream(sd),
        ];
        MultiTransformer::new(store)
    }
}

impl<const N: usize, SD, T> Transform
    for Multiplex<AlbersUsa<SD, T>, Connected<N, AlbersUsaMultiTransformer<SD, T>>, T>
where
    SD: Clone + Stream<EP = SD, T = T>,
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
