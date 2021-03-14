use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::clip::Clip;
use crate::clip::ClipRaw;
use crate::stream::CompareIntersection;
use crate::stream::Stream;
use crate::stream::StreamClone;
use crate::stream::StreamSrc;
// use crate::stream::StreamClone;
// use super::StreamResampleTrait;
// use crate::stream::stream_postclip_node_stub::StreamPostClipNodeStub;
// use crate::stream::StreamPostClipTrait;
use crate::Transform;

pub struct ResampleNone<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    project: Box<dyn Transform<TcC = Coordinate<T>>>,
    // stream: Box<
    //     dyn StreamPostClipTrait<
    //         SpostctStream = StreamSrc<T>,
    //         C = Coordinate<T>,
    //         SctC = Coordinate<T>,
    //         SctT = T,
    //         SctOC = Option<Coordinate<T>>,
    //         SctCi = CompareIntersection<T>,
    //         SctStream = Box<dyn Stream<C = Coordinate<T>>>,
    //     >,
    // >,
    pub stream: Box<Clip<T>>,
}

impl<T> Clone for ResampleNone<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    fn clone(&self) -> Self {
        Self {
            project: self.project.box_clone(),
            stream: self.stream.clone(),
        }
    }
}

impl<T: CoordFloat + FloatConst + Default + 'static> ResampleNone<T> {
    #[inline]
    pub fn new(project: Box<dyn Transform<TcC = Coordinate<T>>>) -> Self {
        Self {
            project: project.box_clone(),
            stream: Box::new(Clip::new(
                ClipRaw::Antimeridian(ClipAntimeridian::default()),
                Coordinate::default(),
            )), // stub value
        }
    }
}

// impl<T: CoordFloat + FloatConst + Default + 'static> StreamResampleTrait for ResampleNone<T> {
//     type SRTsci = Box<
//         dyn StreamPostClipTrait<
//             SpostctStream = StreamSrc<T>,
//             C = Coordinate<T>,
//             SctC = Coordinate<T>,
//             SctT = T,
//             SctOC = Option<Coordinate<T>>,
//             SctCi = CompareIntersection<T>,
//             SctStream = Box<dyn Stream<C = Coordinate<T>>>,
//         >,
//     >;
//     fn stream_postclip_in(
//         &mut self,
//         stream: Box<
//             dyn StreamPostClipTrait<
//                 SpostctStream = StreamSrc<T>,
//                 C = Coordinate<T>,
//                 SctC = Coordinate<T>,
//                 SctT = T,
//                 SctOC = Option<Coordinate<T>>,
//                 SctCi = CompareIntersection<T>,
//                 SctStream = Box<dyn Stream<C = Coordinate<T>>>,
//             >,
//         >,
//     ) {
//         self.stream = stream;
//     }
// }

// impl<T: CoordFloat + FloatConst + 'static> StreamClone for ResampleNone<T> {
//     type RetType = Box<dyn Stream<C = Coordinate<T>>>;
//     #[inline]
//     fn box_clone(&self) -> Self::RetType {
//         Box::new(self.clone())
//     }
// }
impl<T: CoordFloat + FloatConst + Default + 'static> Stream for ResampleNone<T> {
    type C = Coordinate<T>;
    fn point(&mut self, p: Self::C, m: Option<u8>) {
        // let mut s = self.stream.borrow_mut();
        let project = &*self.project;
        let t = project.transform(&p);
        self.stream.point(t, m);
    }
}
