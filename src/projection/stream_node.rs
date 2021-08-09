// use crate::stream::Stream;
// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;
use std::cell::RefCell;
use std::marker::PhantomData;
// use std::fmt::Display;
// use std::ops::AddAssign;
use std::rc::Rc;

/// Stream node is a internal the projection.
///
/// The stream node processor is the
///
/// raw: the proto-node.
/// sink: the next streamNode in the chain.
///
#[derive(Clone, Debug)]
pub struct StreamNode<RAW, SINK, T> {
    pub raw: RAW,
    pub sink: Rc<RefCell<SINK>>,
    pub pd: PhantomData<T>,
}
