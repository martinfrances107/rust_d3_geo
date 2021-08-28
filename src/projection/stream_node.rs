use std::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;

use crate::stream::Stream;

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
    /// The proto node, that is the struct without reference to the sink.
    pub raw: RAW,
    /// The downstream node.
    pub sink: Rc<RefCell<SINK>>,
}
