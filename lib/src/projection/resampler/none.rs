use core::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;

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
    projection_transform: Compose<PR, ScaleTranslateRotate<T>>,
}

impl<PR, T> None<PR, Unconnected, T> {
    #[inline]
    /// Constructor: Resample None.
    pub const fn new(projection_transform: Compose<PR, ScaleTranslateRotate<T>>) -> Self {
        Self {
            state: Unconnected,
            projection_transform,
        }
    }
}

impl<PR, T> Connectable for None<PR, Unconnected, T>
where
    PR: Clone,
    T: CoordFloat,
{
    type Output<SC> = None<PR, Connected<SC>, T>;

    #[inline]
    fn connect<SC>(&self, sink: SC) -> Self::Output<SC> {
        None::<PR, Connected<SC>, T> {
            state: Connected { sink },
            projection_transform: self.projection_transform.clone(),
        }
    }
}

impl<EP, PR, SC, T> Stream for None<PR, Connected<SC>, T>
where
    SC: Stream<EP = EP, T = T>,
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

    fn point(&mut self, p: &Coord<T>, m: Option<u8>) {
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
