use std::cell::RefCell;
use std::rc::Rc;

use core::marker::PhantomData;
use derivative::Derivative;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::identity::Identity;
use crate::projection::post_clip_node::PostClipNode;
use crate::projection::stream_node::StreamNode;
use crate::projection::NodeFactory;
use crate::stream::Stream;

use super::post_clip::PostClip;

/// Used in the construct of a Projection stream pipeline.
///
/// Stream Raw (SR) is the proto-node. ( The node without the link to other node's up the chain)
///
/// SR is precomputed and held in the projection.
///
/// Inside Projection::stream() NodeFactory::generate() will be called to
/// construct the pipeline.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct StreamNodePostClipFactory<SINK, T>
where
    T: CoordFloat + FloatConst,
{
    phantom_sink: PhantomData<SINK>,
    post_clip: PostClip<T>,
}

impl<SINK, T> StreamNodePostClipFactory<SINK, T>
where
    T: CoordFloat + FloatConst,
{
    pub fn new(post_clip: PostClip<T>) -> StreamNodePostClipFactory<SINK, T> {
        StreamNodePostClipFactory {
            phantom_sink: PhantomData::<SINK>,
            post_clip,
        }
    }
}

impl<SINK, T> NodeFactory for StreamNodePostClipFactory<SINK, T>
where
    SINK: Stream<T = T>,
    T: CoordFloat + FloatConst,
{
    type Sink = SINK;
    // type Node = PostClipNode<SINK, T>;
    type T = T;
    type Node = PostClipNode<SINK, Self::T>;
    fn generate(&self, sink: Rc<RefCell<SINK>>) -> Self::Node {
        match &self.post_clip {
            PostClip::I(i) => PostClipNode::I(StreamNode {
                raw: Identity {},
                sink: sink,
            }),
            PostClip::R(r) => PostClipNode::R(StreamNode {
                raw: r.clone(),
                sink,
            }),
        }
    }
}
