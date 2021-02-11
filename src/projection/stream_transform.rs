use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::cell::RefCell;
use std::rc::Rc;

use crate::stream::StreamPreClipNode;
use crate::stream::StreamPreClipNodeStub;
use crate::stream::StreamSimple;
use crate::stream::StreamTransformNode;

use crate::{stream::Stream, TransformIdentity};

use crate::Transform;

pub struct StreamTransform<T> {
    transform: Rc<Box<dyn Transform<T>>>,
    stream: StreamPreClipNode<T>,
}

pub trait StreamPreclipIn<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_preclip_in(&mut self, stream: StreamPreClipNode<T>);
}

impl<T: CoordFloat + FloatConst + 'static> StreamPreclipIn<T> for StreamTransform<T> {
    #[inline]
    fn stream_preclip_in(&mut self, stream: StreamPreClipNode<T>) {
        self.stream = stream;
    }
}

// pub type StreamTransformNode<T> = Rc<RefCell<Box<StreamTransform<T>>>>;
impl<T: CoordFloat + FloatConst + 'static> StreamTransform<T> {
    #[inline]
    pub fn gen_node(transform: Option<Rc<Box<dyn Transform<T>>>>) -> StreamTransformNode<T> {
        {
            match transform {
                Some(transform) => Rc::new(RefCell::new(Box::new(Self {
                    transform: transform.clone(),
                    stream: StreamPreClipNodeStub::new(),
                }))),
                None => Rc::new(RefCell::new(Box::new(Self {
                    transform: Rc::new(Box::new(TransformIdentity {})),
                    stream: StreamPreClipNodeStub::new(),
                }))),
            }
        }
    }
}

impl<T> StreamSimple<T> for StreamTransform<T> where T: CoordFloat + FloatConst + 'static {}

impl<T: CoordFloat + FloatConst> Stream<T> for StreamTransform<T> {
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        let mut stream = self.stream.borrow_mut();
        let rotate = self.transform.clone();
        let r = rotate.transform(&p);
        // Warning the javascript version return the value below but I think it break the implied spec!!!!
        stream.point(r, m);
    }
}
