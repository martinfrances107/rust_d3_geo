use crate::stream::CompareIntersection;
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::stream::StreamPreClipTrait;
// use crate::stream::StreamPreClipNode;
use crate::stream::StreamDummy;
use crate::stream::StreamPreClipNodeStub;
// use crate::stream::StreamTransformNode;
use crate::projection::resample::ResampleNode;

use crate::stream::Stream;
use crate::stream::StreamClone;
use crate::TransformIdentity;

use crate::Transform;

pub struct StreamTransform<T: CoordFloat + FloatConst + 'static> {
    pub transform: Box<dyn Transform<TcC = Coordinate<T>>>,
    pub stream: Box<
        dyn StreamPreClipTrait<
            ScC = Coordinate<T>,
            SctT = T,
            SctOC = Option<Coordinate<T>>,
            SctStream = Box<dyn Stream<ScC = Coordinate<T>>>,
            SctCi = CompareIntersection<T>,
            SpctResample = ResampleNode<T>,
        >,
    >,
}

impl<T> Default for StreamTransform<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    fn default() -> Self {
        Self {
            transform: Box::new(TransformIdentity::<T>::default()),
            stream: Box::new(StreamPreClipNodeStub::default()),
        }
    }
}

pub trait StreamPreclipIn<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_preclip_in(
        &mut self,
        stream: Box<
            dyn StreamPreClipTrait<
                ScC = Coordinate<T>,
                SctT = T,
                SctOC = Option<Coordinate<T>>,
                SctStream = Box<dyn Stream<ScC = Coordinate<T>>>,
                SctCi = CompareIntersection<T>,
                SpctResample = ResampleNode<T>,
            >,
        >,
    );
}

impl<T: CoordFloat + FloatConst> StreamPreclipIn<T> for StreamTransform<T> {
    #[inline]
    fn stream_preclip_in(
        &mut self,
        stream: Box<
            dyn StreamPreClipTrait<
                ScC = Coordinate<T>,
                SctT = T,
                SctOC = Option<Coordinate<T>>,
                SctStream = Box<dyn Stream<ScC = Coordinate<T>>>,
                SctCi = CompareIntersection<T>,
                SpctResample = ResampleNode<T>,
            >,
        >,
    ) {
        self.stream = stream;
    }
}

// pub type StreamTransformNode<T> = Rc<RefCell<Box<StreamTransform<T>>>>;
impl<T: CoordFloat + FloatConst + Default + 'static> StreamTransform<T> {
    #[inline]
    pub fn new(
        transform_in: Option<Box<dyn Transform<TcC = Coordinate<T>>>>,
    ) -> StreamTransform<T> {
        {
            let transform: Box<dyn Transform<TcC = Coordinate<T>>>;
            let stream: Box<
                dyn StreamPreClipTrait<
                    ScC = Coordinate<T>,
                    SctT = T,
                    SctOC = Option<Coordinate<T>>,
                    SctStream = Box<dyn Stream<ScC = Coordinate<T>>>,
                    SctCi = CompareIntersection<T>,
                    SpctResample = ResampleNode<T>,
                >,
            >;
            match transform_in {
                Some(t) => {
                    transform = t.clone_box();
                    stream = Box::new(StreamPreClipNodeStub::default());
                }
                None => {
                    transform = Box::new(TransformIdentity::<T>::default());
                    stream = Box::new(StreamPreClipNodeStub::default());
                }
            }
            Self { transform, stream }
        }
    }
}

impl<T: CoordFloat + FloatConst + 'static> StreamClone for StreamTransform<T> {
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(*self.clone())
    }
}

impl<T: CoordFloat + FloatConst + 'static> Stream for StreamTransform<T> {
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        // let mut stream = self.stream.borrow_mut();
        let rotate = self.transform.clone_box();
        let r = rotate.transform(&p);
        // Warning the javascript version return the value below but I think it break the implied spec!!!!
        self.stream.point(r, m);
    }
}
