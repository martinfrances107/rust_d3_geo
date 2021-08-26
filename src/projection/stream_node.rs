use crate::stream::Stream;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use geo::CoordFloat;

/// Stream node is a internal to projection and clip.
///
/// The stream node processor is the
///
/// raw: the proto-node.
/// sink: the next streamNode in the chain.
///
/// T is required because SINK: Stream<T=T>
#[derive(Clone, Debug)]
pub struct StreamNode<RAW, SINK, T>
where
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    pub raw: RAW,
    pub sink: Rc<RefCell<SINK>>,
    pub pd: PhantomData<T>,
}
