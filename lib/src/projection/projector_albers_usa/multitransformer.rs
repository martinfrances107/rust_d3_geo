use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;

use crate::multidrain::Multidrain;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;

/// Projections like `AlbersUSA` group several projections together.
#[derive(Debug)]
pub struct MultiTransformer<const N: usize, STATE, TRANSFORMER> {
    state: STATE,
    store: [TRANSFORMER; N],
}

impl<DRAIN, const N: usize, TRANSFORMER> MultiTransformer<N, Connected<DRAIN>, TRANSFORMER> {
    /// Constructor
    pub const fn new(sink: DRAIN, store: [TRANSFORMER; N]) -> Self {
        Self {
            state: Connected { sink },
            store,
        }
    }
}

impl<const N: usize, TRANSFORMER> Connectable for MultiTransformer<N, Unconnected, TRANSFORMER>
where
    TRANSFORMER: Clone,
{
    type Output<SC: Clone> = MultiTransformer<N, Connected<SC>, TRANSFORMER>;

    #[inline]
    fn connect<SC: Clone>(&self, sink: SC) -> Self::Output<SC> {
        Self::Output {
            state: Connected { sink },
            store: self.store.clone(),
        }
    }
}

impl<SD, const N: usize, T, TRANSFORMER> Stream
    for MultiTransformer<N, Connected<Multidrain<N, SD, T>>, TRANSFORMER>
where
    SD: Stream<EP = SD, T = T> + Debug,
    T: CoordFloat,
{
    type T = T;
    type EP = Multidrain<N, SD, T>;

    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.sink.endpoint()
    }

    fn line_end(&mut self) {
        for item in &mut self.state.sink.drains {
            item.line_end();
        }
    }

    fn line_start(&mut self) {
        for item in &mut self.state.sink.drains {
            item.line_start();
        }
    }

    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        for item in &mut self.state.sink.drains {
            item.point(p, m);
        }
    }

    fn polygon_end(&mut self) {
        for item in &mut self.state.sink.drains {
            item.polygon_end();
        }
    }

    fn polygon_start(&mut self) {
        for item in &mut self.state.sink.drains {
            item.polygon_start();
        }
    }

    fn sphere(&mut self) {
        for item in &mut self.state.sink.drains {
            item.sphere();
        }
    }
}
