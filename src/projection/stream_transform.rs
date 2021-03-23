// use crate::stream::CompareIntersection;

use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// use crate::stream::StreamPreClipTrait;
// use crate::stream::StreamPreClipNode;
// use crate::stream::stream_dummy::StreamDummy;
// use crate::stream::stream_preclip_node_stub::StreamPreClipNodeStub;
// use crate::stream::StreamTransformNode;
// use crate::projection::resample::ResampleEnum;
use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::clip::Clip;
use crate::rotation::rotate_radians_transform::RotateRadiansEnum;
use crate::rotation::rotation_identity::RotationIdentity;
use crate::stream::Stream;
use crate::stream::StreamClone;
use crate::Transform;
use crate::TransformClone;
use crate::TransformIdentity;

pub struct StreamTransform<T: CoordFloat + FloatConst + Default> {
    pub transform: RotateRadiansEnum<T>,
    // pub transform: Box<dyn Transform<'a, TcC = Coordinate<T>>>,
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

impl<T> Default for StreamTransform<T>
where
    T: CoordFloat + FloatConst + Default,
{
    fn default() -> Self {
        Self {
            transform: RotateRadiansEnum::I(RotationIdentity::default()),
            // transform: Box::new(TransformIdentity::<T>::default()),
            // stream: Box::new(StreamPreClipNodeStub::<T>::default()),
            stream: ClipAntimeridian::gen_clip(),
        }
    }
}

// pub trait StreamPreclipIn<T>
// where
//     T: CoordFloat + FloatConst + Default,
// {
//     fn stream_preclip_in(
//         &mut self,
//         stream: Clip<T>,
//         // stream: Box<
//         //     dyn StreamPreClipTrait<
//         //         // C = Coordinate<T>,
//         //         SctC = Coordinate<T>,
//         //         SctT = T,
//         //         SctOC = Option<Coordinate<T>>,
//         //         SctStream = Box<dyn Stream<C = Coordinate<T>>>,
//         //         SctCi = CompareIntersection<T>,
//         //         SpctResample = ResampleEnum<T>,
//         //     >,
//         // >,
//     );
// }

impl<T: CoordFloat + FloatConst + Default> StreamTransform<T> {
    #[inline]
    pub fn stream_preclip_in(
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

impl<T: CoordFloat + FloatConst + Default> StreamTransform<T> {
    #[inline]
    pub fn new(
        // transform_in: Option<Box<dyn TransformClone<'a, TcC = Coordinate<T>>>>,
        transform_in: Option<RotateRadiansEnum<T>>,
    ) -> StreamTransform<T> {
        {
            let transform: RotateRadiansEnum<T>;
            // let transform: Box<dyn TransformClone<'a, TcC = Coordinate<T>>>;
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
                    transform = t.clone();
                }
                None => {
                    // transform = Box::new(TransformIdentity::<T>::default());
                    transform = RotateRadiansEnum::I(RotationIdentity::<T>::default());
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

// impl<T: CoordFloat + FloatConst + AddAssign> StreamClone for CentroidStream<T> {
//     // type C = Coordinate<T>;
//     type RetType = Box<dyn Stream<C = Coordinate<T>>>;
//     fn box_clone(&self) -> Self::RetType {
//         Box::new(self.clone())
//     }
// }

// impl<'a, T: CoordFloat + FloatConst + Default> TransformClone<'a>
//     for StreamTransform<'a, T>
// {
//     fn box_clone(&'a self) -> Box<dyn TransformClone<'a, TcC = Self::TcC>> {
//         Box::new(Self {
//             transform: self.transform.box_clone(),
//             stream: self.stream.clone(),
//         })
//     }
// }

impl<T: CoordFloat + FloatConst + Default> Transform for StreamTransform<T> {
    type TcC = Coordinate<T>;
    fn transform(&self, p: &Self::TcC) -> Self::TcC {
        // self.transform(p)
        self.transform.transform(p)
    }
    fn invert(&self, p: &Self::TcC) -> Self::TcC {
        self.transform.invert(p)
    }
}

// impl<T: CoordFloat + FloatConst + Default> StreamClone for StreamTransform<T> {
//     type RetType = Box<dyn Stream<C = Coordinate<T>>>;
//     #[inline]
//     fn box_clone(&self) -> Self::RetType {
//         Box::new(Self {
//             stream: self.stream.clone(),
//             transform: self.transform,
//         })
//     }
// }

impl<T: CoordFloat + FloatConst + Default> Stream for StreamTransform<T> {
    type C = Coordinate<T>;
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        // let mut stream = self.stream.borrow_mut();
        // let rotate = self.transform.box_clone();
        // let r = self.transform(&p);
        // Warning the javascript version return the value below but I think it break the implied spec!!!!
        self.stream.point(&self.transform(&p), m);
    }
}
