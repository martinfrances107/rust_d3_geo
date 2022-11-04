use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;

use crate::compose::Compose;
use crate::projection::transform::scale_translate_rotate::ScaleTranslateRotate;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

/// Resample None.
///
/// A pass-through module, when no resampling is required.
#[derive(Clone, Debug)]
pub struct None<PR, STATE, T> {
    state: STATE,
    p_t: PhantomData<T>,
    projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
}

impl<PR, T> None<PR, Unconnected, T> {
    #[inline]
    /// Constructor: Resample None.
    pub fn new(
        projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
    ) -> None<PR, Unconnected, T> {
        Self {
            state: Unconnected,
            p_t: PhantomData::<T>,
            projection_transform,
        }
    }
}

impl<PR, T> Connectable for None<PR, Unconnected, T>
where
    T: CoordFloat,
{
    type Output<SC: Clone> = None<PR, Connected<SC>, T>;

    #[inline]
    fn connect<SC: Clone>(self, sink: SC) -> Self::Output<SC> {
        None::<PR, Connected<SC>, T> {
            state: Connected { sink },
            p_t: self.p_t,
            projection_transform: self.projection_transform,
        }
    }
}

impl<EP, PR, SC, T> Stream for None<PR, Connected<SC>, T>
where
    SC: Clone + Stream<EP = EP, T = T>,
    PR: Transform<T = T>,
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

    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        let t = &self.projection_transform.transform(p);
        self.state.sink.point(t, m);
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
        self.state.sink.sphere();
    }
}
