use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::marker::PhantomData;

use crate::compose::Compose;
use crate::projection::str::scale_translate_rotate::ScaleTranslateRotate;
use crate::projection::stream_node::StreamNode;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;
use crate::Transform;

/// Resample None.
#[derive(Clone, Copy, Debug)]
pub struct None<PR, T>
where
    PR: ProjectionRaw<T>,
    T: CoordFloat + FloatConst,
{
    pt: PhantomData<T>,
    projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
}

// impl<PR, T> Default for None<PR, T>
// where
//     PR: ProjectionRaw<T>,
//     T: CoordFloat,
// {
//     fn default() -> Self {
//         Self {
//             pt: PhantomData::<T>,
//             projection_raw: PR::default(),
//         }
//     }
// }

impl<PR, T> None<PR, T>
where
    T: CoordFloat + FloatConst,
    PR: ProjectionRaw<T>,
{
    pub fn new(projection_transform: Compose<T, PR, ScaleTranslateRotate<T>>) -> None<PR, T> {
        Self {
            pt: PhantomData::<T>,
            projection_transform,
        }
    }
}

impl<PR, SINK, T> Stream for StreamNode<None<PR, T>, SINK, T>
where
    PR: ProjectionRaw<T>,
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn sphere(&mut self) {
        self.sink.borrow_mut().sphere();
    }

    #[inline]
    fn line_start(&mut self) {
        self.sink.borrow_mut().line_start();
    }

    #[inline]
    fn line_end(&mut self) {
        self.sink.borrow_mut().line_end();
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.sink.borrow_mut().polygon_start();
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.sink.borrow_mut().polygon_end();
    }

    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        let t = &self.raw.projection_transform.transform(p);
        let mut s = self.sink.borrow_mut();
        s.point(t, m);
    }
}
