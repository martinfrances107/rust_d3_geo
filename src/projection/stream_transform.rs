use crate::stream::CompareIntersection;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::stream::StreamPreClipTrait;
// use crate::stream::StreamPreClipNode;
// use crate::stream::stream_dummy::StreamDummy;
use crate::stream::stream_preclip_node_stub::StreamPreClipNodeStub;
// use crate::stream::StreamTransformNode;
use crate::projection::resample::ResampleEnum;

use crate::clip::clip::Clip;
use crate::stream::Stream;
use crate::stream::StreamClone;
use crate::Transform;
use crate::TransformIdentity;

pub struct StreamTransform<T: CoordFloat + FloatConst + Default + 'static> {
    pub transform: Box<dyn Transform<TcC = Coordinate<T>>>,
    // pub stream: Box<
    //     dyn StreamPreClipTrait<
    //         // C = Coordinate<T>,
    //         SctC = Coordinate<T>,
    //         SctT = T,
    //         SctOC = Option<Coordinate<T>>,
    //         SctStream = Box<dyn Stream<C = Coordinate<T>>>,
    //         SctCi = CompareIntersection<T>,
    //         SpctResample = ResampleEnum<T>,
    //     >,
    // >,
    pub stream: Clip<T>,
}

use crate::clip::antimeridian::ClipAntimeridian;
impl<T> Default for StreamTransform<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    fn default() -> Self {
        Self {
            transform: Box::new(TransformIdentity::<T>::default()),
            // stream: Box::new(StreamPreClipNodeStub::<T>::default()),
            stream: ClipAntimeridian::gen_clip(),
        }
    }
}

pub trait StreamPreclipIn<T>
where
    T: CoordFloat + FloatConst + Default,
{
    fn stream_preclip_in(
        &mut self,
        stream: Clip<T>,
        // stream: Box<
        //     dyn StreamPreClipTrait<
        //         // C = Coordinate<T>,
        //         SctC = Coordinate<T>,
        //         SctT = T,
        //         SctOC = Option<Coordinate<T>>,
        //         SctStream = Box<dyn Stream<C = Coordinate<T>>>,
        //         SctCi = CompareIntersection<T>,
        //         SpctResample = ResampleEnum<T>,
        //     >,
        // >,
    );
}

impl<T: CoordFloat + FloatConst + Default> StreamPreclipIn<T> for StreamTransform<T> {
    #[inline]
    fn stream_preclip_in(
        &mut self,
        // stream: Box<
        //     dyn StreamPreClipTrait<
        //         SctC = Coordinate<T>,
        //         SctT = T,
        //         SctOC = Option<Coordinate<T>>,
        //         SctStream = Box<dyn Stream<C = Coordinate<T>>>,
        //         SctCi = CompareIntersection<T>,
        //         SpctResample = ResampleEnum<T>,
        //     >,
        // >,
        stream: Clip<T>,
    ) {
        self.stream = stream;
    }
}

impl<T: CoordFloat + FloatConst + Default + 'static> StreamTransform<T> {
    #[inline]
    pub fn new(
        transform_in: Option<Box<dyn Transform<TcC = Coordinate<T>>>>,
    ) -> StreamTransform<T> {
        {
            let transform: Box<dyn Transform<TcC = Coordinate<T>>>;
            // let stream: Box<
            //     dyn StreamPreClipTrait<
            //         SctC = Coordinate<T>,
            //         SctT = T,
            //         SctOC = Option<Coordinate<T>>,
            //         SctStream = Box<dyn Stream<C = Coordinate<T>>>,
            //         SctCi = CompareIntersection<T>,
            //         SpctResample = ResampleEnum<T>,
            //     >,
            // >;

            match transform_in {
                Some(t) => {
                    transform = t.box_clone();
                }
                None => {
                    transform = Box::new(TransformIdentity::<T>::default());
                }
            }
            let stream = ClipAntimeridian::gen_clip();
            Self { transform, stream }
        }
    }
}

// impl<T: CoordFloat + FloatConst + 'static> Clone for StreamTransform<T> {
//     #[inline]
//     fn clone(&self) -> Self {
//         Self {
//             stream: self.stream.box_clone(),
//             transform: self.transform.box_clone(),
//         }
//     }
// }

impl<T: CoordFloat + FloatConst + Default + 'static> StreamClone for StreamTransform<T> {
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(Self {
            stream: self.stream.clone(),
            transform: self.transform.box_clone(),
        })
    }
}

impl<T: CoordFloat + FloatConst + Default + 'static> Stream for StreamTransform<T> {
    type C = Coordinate<T>;
    // fn point(&mut self, p: Self::C, m: Option<u8>) {
    //     // let mut stream = self.stream.borrow_mut();
    //     let rotate = self.transform.box_clone();
    //     let r = rotate.transform(&p);
    //     // Warning the javascript version return the value below but I think it break the implied spec!!!!
    //     self.stream.point(r, m);
    // }
}
