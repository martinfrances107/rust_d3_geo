use std::fmt::Debug;
use std::marker::PhantomData;

use crate::projection::projector_albers_usa::Projector;
use crate::stream::Stream;
use crate::stream::Unconnected;

use super::albers_usa::AlbersUsa;
use super::projector_albers_usa::multiplex::Multiplex;
use super::BuilderTrait;
use super::RawBase;

mod build;
mod clip_extent_asjust;
mod scale_get;
mod scale_set;
mod translate_get;
mod translate_set;

/// Adjustments the pair of parallels
/// use to define the proejctions.
///
/// Differs from PR in the way the PR is generated.
pub trait PRConic: RawBase {
    /// Late initisalisation of a projection
    /// based on a pair of parallels.
    #[must_use]
    fn generate(self, y0: Self::T, y1: Self::T) -> Self;
}

/// A wrapper over Projection\Builder which hold state phi0, phi1 and allow regeneration of the PR.
#[derive(Clone, Debug)]
pub struct Builder<DRAIN>
where
    DRAIN: Clone,
{
    phantom_drain: PhantomData<DRAIN>,
    /// The underlying projection.
    pub pr: AlbersUsa<DRAIN>,
}

impl<DRAIN> BuilderTrait for Builder<DRAIN>
where
    DRAIN: Clone,
{
    type PR = AlbersUsa<DRAIN>;

    /// Given a Raw Projection and a clipping defintion create the associated
    /// Projection builder.
    ///
    /// # Panics
    /// unwrap() is used here but a panic will never happen as constants will always be converted into T.
    fn new(pr: AlbersUsa<DRAIN>) -> Self {
        Self {
            phantom_drain: PhantomData::<DRAIN>,
            pr,
        }
    }
}

impl<DRAIN> Builder<DRAIN>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    /// Using the currently programmed state output a new projection.
    #[inline]
    #[must_use]
    pub fn build(&self) -> Projector<DRAIN, Multiplex<AlbersUsa<DRAIN>, Unconnected>> {
        Projector::<DRAIN, Multiplex<AlbersUsa<DRAIN>, Unconnected>>::default()
    }
}
