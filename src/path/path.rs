use std::cell::RefCell;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::PointVisible;
use crate::data_object::DataObject;
use crate::path::area::Area;
use crate::path::bounds::Bounds;
use crate::path::centroid::Centroid;
use crate::path::context_stream::ContextStream;
use crate::path::Result;
use crate::projection::projection::Projection;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Streamable;

use super::PointRadiusEnum;
use super::ResultEnum;

/// Projection and context stream applied to a DataObject.
#[derive(Debug)]
pub struct Path<PR, PV, T>
where
    PR: ProjectionRaw<T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    PV: PointVisible<T = T>,
{
    context_stream: Rc<RefCell<ContextStream<T>>>,
    point_radius: PointRadiusEnum<T>,
    /// don't store projection stream.
    projection: Rc<Projection<ContextStream<T>, PR, PV, T>>,
}

impl<PR, PV, T> Path<PR, PV, T>
where
    PR: ProjectionRaw<T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    PV: PointVisible<T = T>,
{
    /// Constructor.
    pub fn new(
        context_stream: Rc<RefCell<ContextStream<T>>>,
        projection: Rc<Projection<ContextStream<T>, PR, PV, T>>,
    ) -> Self {
        Self {
            context_stream,
            point_radius: PointRadiusEnum::Val(T::from(4.5_f64).unwrap()),
            projection,
        }
    }

    /// Combines projection, context stream and object.
    pub fn object(&mut self, object: &DataObject<T>) -> Option<ResultEnum<T>> {
        let mut stream_in = self.projection.stream(self.context_stream.clone());
        object.to_stream(&mut stream_in);
        self.context_stream.borrow_mut().result()
    }

    /// Returns the area of the Path
    /// This operation consumes the  Path.
    pub fn area(self, object: &DataObject<T>) -> Option<ResultEnum<T>>
    where
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let stream_dst = Rc::new(RefCell::new(ContextStream::A(Area::default())));
        let mut stream_in = self.projection.stream(stream_dst.clone());
        object.to_stream(&mut stream_in);

        let x = stream_dst.borrow_mut().result();
        x
    }

    /// Returns the bounds of the object
    ///
    /// This operation consumes the  Path.
    pub fn bounds(self, object: &DataObject<T>) -> Option<ResultEnum<T>>
    where
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let stream_dst = Rc::new(RefCell::new(ContextStream::B(Bounds::default())));
        let mut stream_in = self.projection.stream(stream_dst.clone());
        object.to_stream(&mut stream_in);

        let b = stream_dst.borrow_mut().result();
        b
    }

    /// Returns the centroid of the object.
    pub fn centroid(self, object: &DataObject<T>) -> Option<ResultEnum<T>>
    where
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let stream_dst = Rc::new(RefCell::new(ContextStream::Centroid(Centroid::default())));
        let mut stream_in = self.projection.stream(stream_dst.clone());
        object.to_stream(&mut stream_in);

        let c = stream_dst.borrow_mut().result();
        c
    }

    /// Sets the context stream.
    pub fn context(mut self, context_stream: Rc<RefCell<ContextStream<T>>>) -> Self
    where
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        self.context_stream = context_stream;
        self
    }

    #[inline]
    fn point_radius(mut self, input: PointRadiusEnum<T>) -> Self {
        self.point_radius = match input {
            PointRadiusEnum::F(ref _input_fn) => input,
            PointRadiusEnum::Val(_input_value) => {
                // match &mut self.context_stream {
                // 	PathContextStream::S(s) => {
                // 		s.point_radius(Some(input_value));
                // 	}
                // 	PathContextStream::C(c) => {
                // 		c.point_radius(Some(input_value));
                // 	}
                // }
                // self.context_stream.point_radius(Some(input_value));
                input
            }
        };
        self
    }
}
