use std::fmt::Debug;
use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::projection::str::scale_translate_rotate::ScaleTranslateRotate;
use crate::projection::stream_node::StreamNode;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;
use crate::Transform;

/// Resample None.
///
/// A pass-through module, when no resampling is required.
#[derive(Clone, Copy, Debug)]
pub struct None<PR, T>
where
    PR: ProjectionRaw<T>,
    T: CoordFloat + FloatConst,
{
    pt: PhantomData<T>,
    projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
}

impl<PR, T> None<PR, T>
where
    T: CoordFloat + FloatConst,
    PR: ProjectionRaw<T>,
{
    /// Contructor: Resample None.
    pub fn new(projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>) -> None<PR, T> {
        Self {
            pt: PhantomData::<T>,
            projection_transform,
        }
    }
}

impl<EP, PR, SINK, T> Stream for StreamNode<EP, None<PR, T>, SINK, T>
where
    EP: Clone + Debug + Stream<EP = EP, T = T>,
    PR: ProjectionRaw<T>,
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn get_endpoint(self) -> Self::EP {
        self.sink.get_endpoint()
    }

    #[inline]
    fn sphere(&mut self) {
        self.sink.sphere();
    }

    #[inline]
    fn line_start(&mut self) {
        self.sink.line_start();
    }

    #[inline]
    fn line_end(&mut self) {
        self.sink.line_end();
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.sink.polygon_start();
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.sink.polygon_end();
    }

    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        let t = &self.raw.projection_transform.transform(p);
        self.sink.point(t, m);
    }
}
