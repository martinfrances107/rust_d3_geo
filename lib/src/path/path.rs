use std::fmt::Display;
use std::ops::AddAssign;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use super::PointRadiusEnum;
use super::ResultEnum;
use crate::clip::buffer::Buffer;
use crate::clip::post_clip_node::PostClipNode;
use crate::clip::Line;
use crate::clip::PointVisible;
use crate::path::area::Area;
use crate::path::bounds::Bounds;
use crate::path::centroid::Centroid;
use crate::path::context_stream::ContextStream;
use crate::path::Result;
use crate::projection::projection::Projection;
use crate::projection::resample::ResampleNode;
use crate::projection::stream_node::StreamNode;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;
use crate::stream::Streamable;

/// Projection and context stream applied to a Streamable.
#[derive(Debug)]
pub struct Path<LINE, PR, PV, T>
where
    LINE: Line,
    StreamNode<
        ContextStream<T>,
        LINE,
        ResampleNode<ContextStream<T>, PR, PostClipNode<ContextStream<T>, ContextStream<T>, T>, T>,
        T,
    >: Stream<EP = ContextStream<T>, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    PV: PointVisible<T = T>,
{
    context_stream: ContextStream<T>,
    point_radius: PointRadiusEnum<T>,
    /// don't store projection stream.
    projection: Projection<ContextStream<T>, LINE, PR, PV, T>,
}

impl<LINE, PR, PV, T> Path<LINE, PR, PV, T>
where
    LINE: Line,
    StreamNode<
        ContextStream<T>,
        LINE,
        ResampleNode<ContextStream<T>, PR, PostClipNode<ContextStream<T>, ContextStream<T>, T>, T>,
        T,
    >: Stream<EP = ContextStream<T>, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    PV: PointVisible<T = T>,
{
    /// Constructor.
    pub fn new(
        context_stream: ContextStream<T>,
        projection: Projection<ContextStream<T>, LINE, PR, PV, T>,
    ) -> Self {
        Self {
            context_stream,
            point_radius: PointRadiusEnum::Val(T::from(4.5_f64).unwrap()),
            projection,
        }
    }

    /// Combines projection, context stream and object.
    pub fn object(&mut self, object: &impl Streamable<T = T>) -> Option<ResultEnum<T>> {
        let mut stream_in = self.projection.stream(self.context_stream.clone());
        object.to_stream(&mut stream_in);
        stream_in.get_endpoint().result()
    }

    /// Returns the area of the Path
    /// This operation consumes the  Path.
    pub fn area(mut self, object: &impl Streamable<T = T>) -> Option<ResultEnum<T>>
    where
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let stream_dst = ContextStream::A(Area::default());
        let mut stream_in = self.projection.stream(stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.sink.get_endpoint().result()
    }

    /// Returns the bounds of the object
    ///
    /// This operation consumes the  Path.
    pub fn bounds(mut self, object: &impl Streamable<T = T>) -> Option<ResultEnum<T>>
    where
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let stream_dst = ContextStream::B(Bounds::default());
        let mut stream_in = self.projection.stream(stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.get_endpoint().result()
    }

    /// Returns the centroid of the object.
    pub fn centroid(mut self, object: &impl Streamable<T=T>) -> Option<ResultEnum<T>>
    where
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let stream_dst = ContextStream::Centroid(Centroid::default());
        let mut stream_in = self.projection.stream(stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.get_endpoint().result()
    }

    /// Sets the context stream.
    pub fn context(mut self, context_stream: ContextStream<T>) -> Self
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
