use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;

use crate::stream::Stream;

use super::multidrain::Multidrain;

/// Projections like `AlbersUSA` group several projections together.
/// TODO can I remove this wrapper.
#[derive(Debug)]
pub struct MultiTransformer<const N: usize, SD, T, TRANSFORMER> {
    /// The contained endpoint.
    pub md: Multidrain<N, SD, T, TRANSFORMER>,
}

impl<const N: usize, SD, T, TRANSFORMER> MultiTransformer<N, SD, T, TRANSFORMER>
where
    SD: Default,
{
    /// Constructor
    pub fn new(store: [TRANSFORMER; N]) -> Self {
        Self {
            md: Multidrain::new(store),
        }
    }
}

impl<const N: usize, SD, T, TRANSFORMER> Stream for MultiTransformer<N, SD, T, TRANSFORMER>
where
    T: CoordFloat,
    TRANSFORMER: Stream<EP = SD, T = T>,
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
