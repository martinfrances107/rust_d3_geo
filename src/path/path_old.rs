use std::cell::RefCell;
use std::default::Default;
use std::marker::PhantomData;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use web_sys::CanvasRenderingContext2d;

use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::data_object::DataObject;
use crate::path::Result;
use crate::projection::projection::Projection;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

use super::context::Context as PathContext;
use super::context_stream::ContextStream as PathContextStream;
use super::PointRadiusEnum;
use super::ResultEnum;

#[derive(Debug)]
pub struct Path<'a, CS, PR, T>
where
    PR: ProjectionRaw<T>,
    T: CoordFloat + FloatConst,
    CS: Stream<T = T> + Default,
{
    pd: PhantomData<&'a u8>,
    context: Option<CanvasRenderingContext2d>,
    context_stream: Rc<RefCell<CS>>,
    point_radius: PointRadiusEnum<T>,
    projection_stream: Option<PathContextStream<T>>,
    projection: Option<Projection<CS, Line<T>, PR, PV<T>, T>>,
}

impl<'a, CS, PR, T> Default for Path<'a, CS, PR, T>
where
    PR: ProjectionRaw<T>,
    T: CoordFloat + FloatConst,
    CS: Stream<T = T> + Default,
{
    fn default() -> Self {
        Self {
            // pd: PhantomData::<PR>::new(),
            pd: PhantomData::<&u8>,
            context: None,
            context_stream: Rc::new(RefCell::new(CS::default())),
            point_radius: PointRadiusEnum::Val(T::zero()),
            projection_stream: None,
            projection: None,
        }
    }
}

impl<'a, CS, PR, T> Path<'a, CS, PR, T>
where
    CS: StreamDrain<T> + Result,
    PR: ProjectionRaw<T>,
    T: AsPrimitive<T> + CoordFloat + FloatConst,
{
    #[inline]
    pub fn generate(
        projection: Option<Projection<CS, Line<T>, PR, PV<T>, T>>,
        context: Option<CanvasRenderingContext2d>,
    ) -> Self {
        println!("path generate");
        Self::default()
            .projection_fn(projection)
            .context_fn(context)
    }

    pub fn object(&mut self, object: Option<DataObject<T>>) -> Option<ResultEnum<T>> {
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
                        let mut stream_in = projection.stream(self.context_stream);
                        object.to_stream(&mut stream_in);
                        // let end_point = stream_in.get_dst();
                        // match end_point {
                        //     StreamDst::PathString(mut pas) => pas.result(),
                        //     _ => {
                        //         panic!("Did no get the expected PathString.");
                        //     }
                        // }
                        self.context_stream.borrow().result()
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

    // #[inline]
    // pub fn area(&self, object: &DataObject<T>) -> Option<ResultEnum<T>>
    // where
    //     T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst + Float,
    //     // TODO Is this a bodge ... can I place this higher up?
    //     <P as ProjectionTrait<'a>>::T:
    //         AsPrimitive<T> + AddAssign + std::fmt::Display + Float + FloatConst,
    //     <P as ProjectionTrait<'a>>::T: AsPrimitive<<P as ProjectionTrait<'a>>::T>,
    //     <P as ProjectionTrait<'a>>::SD:
    //         Stream<SC = Coordinate<<P as ProjectionTrait<'a>>::T>> + Default,
    // {
    //     let mut stream_dst = PathAreaStream::default();
    //     match &self.projection {
    //         Some(projection) => {
    //             let mut stream_in = projection.stream(&mut stream_dst);
    //             object.to_stream(&mut stream_in);
    //             // let end_point = stream_in.get_dst();

    //             // match end_point {
    //             //     StreamDst::PAS(mut pas) => pas.result(),
    //             //     _ => panic!("unexpected end_point"),
    //             // }
    //             stream_dst.result()
    //         }
    //         None => None,
    //     }
    // }

    // fn projection_fn(mut self, projection: Option<Projection<CS, Line<T>, PR, PV<T>, T>>) -> Self {
    //     match projection {
    //         None => {
    //             self.projection = None;
    //             // self.projection_stream = Identity; // A stream that is pass through?
    //             // Self::default()
    //             todo!();
    //         }
    //         Some(projection) => {
    //             self.projection = Some(projection);
    //             // Warm the cache before storage.
    //             self.projection_stream = Some(projection.stream());
    //             self
    //         }
    //     }
    // }

    //     // pub fn projection(p_in: Option<ProjectionMutator<PR, T>>) -> Path<T>
    //     // where
    //     //     T: CoordFloat + FloatConst,
    //     // {
    //     //     let projection: Option<ProjectionMutator<PR, T>>;
    //     //     let projection_stream: Box<
    //     //         dyn Fn(Box<dyn Stream>) -> StreamTransformRadians<T>,
    //     //     >;

    //     //     Path {
    //     //         ..Default::default()
    //     //     }
    //     // }

    //     // #[inline]
    //     // fn get_context(&self) -> Option<Box<dyn PointRadiusTrait<PrtT=T>>> {
    //     //     self.context_stream.as_ref().unwrap()
    //     // }

    // fn context_fn(mut self, c_in: Option<CanvasRenderingContext2d>) -> Self {
    //     match c_in {
    //         None => {
    //             self.context = None;
    //             self.context_stream = Rc::new(RefCell::new(CS::default()));
    //         }
    //         Some(c) => {
    //             self.context = Some(c.clone());
    //             self.context_stream = Rc::new(RefCell::new(PathContext::new(c)));
    //         }
    //     }
    //     match &self.point_radius {
    //         PointRadiusEnum::F(_pr) => {
    //             // do nothing.
    //         }
    //         PointRadiusEnum::Val(pr) => {
    //             self.context_stream.borrow().point_radius(Some(*pr));
    //         }
    //     }
    //     self
    // }

    //     #[inline]
    //     fn get_point_radius(&self) -> PointRadiusEnum<T> {
    //         self.point_radius
    //     }

    //     #[inline]
    //     fn point_radius(mut self, input: PointRadiusEnum<T>) -> Self {
    //         self.point_radius = match input {
    //             PointRadiusEnum::F(ref _input_fn) => input,
    //             PointRadiusEnum::Val(input_value) => {
    //                 // match &mut self.context_stream {
    //                 //     PathContextStream::PS(ps) => {
    //                 //         ps.point_radius(Some(input_value));
    //                 //     }
    //                 //     PathContextStream::PC(pc) => {
    //                 //         pc.point_radius(Some(input_value));
    //                 //     }
    //                 // }
    //                 self.context_stream.point_radius(Some(input_value));
    //                 input
    //             }
    //         };
    //         self
    //     }
}
