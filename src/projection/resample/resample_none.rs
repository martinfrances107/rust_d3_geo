use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::stream::CompareIntersection;
use crate::stream::Stream;
use crate::stream::StreamClone;
// use crate::stream::StreamClone;
use super::StreamResampleTrait;
use crate::stream::StreamPostClipTrait;
use crate::stream::{StreamNodeStub, StreamSimpleNode};
use crate::Transform;

pub struct ResampleNone<T>
where
    T: CoordFloat,
{
    project: Box<dyn Transform<TcC = Coordinate<T>>>,
    stream: Box<dyn Stream<ScC = Coordinate<T>>>,
}

impl<T> Clone for ResampleNone<T>
where
    T: CoordFloat,
{
    fn clone(&self) -> Self {
        Self {
            project: self.project.clone_box(),
            stream: self.stream.clone_box(),
        }
    }
}

impl<T: CoordFloat + FloatConst + Default + 'static> ResampleNone<T> {
    #[inline]
    pub fn new(project: Box<dyn Transform<TcC = Coordinate<T>>>) -> Self {
        Self {
            project: project.clone_box(),
            stream: StreamNodeStub::new(),
        }
    }
}

impl<T: CoordFloat + FloatConst + Default + 'static> StreamResampleTrait for ResampleNone<T> {
    type SRTsci = Box<
        dyn StreamPostClipTrait<
            ScC = Coordinate<T>,
            SctT = T,
            SctOC = Option<Coordinate<T>>,
            SctCi = CompareIntersection<T>,
            SctStream = Box<dyn Stream<ScC = Coordinate<T>>>,
        >,
    >;
    fn stream_postclip_in(
        &mut self,
        _stream_in: Box<
            dyn StreamPostClipTrait<
                ScC = Coordinate<T>,
                SctT = T,
                SctOC = Option<Coordinate<T>>,
                SctCi = CompareIntersection<T>,
                SctStream = Box<dyn Stream<ScC = Coordinate<T>>>,
            >,
        >,
    ) {
    }
}

impl<T: CoordFloat + FloatConst + 'static> StreamClone for ResampleNone<T> {
    type ScC = Coordinate<T>;
    #[inline]
    fn clone_box(&self) -> Box<dyn Stream<ScC = Coordinate<T>>> {
        Box::new(self.clone())
    }
}
impl<T: CoordFloat + FloatConst + 'static> Stream for ResampleNone<T> {
    fn point(&mut self, p: Coordinate<T>, m: Option<u8>) {
        // let mut s = self.stream.borrow_mut();
        let project = &*self.project;
        let t = project.transform(&p);
        self.stream.point(t, m);
    }
}
