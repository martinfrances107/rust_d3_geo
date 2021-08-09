use crate::projection::stream_node::StreamNode;
// use crate::projection::stream_node_factory::StreamNodeFactory;
// use crate::projection::NodeFactory;
use crate::projection::Raw as ProjectionRaw;
// use std::cell::RefCell;
use std::fmt::Display;
// use std::rc::Rc;
// use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// use crate::projection::ProjectionRawTrait;
// use super::ResampleTrait;
// use super::ResampleEnum;
use crate::stream::Stream;

/// Resample None.
#[derive(Clone, Copy, Debug)]
pub struct None<PR, T>
where
    PR: ProjectionRaw<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    // pd: PhantomData<&'a u8>,
    projection_raw: PR, // Box to prevent infinite recusion.
                        // pub stream: Box<ClipSinkEnum<'a, PR, T>>,
                        // pub stream: Box<STREAM>
}

impl<PR, T> Default for None<PR, T>
where
    PR: ProjectionRaw<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    fn default() -> Self {
        Self {
            projection_raw: PR::default(),
        }
    }
}

// impl<PR, SINK, T> NodeFactory for StreamNodeFactory<SINK, None<PR, T>, T>
// where
//     PR: ProjectionRaw<T = T>,
//     SINK: Stream<SC = Coordinate<T>>,
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// {
//     type T = T;
//     type Sink = SINK;
//     type SR = None<PR, T>;
//     fn generate(
//         &self,
//         sink: Rc<RefCell<Self::Sink>>,
//     ) -> Rc<RefCell<StreamNode<Self::SR, Self::Sink>>> {
//         Rc::new(RefCell::new(StreamNode {
//             raw: self.raw.clone(),
//             sink: sink,
//         }))
//     }
// }

impl<PR, T> None<PR, T>
where
    // STREAM: Stream<SC = Coordinate<T>> + Default,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    PR: ProjectionRaw<T = T>,
{
    pub fn new(projection_raw: PR) -> None<PR, T> {
        Self {
            // pd: PhantomData,
            projection_raw,
            // stream: Box::new(STREAM::default()), // stub value
        }
    }
}

impl<PR, SINK, T> Stream for StreamNode<None<PR, T>, SINK, T>
where
    PR: ProjectionRaw<T = T>,
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SC = Coordinate<T>;

    #[inline]
    fn sphere(&mut self) {
        self.sink.borrow_mut().sphere()
    }

    #[inline]
    fn line_start(&mut self) {
        self.sink.borrow_mut().line_start()
    }

    #[inline]
    fn line_end(&mut self) {
        self.sink.borrow_mut().line_end()
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.sink.borrow_mut().polygon_start()
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.sink.borrow_mut().polygon_end()
    }

    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        let t = &self.raw.projection_raw.transform(p);
        self.sink.borrow_mut().point(t, m);
    }
}
