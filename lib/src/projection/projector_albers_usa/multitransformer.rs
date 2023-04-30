use std::fmt::Debug;

use geo::Coord;
use geo::CoordFloat;

use crate::stream::Stream;

use super::multidrain::Multidrain;
use super::multidrain::Populated;

/// Projections like `AlbersUSA` group several projections together.
/// TODO can I remove this wrapper.
#[derive(Debug)]
pub struct MultiTransformer<const N: usize, SD, SUBTRANS> {
    /// The contained endpoint.
    pub md: Multidrain<N, SD, Populated<N, SUBTRANS>>,
}

impl<const N: usize, SD, SUBTRANS> MultiTransformer<N, SD, SUBTRANS>
where
    SD: Clone + Default,
{
    /// Constructor
    #[must_use]
    pub fn new(store: [SUBTRANS; N]) -> Self {
        let md = Multidrain::new(SD::default());
        Self {
            md: md.populate(store),
        }
    }
}

impl<const N: usize, SD, SUBTRANS, T> Stream for MultiTransformer<N, SD, SUBTRANS>
where
    SUBTRANS: Stream<EP = SD, T = T>,
    T: CoordFloat,
{
    type T = T;
    type EP = Multidrain<N, SD, Populated<N, SUBTRANS>>;

    fn endpoint(&mut self) -> &mut Self::EP {
        &mut self.md
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
