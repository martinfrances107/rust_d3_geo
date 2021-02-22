use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::cell::RefCell;
use std::rc::Rc;

use crate::stream::StreamPreClipNode;
use crate::stream::StreamPreClipNodeStub;
use crate::stream::StreamTransformNode;

use crate::{stream::Stream, TransformIdentity};

use crate::Transform;
pub struct StreamTransform<T: CoordFloat> {
    pub transform: Rc<Box<dyn Transform<C = Coordinate<T>>>>,
    pub stream: StreamPreClipNode<T>,
}

pub trait StreamPreclipIn<T>
where
    T: CoordFloat,
{
    fn stream_preclip_in(&mut self, stream: StreamPreClipNode<T>);
}

impl<T: CoordFloat> StreamPreclipIn<T> for StreamTransform<T> {
    #[inline]
    fn stream_preclip_in(&mut self, stream: StreamPreClipNode<T>) {
        self.stream = stream;
    }
}

// pub type StreamTransformNode<T> = Rc<RefCell<Box<StreamTransform<T>>>>;
impl<T: CoordFloat + FloatConst + std::default::Default + 'static> StreamTransform<T> {
    #[inline]
    pub fn gen_node(
        transform: Option<Rc<Box<dyn Transform<C = Coordinate<T>>>>>,
    ) -> StreamTransformNode<T> {
        {
            match transform {
                Some(transform) => Rc::new(RefCell::new(Self {
                    transform: transform.clone(),
                    stream: StreamPreClipNodeStub::new(),
                })),
                None => Rc::new(RefCell::new(Self {
                    transform: Rc::new(Box::new(TransformIdentity::<T>::default())),
                    stream: StreamPreClipNodeStub::new(),
                })),
            }
        }
    }
}

impl<T: CoordFloat + FloatConst> Stream for StreamTransform<T> {
    type C = Coordinate<T>;
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        let mut stream = self.stream.borrow_mut();
        let rotate = self.transform.clone();
        let r = rotate.transform(&p);
        // Warning the javascript version return the value below but I think it break the implied spec!!!!
        stream.point(r, m);
    }
}
