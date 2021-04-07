pub mod area_stream;

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

pub struct Path<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    context: Option<CanvasRenderingContext2d>,
    context_stream: Option<Box<dyn PointRadiusTrait<PrtT = T>>>,
    point_radius: PointRadiusEnum<T>,
    projection_stream: Box<dyn Fn(StreamDst<T>) -> StreamTransformRadians<T>>,
    pm: ProjectionMutator<T>,
}

fn projection_stream_noop() {}
impl<T> Default for Path<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    #[inline]
    fn default() -> Self {
        Self {
            context: None,
            context_stream: None,
            point_radius: PointRadiusEnum::Val(T::from(4.5f64).unwrap()),
            pm: ProjectionMutator::default(),
            projection_stream: Box::new(|_| StreamTransformRadians::default()),
        }
    }
}

impl<T> Path<T>
where
    T: AddAssign + CoordFloat + Default + Display + FloatConst,
{
    #[inline]
    pub fn generate(
        pm: ProjectionMutator<T>,
        context: Option<CanvasRenderingContext2d>,
    ) -> Path<T> {
        Path {
            pm,
            context,
            ..Default::default()
        }
    }

    // #[inline]
    // fn path(&self, object: Option<DataObject<T>>) -> PathResultEnum<T> {
    //     match object{
    //         Some(object) => {
    //             match self.point_radius{
    //                 Some(point_radius) => {
    //                     let radius = (self.point_radius)(self,object);
    //                     context_stream.point_radius(radius)
    //                 }
    //                 None => {

    //                 }
    //             }
    //         }
    //         None => {
    //         }
    //     }

    //     self.context_stream.unwrap().result()
    // }

    #[inline]
    pub fn area(&self, object: &DataObject<T>) -> Option<PathResultEnum<T>>
    where
        T: CoordFloat + FloatConst,
    {
        let mut stream_in = self.pm.stream(StreamDst::PAS(PathAreaStream::default()));
        object.to_stream(&mut stream_in);
        let end_point = stream_in.get_dst();

        match end_point {
            StreamDst::PAS(mut pas) => pas.result(),
            _ => panic!("unexpected end_point"),
        }
    }

    // fn set_projection(&mut self, projection: Option<ProjectionMutator<T>>) {
    //     match projection {
    //         None => {
    //             self.pm = ProjectionMutator::default();
    //             self.projection_stream = Box::new(|_| StreamTransformRadians::default());
    //         }
    //         Some(projection) => {
    //             self.pm = projection;
    //             // self.projection_stream = projection.stream();
    //         }
    //     }
    // }

    // pub fn projection(p_in: Option<ProjectionMutator<T>>) -> Path<T>
    // where
    //     T: CoordFloat + FloatConst,
    // {
    //     let projection: Option<ProjectionMutator<T>>;
    //     let projection_stream: Box<
    //         dyn Fn(Box<dyn Stream<T, C = Coordinate<T>>>) -> StreamTransformRadians<T>,
    //     >;

    //     Path {
    //         ..Default::default()
    //     }
    // }

    // #[inline]
    // fn get_context(&self) -> Option<Box<dyn PointRadiusTrait<T>>> {
    //     self.context_stream
    // }

    fn context(&mut self, c_in: Option<CanvasRenderingContext2d>) {
        match c_in {
            None => {
                self.context = None;
                // self.context_stream = Some(Box::new(PathString::new()));
            }
            Some(ref c) => {
                self.context = c_in;
                // self.context_stream = Some(Box::new(PathContext::new(c)));
            }
        }
        match &self.point_radius {
            PointRadiusEnum::F(_pr) => {
                // do nothing.
            }
            PointRadiusEnum::Val(pr) => {
                if self.context_stream.is_some() {
                    self.context_stream.as_ref().unwrap().point_radius(*pr);
                }
            }
        }
        // self
    }

    // #[inline]
    // fn get_point_radius(&self) -> PointRadiusEnum<T> {
    //     self.point_radius
    // }

    #[inline]
    fn point_radius(mut self, input: PointRadiusEnum<T>) {
        match input {
            PointRadiusEnum::F(ref _input_fn) => {
                self.point_radius = input;
            }
            PointRadiusEnum::Val(input_val) => {
                if self.context_stream.is_some() {
                    self.context_stream.unwrap().point_radius(input_val);
                }
                self.point_radius = input;
            }
        }
        // self
    }

    // #[inline]
    // pub fn generate_path(
    //     projection: ProjectionMutator<T>,
    //     context: Option<CanvasRenderingContext2d>,
    // ) -> Path<T>
    // where
    //     T: CoordFloat + std::fmt::Display + AsPrimitive<T>,
    // {
    //     let mut ret = Path::generate(projection, None);
    //     ret.context(context);
    //     ret
    // }
}
