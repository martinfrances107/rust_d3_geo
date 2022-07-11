use std::fmt::Debug;
use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};

use crate::projection::builder::PostClipNode;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;

/// Identity is a stream pipe line stage.
/// that acts as a pass through node.
pub struct Identity<EP, SINK, STATE> {
    p_ep: PhantomData<EP>,
    p_sink: PhantomData<SINK>,
    // p_t: PhantomData<T>,
    state: STATE,
}

impl<EP, SINK, STATE> Debug for Identity<EP, SINK, STATE>
where
    STATE: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.state).finish()
    }
}

impl<EP, SINK, STATE> Clone for Identity<EP, SINK, STATE>
where
    STATE: Clone,
{
    fn clone(&self) -> Self {
        Self {
            p_ep: PhantomData::<EP>,
            p_sink: PhantomData::<SINK>,
            // p_t: PhantomData::<T>,
            state: self.state.clone(),
        }
    }
}

/// Not auto deriving here - it does not makes sense to provide
/// a default for the connected state.
impl<EP, SINK> Default for Identity<EP, SINK, Unconnected> {
    fn default() -> Self {
        Self {
            p_ep: PhantomData::<EP>,
            p_sink: PhantomData::<SINK>,
            // p_t: PhantomData::<T>,
            state: Unconnected,
        }
    }
}

impl<EP, SINK, STATE> PostClipNode for Identity<EP, SINK, STATE> {}

impl<EP, SINK> Connectable for Identity<EP, SINK, Unconnected> {
    type SC = SINK;
    type Output = Identity<EP, SINK, Connected<Self::SC>>;
    fn connect(self, sink: Self::SC) -> Self::Output {
        Identity {
            p_ep: PhantomData::<EP>,
            p_sink: PhantomData::<SINK>,
            // p_t: PhantomData::<T>,
            state: Connected { sink },
        }
    }
}

impl<EP, SINK, T> Stream for Identity<EP, SINK, Connected<SINK>>
where
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn get_endpoint(&mut self) -> &mut Self::EP {
        self.state.sink.get_endpoint()
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<Self::T>, m: Option<u8>) {
        self.state.sink.point(p, m)
    }

    #[inline]
    fn sphere(&mut self) {
        self.state.sink.sphere()
    }

    #[inline]
    fn line_start(&mut self) {
        self.state.sink.line_start();
    }

    #[inline]
    fn line_end(&mut self) {
        self.state.sink.line_end();
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.state.sink.polygon_start();
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.state.sink.polygon_end();
    }
}
