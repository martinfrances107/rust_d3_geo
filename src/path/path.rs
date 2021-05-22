use std::default::Default;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use web_sys::CanvasRenderingContext2d;

use crate::path::path_context_stream::PathContextStream;
use crate::path::path_string::PathString;
use crate::path::PathResult;
use crate::projection::projection_mutator::ProjectionMutator;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::{data_object::DataObject, path::path_area_stream::PathAreaStream};

use super::path_context::PathContext;
use super::PathResultEnum;
use super::PointRadiusEnum;
use super::PointRadiusTrait;

pub struct Path<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    context: Option<CanvasRenderingContext2d>,
    context_stream: PathContextStream<T>,
    point_radius: PointRadiusEnum<T>,
    projection_stream: Option<PathContextStream<T>>,
    projection: Option<ProjectionMutator<T>>,
}

impl<T> Default for Path<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    fn default() -> Self {
        Self {
            context: None,
            context_stream: PathContextStream::PS(PathString::<T>::default()),
            point_radius: PointRadiusEnum::Val(T::zero()),
            projection_stream: None,
            projection: None,
        }
    }
}

impl<T> Path<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    #[inline]
    pub fn generate(
        projection: Option<ProjectionMutator<T>>,
        context: Option<CanvasRenderingContext2d>,
    ) -> Self {
        println!("path genrate");
        Self::default().projection(projection).context(context)
    }

    pub fn object(&mut self, object: Option<DataObject<T>>) -> Option<PathResultEnum<T>> {
        match object {
            Some(object) => {
                // match self.point_radius{
                //     Some(point_radius) => {
                //         let radius = (self.point_radius)(self,object);
                //         context_stream.point_radius(radius)
                //     }
                //     None => {

                //     }
                // }
                match &self.projection {
                    Some(projection) => {
                        let mut stream_in = projection
                            .stream(StreamDst::PathContextStream(self.context_stream.clone()));
                        object.to_stream(&mut stream_in);
                        let end_point = stream_in.get_dst();
                        match end_point {
                            StreamDst::PathString(mut pas) => pas.result(),
                            _ => {
                                panic!("Did no get the expected PathString.");
                            }
                        }
                    }
                    None => {
                        panic!("How to handle no projection dropping projection.");
                    }
                }
            }
            None => {
                panic!("No object supplied.");
            }
        }

        // self.context_stream.result()
    }

    #[inline]
    pub fn area(&self, object: &DataObject<T>) -> Option<PathResultEnum<T>>
    where
        T: CoordFloat + FloatConst,
    {
        match &self.projection {
            Some(projection) => {
                let mut stream_in = projection.stream(StreamDst::PAS(PathAreaStream::default()));
                object.to_stream(&mut stream_in);
                let end_point = stream_in.get_dst();

                match end_point {
                    StreamDst::PAS(mut pas) => pas.result(),
                    _ => panic!("unexpected end_point"),
                }
            }
            None => None,
        }
    }

    fn projection(mut self, projection: Option<ProjectionMutator<T>>) -> Self {
        match projection {
            None => {
                self.projection = None;
                self.projection_stream = Some(PathContextStream::PS(PathString::default()));
                Self::default()
            }
            Some(projection) => {
                self.projection = Some(projection);
                self
            }
        }
    }

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
    // fn get_context(&self) -> Option<Box<dyn PointRadiusTrait<PrtT=T>>> {
    //     self.context_stream.as_ref().unwrap()
    // }

    fn context(mut self, c_in: Option<CanvasRenderingContext2d>) -> Self {
        match c_in {
            None => {
                self.context = None;
                self.context_stream = PathContextStream::PS(PathString::default());
            }
            Some(c) => {
                self.context = Some(c.clone());
                self.context_stream = PathContextStream::PC(PathContext::new(c));
            }
        }
        match &self.point_radius {
            PointRadiusEnum::F(_pr) => {
                // do nothing.
            }
            PointRadiusEnum::Val(pr) => {
                self.context_stream.point_radius(Some(*pr));
            }
        }
        self
    }

    // #[inline]
    // fn get_point_radius(&self) -> PointRadiusEnum<T> {
    //     self.point_radius
    // }

    #[inline]
    fn point_radius(mut self, input: PointRadiusEnum<T>) -> Self {
        self.point_radius = match input {
            PointRadiusEnum::F(ref _input_fn) => input,
            PointRadiusEnum::Val(input_value) => {
                match &mut self.context_stream {
                    PathContextStream::PS(ps) => {
                        ps.point_radius(Some(input_value));
                    }
                    PathContextStream::PC(pc) => {
                        pc.point_radius(Some(input_value));
                    }
                }
                input
            }
        };
        self
    }
}
