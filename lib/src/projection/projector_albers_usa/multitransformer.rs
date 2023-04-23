use std::fmt::Debug;

use geo::Coord;
use geo::CoordFloat;

use crate::stream::Stream;

use super::multidrain::Multidrain;

/// Projections like `AlbersUSA` group several projections together.
/// TODO can I remove this wrapper.
#[derive(Debug)]
pub struct MultiTransformer<const N: usize, SD, T, TRANSFORM> {
    /// The contained endpoint.
    pub md: Multidrain<N, SD, T, TRANSFORM>,
}

impl<const N: usize, SD, T, TRANSFORM> MultiTransformer<N, SD, T, TRANSFORM>
where
    SD: Clone + Default,
{
    /// Constructor
    pub fn new(store: Vec<TRANSFORM>) -> Self {
        let md: Multidrain<N, SD, T, TRANSFORM> = Multidrain::default();
        Self {
            md: md.populate(store),
        }
    }
}

impl<const N: usize, SD, T, TRANSFORM> Stream for MultiTransformer<N, SD, T, TRANSFORM>
where
    T: CoordFloat,
    TRANSFORM: Stream<EP = SD, T = T>,
    SD: Stream<EP = SD, T = T>,
{
    type T = T;
    type EP = Self;

    fn endpoint(&mut self) -> &mut Self::EP {
        self
    }

    fn line_end(&mut self) {
        self.md.line_end();
    }

    fn line_start(&mut self) {
        self.md.line_start();
    }

    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        self.md.point(p, m);
    }

    fn polygon_end(&mut self) {
        self.md.polygon_end();
    }

    fn polygon_start(&mut self) {
        self.md.polygon_start();
    }

    fn sphere(&mut self) {
        self.md.sphere();
    }
}
