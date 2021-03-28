use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;
use std::ops::AddAssign;

use crate::clip::antimeridian::ClipAntimeridian;
use crate::clip::clip::Clip;
use crate::clip::ClipRaw;
use crate::compose::Compose;
// use crate::stream::CompareIntersection;
use crate::stream::Stream;
// use crate::stream::StreamClone;
use crate::stream::StreamDst;
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
    project: Compose<T>,
    /// Box to prevent infinite recusion.
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

impl<T: CoordFloat + FloatConst + Default> ResampleNone<T> {
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
    #[inline]
    pub fn stream_in(&mut self, stream: Clip<T>) {
        self.stream = Box::new(stream);
    }
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
}

impl<T: AddAssign + CoordFloat + FloatConst + Default> Stream<T> for ResampleNone<T> {
    type C = Coordinate<T>;
    fn get_dst(&self) -> StreamDst<T> {
        self.stream.get_dst()
    }

    fn sphere(&mut self) {
        self.stream.sphere()
    }
    fn line_start(&mut self) {
        self.stream.line_start()
    }
    fn line_end(&mut self) {
        self.stream.line_end()
    }
    fn polygon_start(&mut self) {
        self.stream.polygon_start()
    }
    fn polygon_end(&mut self) {
        self.stream.polygon_end()
    }
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        let p = p.clone();
        let project = &self.project;
        let t = project.transform(&p);
        self.stream.point(&t, m);
    }
}
