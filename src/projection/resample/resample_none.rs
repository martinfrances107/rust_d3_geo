use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::clip::Clip;
use crate::clip::ClipRaw;
use crate::compose::Compose;
// use crate::stream::CompareIntersection;
use crate::stream::Stream;
// use crate::stream::StreamClone;
// use crate::stream::StreamDst;
// use crate::stream::StreamClone;
// use super::StreamResampleTrait;
// use crate::stream::stream_postclip_node_stub::StreamPostClipNodeStub;
// use crate::stream::StreamPostClipTrait;
use crate::Transform;
// use crate::TransformClone;

#[derive(Debug)]
pub struct ResampleNone<T>
where
    T: CoordFloat + FloatConst + Default,
{
    // project: Box<dyn TransformClone<'a, TcC = Coordinate<T>>>,
    project: Compose<T>,
    // stream: Box<
    //     dyn StreamPostClipTrait<
    //         SpostctStream = StreamDst<T>,
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
    T: CoordFloat + FloatConst + Default,
{
    fn clone(&self) -> Self {
        Self {
            project: self.project.clone(),
            stream: self.stream.clone(),
        }
    }
}

impl<T: CoordFloat + FloatConst + Default> ResampleNone<T> {
    #[inline]
    pub fn new(project: Compose<T>) -> Self {
        Self {
            project: project,
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
//             SpostctStream = StreamDst<T>,tatic + CoordFloat> Clone for Compose<'a, T> {
//     fn clone(&self) -> Compose<'a, T> {
//         Compose::<'a, T> {
//             a: self.a.box_clone(),
//             b: self.b.box_clone(),
//         }
//     }
// }
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
//                 SpostctStream = StreamDst<T>,
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
impl<T: CoordFloat + FloatConst + Default> Stream for ResampleNone<T> {
    type C = Coordinate<T>;
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        let p = p.clone();
        let project = &self.project;
        let t = project.transform(&p);
        self.stream.point(&t, m);
    }
}
