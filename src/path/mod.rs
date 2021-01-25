mod area_stream;
mod context;
mod string;

use super::data_object::DataObject;
use crate::path::area_stream::PathAreaStream;
use crate::path::string::PathString;
use crate::transform_stream::StreamIdentity;
use crate::Transform;

use crate::path::context::PathContext;

use crate::projection::projection_mutator::ProjectionMutator;
use crate::stream::Stream;

use geo::CoordFloat;
use num_traits::{AsPrimitive, FloatConst};
use std::default::Default;

use web_sys::CanvasRenderingContext2d;

enum PathResultEnum<T>
where
    T: CoordFloat,
{
    Blank,
    Path(),
    Area(T),
    Measure(),
    Bound(),
    Centroid(),
}
pub trait PathResult<T>
where
    T: CoordFloat,
{
    fn result(&self) -> PathResultEnum<T>;
}

trait PathTrait<T>
where
    T: CoordFloat,
{
    // fn area(&self, d: impl DataObject<T>) -> Option<String> {
    //     // Stream(d, self.projection_stream);
    //     // PathArea::result();
    // };
    // fn measure(&self, d: impl DataObject<T>) -> PathResultEnum<T>
    // where
    //     T: Float;
    // fn bound(&self, d: impl DataObject<T>) -> PathResultEnum<T>
    // where
    //     T: Float;
    // fn centroid(&self, d: impl DataObject<T>) -> PathResultEnum<T>
    // where
    //     T: Float;
    // fn projection(&self, d: impl DataObject<T>) -> PathResultEnum<T>
    // where
    //     T: Float;
    fn context_get(&self) -> CanvasRenderingContext2d;
    fn context(&self);
    fn point_radius_get(&self);
    fn point_radius_set(&self);
    fn point_radius(&self);
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

trait PathStreamTrait<T>: Stream<T> + PathTrait<T> + PathResult<T>
where
    T: CoordFloat + FloatConst,
{
}

struct Path<T>
where
    T: CoordFloat,
{
    context_in: Option<CanvasRenderingContext2d>,
    context_stream: Option<Box<dyn PathStreamTrait<T>>>,
    point_radius: T,
    projection_stream_fn: Box<dyn Fn(&dyn PathTrait<T>) -> dyn Stream<T>>,
    projection_in: Option<Box<dyn Transform<T>>>,
}

fn projection_stream_noop() {}
impl<T> Default for Path<T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn default() -> Self {
        Self {
            context_in: None,
            context_stream: None,
            point_radius: T::from(4.5f64).unwrap(),
            projection_in: None,
            projection_stream_fn: Box::new(|&_| StreamIdentity {}),
        }
    }
}

impl<T> Path<T>
where
    T: CoordFloat + std::fmt::Display + AsPrimitive<T> + FloatConst,
{
    #[inline]
    fn gnerate(
        projection_in: Option<Box<dyn Transform<T>>>,
        context_in: Option<CanvasRenderingContext2d>,
    ) -> Path<T> {
        Path {
            projection_in,
            context_in,
            ..Default::default()
        }
        // self.projection(projection).context(context);
    }

    #[inline]
    fn path(&self) -> Box<dyn PathResult<T>> {
        Box::new(self.context_stream.result())
    }

    #[inline]
    fn area(&self, d: &impl DataObject<T>) {
        let pa = PathAreaStream::default();
        d.to_stream(&mut (self.projection_stream_fn)(pa));
    }

    // fn set_projection(&mut self, ps: Option<Box<dyn Transform<T>>>) {
    //     self.projection_in = ps;
    //     self.projection_stream_fn = None;
    // }

    fn projection(p_maybe: Option<ProjectionMutator<T>>) -> Path<T> {
        let projection_in: Option<ProjectionMutator<T>>;
        let projection_stream_fn: Option<Box<dyn Stream<T>>>;

        //  (projectionStream = _ == null ? (projection = null, identity) : (projection = _).stream, path)

        match p_maybe {
            None => {
                projection_in = None;
                projection_stream_fn = Some(Box::new(StreamIdentity {}));
            }
            Some(projection) => {
                projection_in = Some(projection);
                projection_stream_fn = projection.stream;
            }
        }

        return Path {
            projection_in,
            projection_stream_fn,
            ..Default::default()
        };
    }

    #[inline]
    fn get_context(&self) -> Option<Box<dyn Stream<T>>> {
        self.context_stream
    }

    fn context(self, c_in: Option<CanvasRenderingContext2d>) -> Self {
        match c_in {
            None => {
                self.context_in = None;
                self.context_stream = Some(Box::new(PathString::new()));
            }
            Some(c) => {
                self.context_in = c_in;
                self.context_stream = Some(Box::new(PathContext::new(c)));
            }
        }
        self
    }

    #[inline]
    fn set_point_radius(&mut self, pr: Box<dyn Stream<T>>) {
        self.point_radius = pr;
    }

    #[inline]
    fn get_point_radius(&self) -> T {
        self.point_radius
    }

    #[inline]
    fn point_radius(self, d: T) -> Self {
        (self.context_stream.point_radius(d), d);
        self
    }

    #[inline]
    fn generate_path(
        projection: Option<ProjectionMutator<T>>,
        context: Option<CanvasRenderingContext2d>,
    ) -> Path<T>
    where
        T: CoordFloat + std::fmt::Display + AsPrimitive<T>,
    {
        Path::projection(projection).context(context)
    }
}
