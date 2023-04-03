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
pub struct MultiTransformer<DRAIN, STATE, T, TRANSFORMER> {
    state: STATE,
    store: Vec<TRANSFORMER>,
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T, TRANSFORMER> MultiTransformer<DRAIN, Connected<DRAIN>, T, TRANSFORMER> {
    /// Constructor
    pub fn new(sink: DRAIN, store: Vec<TRANSFORMER>) -> Self {
        Self {
            state: Connected { sink },
            store,
            p_drain: PhantomData::<DRAIN>,
            p_t: PhantomData::<T>,
        }
    }
}

impl<DRAIN, T, TRANSFORMER> Connectable for MultiTransformer<DRAIN, Unconnected, T, TRANSFORMER>
where
    T: CoordFloat,
    TRANSFORMER: Clone,
{
    type Output<SC: Clone> = MultiTransformer<DRAIN, Connected<SC>, T, TRANSFORMER>;

    #[inline]
    fn connect<SC: Clone>(&self, sink: SC) -> Self::Output<SC> {
        Self::Output {
            state: Connected { sink },
            p_t: self.p_t,
            p_drain: self.p_drain,
            store: self.store.clone(),
        }
    }
}

impl<SD, T, TRANSFORMER> Stream
    for MultiTransformer<Multidrain<SD, T>, Connected<Multidrain<SD, T>>, T, TRANSFORMER>
where
    SD: Stream<EP = SD, T = T>,
    T: CoordFloat,
    // TODO must define ER=XXX?
    TRANSFORMER: Stream<T = T>,
{
    type T = T;
    type EP = Multidrain<SD, T>;

    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.sink.endpoint()
    }

    fn line_end(&mut self) {
        for item in &mut self.store {
            item.line_end();
        }
    }

    fn line_start(&mut self) {
        for item in &mut self.store {
            item.line_start();
        }
    }

    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        for item in &mut self.store {
            item.point(p, m);
        }
    }

    fn polygon_end(&mut self) {
        for item in &mut self.store {
            item.polygon_end();
        }
    }

    fn polygon_start(&mut self) {
        for item in &mut self.store {
            item.polygon_start();
        }
    }

    fn sphere(&mut self) {
        for item in &mut self.store {
            item.sphere();
        }
    }
}
