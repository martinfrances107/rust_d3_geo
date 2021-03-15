mod area_stream;
mod context;
mod string;

use std::default::Default;

use crate::projection::stream_transform_radians::StreamTransformRadians;
// use crate::projection::stream_transform_radians::StreamTransformRadiansNode;
// use crate::stream::stream_identity::StreamIdentity;

use crate::clip::buffer::LineElem;
// use crate::stream::stream_dummy::StreamDummy;
// use crate::stream::Streamable;
use crate::{data_object::DataObject, path::area_stream::PathAreaStream};
use geo::Coordinate;
use web_sys::CanvasRenderingContext2d;

use crate::projection::projection_mutator::ProjectionMutator;
use crate::stream::Stream;

use geo::CoordFloat;
use num_traits::{AsPrimitive, FloatConst};

#[derive(Clone)]
pub enum PathResultEnum<T>
where
    T: CoordFloat,
{
    Path(Vec<Vec<Coordinate<T>>>),
    ClipBufferOutput(Vec<Vec<LineElem<T>>>),
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

// #[inline]
// fn projection_stream_identity<T>(_path: dyn  PathStreamTrait<T>  + 'static) -> Box<dyn Stream<T>>
// where T: Float {
//     Box::new(StreamIdentity{})
// }

// pub struct PathIdentity{}

// impl<T> PathTrait<T> for PathIdentity
// where T: Float {

// }

trait PathStreamTrait: Stream + PathTrait + PathResult {}

pub struct Path<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    context: Option<CanvasRenderingContext2d>,
    context_stream: Option<Box<dyn PointRadiusTrait<PrtT = T>>>,
    point_radius: PointRadiusEnum<T>,
    projection: Option<ProjectionMutator<T>>,
}

fn projection_stream_noop() {}
impl<T> Default for Path<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    fn default() -> Self {
        Self {
            context: None,
            context_stream: None,
            point_radius: PointRadiusEnum::Val(T::from(4.5f64).unwrap()),
            projection: None,
            // projection_stream: Box::new(|_| StreamTransformRadians::gen_node()),
        }
    }
}

impl<T> Path<T>
where
    T: CoordFloat + std::fmt::Display + FloatConst + std::ops::AddAssign + Default + 'static,
{
    #[inline]
    fn generate(
        projection: Option<ProjectionMutator<T>>,
        context: Option<CanvasRenderingContext2d>,
    ) -> Path<T> {
        Path {
            projection,
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
    pub fn area(&self, _d: &DataObject<T>) -> T
    where
        T: CoordFloat + FloatConst,
    {
        let pa: PathAreaStream<T> = PathAreaStream::default();
        // d.to_stream(&mut (self.projection_stream_fn)(pa))
        T::zero()
    }

    // fn set_projection(&mut self, ps: Option<Box<dyn Transform<>>>) {
    //     self.projection_in = ps;
    //     self.projection_stream_fn = None;
    // }

    pub fn projection<'a>(p_in: Option<ProjectionMutator<T>>) -> Path<T>
    where
        T: CoordFloat + FloatConst,
    {
        let projection: Option<ProjectionMutator<T>>;
        let projection_stream: Box<
            dyn Fn(Box<dyn Stream<C = Coordinate<T>>>) -> StreamTransformRadians<T>,
        >;

        //  let ret =  arguments.length ? (projectionStream = _ == null ? (projection = null, identity) : (projection = _).stream, path) : projection;

        // match p_in {
        //     None => {
        //         projection = None;
        //         projection_stream = Box::new(|_| StreamTransformRadians::gen_node());
        //     }
        //     Some(mut p_in) => {
        //         {
        //             projection_stream = p_in.stream();
        //         }
        //         {
        //             projection = Some(p_in);
        //         }
        //     }
        // }

        return Path {
            // projection,
            // projection_stream,
            ..Default::default()
        };
    }

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

    #[inline]
    fn generate_path(
        projection: Option<ProjectionMutator<T>>,
        context: Option<CanvasRenderingContext2d>,
    ) -> Path<T>
    where
        T: CoordFloat + std::fmt::Display + AsPrimitive<T>,
    {
        let mut ret = Path::projection(projection);
        ret.context(context);
        ret
    }
}
