use std::fmt::Debug;
use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::projection::transform::scale_translate_rotate::ScaleTranslateRotate;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::Resampler;

/// Resample None.
///
/// A pass-through module, when no resampling is required.
#[derive(Clone)]
pub struct None<PR, SC, STATE, T> {
    state: STATE,
    /// PhantomData<SC>
    /// The hidden linkage is in Connetable
    /// when the input parameter changes so
    /// must the output.
    p_sc: PhantomData<SC>,

    p_t: PhantomData<T>,
    projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
}

impl<PR, SC, STATE, T> Debug for None<PR, SC, STATE, T>
where
    STATE: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.state).finish()
    }
}

impl<PR, SC, T> None<PR, SC, Unconnected, T> {
    /// Constructor: Resample None.
    pub fn new(
        projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
    ) -> None<PR, SC, Unconnected, T> {
        Self {
            state: Unconnected,
            p_sc: PhantomData::<SC>,
            p_t: PhantomData::<T>,
            projection_transform,
        }
    }
}

impl<PR, SC, STATE, T> Resampler for None<PR, SC, STATE, T> where T: CoordFloat + FloatConst {}

impl<PR, SC, T> Connectable for None<PR, SC, Unconnected, T>
where
    PR: Transform<T = T>,
    T: CoordFloat + FloatConst,
{
    type Output = None<PR, SC, Connected<SC>, T>;
    type SC = SC;
    fn connect(self, sink: SC) -> Self::Output {
        None::<PR, SC, Connected<SC>, T> {
            state: Connected { sink },
            p_sc: PhantomData::<SC>,
            p_t: self.p_t,

            projection_transform: self.projection_transform,
        }
    }
}

impl<EP, PR, SC, T> Stream for None<PR, SC, Connected<SC>, T>
where
    SC: Stream<EP = EP, T = T>,
    PR: Transform<T = T>,
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
