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
pub struct MultiTransformer<DRAIN, const N: usize, STATE, T, TRANSFORMER> {
    state: STATE,
    store: [TRANSFORMER; N],
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, const N: usize, T, TRANSFORMER>
    MultiTransformer<DRAIN, N, Connected<DRAIN>, T, TRANSFORMER>
{
    /// Constructor
    pub const fn new(sink: DRAIN, store: [TRANSFORMER; N]) -> Self {
        Self {
            state: Connected { sink },
            store,
            p_drain: PhantomData::<DRAIN>,
            p_t: PhantomData::<T>,
        }
    }
}

impl<DRAIN, const N: usize, T, TRANSFORMER> Connectable
    for MultiTransformer<DRAIN, N, Unconnected, T, TRANSFORMER>
where
    T: CoordFloat,
    TRANSFORMER: Clone,
{
    type Output<SC: Clone> = MultiTransformer<DRAIN, N, Connected<SC>, T, TRANSFORMER>;

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

impl<SD, const N: usize, T, TRANSFORMER> Stream
    for MultiTransformer<Multidrain<N, SD, T>, N, Connected<Multidrain<N, SD, T>>, T, TRANSFORMER>
where
    SD: Stream<EP = SD, T = T> + Debug,
    T: CoordFloat,
    // TODO must define ER=XXX?
    TRANSFORMER: Stream<T = T>,
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
        dbg!("MultiTransformer {:?}", p);
        dbg!("{:?}", &self.state.sink);
        dbg!("length {}", self.state.sink.drains.len());
        // TODO - must add transform here.
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
