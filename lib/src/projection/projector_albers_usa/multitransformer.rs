use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;

use crate::stream::Stream;

/// Projections like `AlbersUSA` group several projections together.
#[derive(Debug)]
pub struct MultiTransformer<DRAIN, T, TRANSFORMER> {
    store: Vec<TRANSFORMER>,
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T, TRANSFORMER> MultiTransformer<DRAIN, T, TRANSFORMER> {
    /// Constructor
    pub fn new(store: Vec<TRANSFORMER>) -> Self {
        Self {
            store,
            p_drain: PhantomData::<DRAIN>,
            p_t: PhantomData::<T>,
        }
    }
}

impl<DRAIN, T, TRANSFORMER> Stream for MultiTransformer<DRAIN, T, TRANSFORMER>
where
    T: CoordFloat,
    TRANSFORMER: Stream<EP = DRAIN, T = T>,
{
    type T = T;
    type EP = DRAIN;

    fn endpoint(&mut self) -> &mut Self::EP {
        todo!();
    }

    fn line_end(&mut self) {}

    fn line_start(&mut self) {}

    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {}

    fn polygon_end(&mut self) {}

    fn polygon_start(&mut self) {}

    fn sphere(&mut self) {}
}
