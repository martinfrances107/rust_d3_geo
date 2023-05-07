use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::albers_usa::AlbersUsa;
use crate::projection::Build;
use crate::projection::Projector as ProjectoTait;
use crate::stream::Stream;
use crate::Transform;

use super::multidrain::Multidrain;
use super::multidrain::Populated;
use super::multidrain::Unpopulated;
use super::AlbersTransformer;

/// A projection stream pipeline stage which holds a collection of
/// Projectors, in the case of `AlbersUSA` one for every region.
/// `lower_48`, `alaaska`, `hawaii`.
///
/// NB This is not just a wrapper around an array of transforms.
/// to implement Transform here we store PR which hold the
/// complexity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Multiplex<PR, T> {
    p_t: PhantomData<T>,
    pr: PR,
}

impl<PR, T> Default for Multiplex<PR, T>
where
    PR: Default,
{
    fn default() -> Self {
        Self {
            p_t: PhantomData::<T>,
            pr: PR::default(),
        }
    }
}

/// Hardcode type for now until things are generic
impl<PR, T> Multiplex<PR, T>
where
    T: CoordFloat + Default + FloatConst,
{
    /// Connects the next stage in the stream pipline.
    #[inline]
    pub fn connect<SD>(
        &self,
        drain: &Multidrain<3, SD, Unpopulated>,
    ) -> Multidrain<3, SD, Populated<3, AlbersTransformer<SD, T>>>
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
            pr.lower_48_stream.build().stream(sd),
            pr.alaska_stream.build().stream(sd),
            pr.hawaii_stream.build().stream(sd),
        ];

        let md = Multidrain::new(SD::default());
        md.populate(store)
    }
}

impl<SD, T> Transform for Multiplex<AlbersUsa<SD, T>, T>
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
