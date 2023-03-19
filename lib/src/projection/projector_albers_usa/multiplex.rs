use std::marker::PhantomData;

use geo::{Coord, CoordFloat};

use crate::stream::Connectable;
use crate::stream::Stream;

#[derive(Clone, Debug)]
pub(super) struct Multiplex<DRAIN, const N: usize, P, T> {
    phantom_drain: PhantomData<DRAIN>,
    phantom_t: PhantomData<T>,
    store: [P; N],
}

// impl<SINK, const N: usize, P, T> Connectable for Multiplex<SINK, N, P, T>
// where
//     T: CoordFloat,
// {
//     type Output<SINK: Clone> = Multiplex<SINK, N, P, T>;
//     /// Connects the next stage in the stream pipline.
//     #[inline]
//     fn connect<SINK>(&self, sink: SINK) -> Self::Output<SINK>
//     where
//         SINK: Clone,
//     {
//         Mutli
//     }
// }

impl<DRAIN, const N: usize, P, T> Stream for Multiplex<DRAIN, N, P, T>
where
    DRAIN: Clone + PartialEq,
    P: Stream<EP = DRAIN, T = T>,
    T: CoordFloat,
{
    type EP = DRAIN;
    type T = T;
    /// Returns the end point of the stream.
    fn endpoint(&mut self) -> &mut Self::EP {
        todo!();
        // self.store
        //     .first()
        //     .expect("Cannot supply an empty list of Projectors.")
        //     .endpoint()
    }

    /// Declare the end of a line segment.
    fn line_end(&mut self) {
        for item in self.store.iter_mut() {
            item.line_end();
        }
    }

    /// Declare the start of a line segment.
    fn line_start(&mut self) {
        for item in self.store.iter_mut() {
            item.line_start();
        }
    }

    /// Declare a point.
    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        for item in self.store.iter_mut() {
            item.point(p, m);
        }
    }

    /// Declare the end of a polygon.
    fn polygon_end(&mut self) {
        for item in self.store.iter_mut() {
            item.polygon_end();
        }
    }
    /// Declare the start of a polygon.
    fn polygon_start(&mut self) {
        for item in self.store.iter_mut() {
            item.polygon_start();
        }
    }
    /// Declare a sphere object.
    fn sphere(&mut self) {
        for item in self.store.iter_mut() {
            item.sphere();
        }
    }
}
