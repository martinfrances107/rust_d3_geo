use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;

use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;

/// Identity is a stream pipe line stage.
/// that acts as a pass through node.
#[derive(Debug)]
pub struct Identity<SINK, STATE> {
    // PhantomData:
    // The hidden linkage is in the implementation of Connectable.
    // if Self::SC changes then Self::Output must change.
    p_sink: PhantomData<SINK>,
    state: STATE,
}

impl<SINK, STATE> Clone for Identity<SINK, STATE>
where
    STATE: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            p_sink: PhantomData::<SINK>,
            state: self.state.clone(),
        }
    }
}

/// Not auto deriving here - it does not makes sense to provide
/// a default for the connected state.
impl<SINK> Default for Identity<SINK, Unconnected> {
    #[inline]
    fn default() -> Self {
        Self {
            p_sink: PhantomData::<SINK>,
            state: Unconnected,
        }
    }
}

impl<SINK> Connectable for Identity<SINK, Unconnected>
where
    SINK: Clone,
{
    type SC = SINK;
    /// The resultant builder type.
    type Output = Identity<SINK, Connected<Self::SC>>;

    #[inline]
    fn connect(self, sink: Self::SC) -> Self::Output {
        Identity {
            p_sink: PhantomData::<SINK>,
            state: Connected { sink },
        }
    }
}

impl<EP, SINK, T> Stream for Identity<SINK, Connected<SINK>>
where
    SINK: Clone + Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.sink.endpoint()
    }

    #[inline]
    fn line_end(&mut self) {
        self.state.sink.line_end();
    }

    #[inline]
    fn line_start(&mut self) {
        self.state.sink.line_start();
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<Self::T>, m: Option<u8>) {
        self.state.sink.point(p, m)
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.state.sink.polygon_end();
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.state.sink.polygon_start();
    }

    #[inline]
    fn sphere(&mut self) {
        self.state.sink.sphere()
    }
}
