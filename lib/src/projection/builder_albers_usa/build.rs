use std::marker::PhantomData;

use super::Builder;
use crate::projection::projector_albers_usa::Projector;

impl<DRAIN> Builder<DRAIN>
where
    DRAIN: Clone,
{
    /// Using the currently programmed state output a new projection.
    #[inline]
    #[must_use]
    pub fn build<MULTIPLEX: Default>(&self) -> Projector<DRAIN, MULTIPLEX> {
        Projector::<DRAIN, MULTIPLEX>::default()
    }
}
