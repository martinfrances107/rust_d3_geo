use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::projector_albers_usa::Projector;
use crate::stream::Stream;

use super::albers_usa::AlbersUsa;
use super::BuilderTrait;

mod clip_extent_asjust;
mod scale_get;
mod scale_set;
mod translate_get;
mod translate_set;

/// A wrapper over Projection\Builder which hold state phi0, phi1 and allow regeneration of the PR.
#[derive(Clone, Debug)]
pub struct Builder<SD, T>
where
    SD: Clone,
    T: CoordFloat + Debug + Default + FloatConst,
{
    phantom_sd: PhantomData<SD>,
    /// The underlying projection.
    pub pr: AlbersUsa<SD, T>,
}

impl<SD, T> BuilderTrait for Builder<SD, T>
where
    T: CoordFloat + Debug + Default + FloatConst,
    SD: Clone,
{
    type PR = AlbersUsa<SD, T>;

    /// Given a Raw Projection and a clipping defintion create the associated
    /// Projection builder.
    ///
    /// # Panics
    /// unwrap() is used here but a panic will never happen as constants will always be converted into T.
    fn new(pr: AlbersUsa<SD, T>) -> Self {
        Self {
            phantom_sd: PhantomData::<SD>,
            pr,
        }
    }
}

impl<SD, T> Builder<SD, T>
where
    SD: Clone + Default + PartialEq + Stream<EP = SD, T = T>,
    T: CoordFloat + Debug + Default + FloatConst,
{
    /// Using the currently programmed state output a new projection.
    #[inline]
    #[must_use]
    pub fn build(&self) -> Projector<SD, T> {
        Projector::<SD, T>::default()
    }
}
