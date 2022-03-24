use std::fmt::Debug;
use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::projection::builder::PostClipNode;
use crate::stream::Connectable;
use crate::stream::Connected;
// //use crate::stream::ConnectionState;
use crate::stream::Stream;
use crate::stream::Unconnected;

/// Identity is a stream pipe line stage.
/// that acts as a pass through node.
#[derive(Debug, Clone)]
pub struct Identity<EP, SC, SU, STATE, T>
where
// EP: Stream<EP = EP, T = T> + Default,
// SU: Clone + Debug,
// SC: Stream<EP = EP, T = T>,
// T: CoordFloat,
{
    p_ep: PhantomData<EP>,
    p_sc: PhantomData<SC>,
    p_su: PhantomData<SU>,
    p_t: PhantomData<T>,
    state: STATE,
}

/// Not auto deriving here - it does not makes sense to provide
/// a default for the connected state.
impl<EP, SC, SU, T> Default for Identity<EP, SC, SU, Unconnected, T>
// where
// EP: Stream<EP = EP, T = T> + Default,
// SU: Clone + Debug,
// SC: Stream<EP = EP, T = T>,
// T: CoordFloat,
{
    fn default() -> Self {
        Self {
            p_ep: PhantomData::<EP>,
            p_sc: PhantomData::<SC>,
            p_su: PhantomData::<SU>,
            p_t: PhantomData::<T>,
            state: Unconnected,
        }
    }
}

impl<EP, SC, SU, STATE, T> PostClipNode for Identity<EP, SC, SU, STATE, T>
where
    EP: Clone + Debug,
    SC: Clone + Debug,
    SU: Clone + Debug,
    STATE: Clone + Debug,
    T: Clone + Debug,
    //     EP: Stream<EP = EP, T = T> + Default,
    //     SC: Stream<EP = EP, T = T>,
    //     SU: Clone + Debug,
    //     STATE: Clone + Debug,
    //     T: CoordFloat,
{
}

impl<EP, SC, SU, T> Identity<EP, SC, SU, Unconnected, T>
where
// EP: Stream<EP = EP, T = T> + Default,
// SC: Stream<EP = EP, T = T>,
// SU: Clone + Debug,
// T: CoordFloat,
{
    pub fn default() -> Identity<EP, SC, SU, Unconnected, T> {
        Identity {
            p_ep: PhantomData::<EP>,
            p_sc: PhantomData::<SC>,
            p_su: PhantomData::<SU>,
            p_t: PhantomData::<T>,
            state: Unconnected,
        }
    }
}

impl<EP, SC, SU, T> Connectable for Identity<EP, SC, SU, Unconnected, T>
where
    // EP: Stream<EP = EP, T = T> + Default,
    SU: Clone + Debug,
    // SC: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    type SC = SC;
    type Output = Identity<EP, SC, SU, Connected<Self::SC>, T>;
    fn connect(self, sink: Self::SC) -> Self::Output {
        Identity {
            p_ep: PhantomData::<EP>,
            p_sc: PhantomData::<SC>,
            p_su: PhantomData::<SU>,
            p_t: PhantomData::<T>,
            state: Connected { sink },
        }
    }
}

impl<EP, SC, SU, T> Stream for Identity<EP, SC, SU, Connected<SC>, T>
where
    // EP: Stream<EP = EP, T = T> + Default,
    EP: Clone + Debug,
    SU: Clone + Debug,
    SC: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
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
