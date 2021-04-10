use std::collections::VecDeque;
use std::default::Default;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use web_sys::CanvasRenderingContext2d;

use crate::clip::buffer::LineElem;
use crate::path::PathResult;
use crate::projection::projection_mutator::ProjectionMutator;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::{data_object::DataObject, path::area_stream::PathAreaStream};

use super::PathResultEnum;
use super::PointRadiusEnum;
use super::PointRadiusTrait;

pub struct Path<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    context: Option<CanvasRenderingContext2d>,
    context_stream: Option<Box<dyn PointRadiusTrait<PrtT = T>>>,
    point_radius: PointRadiusEnum<T>,
    projection_stream: Option<Box<dyn Fn(StreamDst<T>) -> StreamTransformRadians<T>>>,
    pm: ProjectionMutator<T>,
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
            point_radius: PointRadiusEnum::Val(T::from(4.5).unwrap()),
            pm,
            context,
            context_stream: None,
            projection_stream: None,
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
            Some(ref _c) => {
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
}
