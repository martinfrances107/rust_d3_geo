pub mod area_stream;
pub mod path;

mod context;
mod string;

use std::collections::VecDeque;
use std::default::Default;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use web_sys::CanvasRenderingContext2d;

use crate::clip::buffer::LineElem;
use crate::projection::projection_mutator::ProjectionMutator;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::{data_object::DataObject, path::area_stream::PathAreaStream};

#[derive(Clone)]
pub enum PathResultEnum<T>
where
    T: CoordFloat,
{
    Path(Vec<Vec<Coordinate<T>>>),
    ClipBufferOutput(VecDeque<Vec<LineElem<T>>>),
    Sring(String),
    Area(T),
    Measure(T),
    Bound(T),
    Centroid(T),
}

pub trait PathResult // where
{
    type Out;
    fn result(&mut self) -> Self::Out;
}

trait PointRadiusTrait {
    type PrtT;
    fn point_radius(&self, val: Self::PrtT);
}

// #[derive(Clone)]
enum PointRadiusEnum<T> {
    Val(T),
    F(Box<dyn Fn() -> T>),
}

trait PathTrait: PointRadiusTrait // where
//     T: CoordFloat + FloatConst,
{
    type PtDo;
    type PtPRE;
    fn area(&self, d: Self::PtDo) -> Option<String> {
        // Stream(d, self.projection_stream);
        // PathArea::result();
        None
    }
    fn measure(&self, d: Self::PtDo) -> Self::PtPRE;

    fn bound(&self, d: Self::PtDo) -> Self::PtPRE;

    fn centroid(&self, d: Self::PtDo) -> Self::PtPRE;

    fn projection(&self, d: Self::PtDo) -> Self::PtPRE;

    fn context_get(&self) -> CanvasRenderingContext2d;
    fn context(&self);
    // fn point_radius_get(&self);
    // fn point_radius_set(&self);
    // fn point_radius(&self);
    // fn result(&self);
}

// pub struct PathIdentity{}

// impl<T> PathTrait<T> for PathIdentity
// where T: Float {

// }

trait PathStreamTrait<T>: Stream<T> + PathTrait + PathResult
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
}
