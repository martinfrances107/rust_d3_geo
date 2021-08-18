use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

/// Stream node is a internal to projection and clip.
///
/// The stream node processor is the
///
/// raw: the proto-node.
/// sink: the next streamNode in the chain.
///
/// todo return and remove T.
#[derive(Clone, Debug)]
pub struct StreamNode<RAW, SINK, T> {
    pub raw: RAW,
    pub sink: Rc<RefCell<SINK>>,
    pub pd: PhantomData<T>,
}
