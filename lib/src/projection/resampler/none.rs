use std::fmt::Debug;
use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::projection::transform::scale_translate_rotate::ScaleTranslateRotate;
use crate::projection::ProjectionRawBase;
use crate::stream::Connectable;
use crate::stream::Connected;
//use crate::stream::ConnectionState;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Resampler;

/// Resample None.
///
/// A pass-through module, when no resampling is required.
#[derive(Clone, Debug)]
pub struct None<EP, PR, SC, SU, STATE, T>
where
    T: CoordFloat + FloatConst,
    PR: Clone,
{
    state: STATE,
    p_ep: PhantomData<EP>,
    p_sc: PhantomData<SC>,
    p_su: PhantomData<SU>,
    p_t: PhantomData<T>,
    projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
}

impl<EP, PR, SC, SU, T> None<EP, PR, SC, SU, Unconnected, T>
where
    T: CoordFloat + FloatConst,
    PR: Clone + Transform<T = T>,
{
    /// Constructor: Resample None.
    pub fn new(
        projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
    ) -> None<EP, PR, SC, SU, Unconnected, T> {
        Self {
            state: Unconnected,
            p_ep: PhantomData::<EP>,
            p_sc: PhantomData::<SC>,
            p_su: PhantomData::<SU>,
            p_t: PhantomData::<T>,
            projection_transform,
        }
    }
}

impl<EP, PR, SC, SU, STATE, T> Resampler for None<EP, PR, SC, SU, STATE, T>
where
    EP: Clone + Debug,
    PR: Clone + Debug,
    SC: Clone + Debug,
    SU: Clone + Debug,
    //STATE: ConnectionState,
    STATE: Clone + Debug,
    PR: Transform<T = T>,
    T: CoordFloat + FloatConst,
{
}

impl<EP, PR, SC, SU, T> Connectable for None<EP, PR, SC, SU, Unconnected, T>
where
    PR: Clone + Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type Output = None<EP, PR, SC, SU, Connected<SC>, T>;
    type SC = SC;
    fn connect(self, sink: SC) -> Self::Output {
        None::<EP, PR, SC, SU, Connected<SC>, T> {
            state: Connected { sink },
            p_ep: PhantomData::<EP>,
            p_sc: PhantomData::<SC>,
            p_su: PhantomData::<SU>,
            p_t: self.p_t,

            projection_transform: self.projection_transform,
        }
    }
}

impl<EP, PR, SC, SU, T> Stream for None<EP, PR, SC, SU, Connected<SC>, T>
where
    EP: Stream<EP = EP, T = T> + Default,
    PR: ProjectionRawBase<T>,
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
    fn sphere(&mut self) {
        self.state.sink.sphere();
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

    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        let t = &self.projection_transform.transform(p);
        self.state.sink.point(t, m);
    }
}
