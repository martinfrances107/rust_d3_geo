use std::fmt::Display;
use std::ops::AddAssign;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::post_clip_node::PostClipNode;
use crate::clip::Line;
use crate::clip::PointVisible;
use crate::path::area::Area;
use crate::path::bounds::Bounds;
use crate::path::centroid::Centroid;
use crate::path::Result;
use crate::projection::projection::Projection;
use crate::projection::resample::ResampleNode;
use crate::projection::stream_node::StreamNode;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;
use crate::stream::Streamable;

use super::PointRadiusEnum;

/// Projection and context stream applied to a Streamable.
#[derive(Debug)]
pub struct Path<CS, LINE, PR, PV, T>
where
    CS: Stream<EP = CS, T = T> + Result + PartialEq,
    LINE: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    StreamNode<CS, LINE, ResampleNode<CS, PR, PostClipNode<CS, CS, T>, T>, T>:
        Stream<EP = CS, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    context_stream: CS,
    point_radius: PointRadiusEnum<T>,
    /// don't store projection stream.
    projection: Projection<CS, LINE, PR, PV, T>,
}

impl<CS, LINE, PR, PV, T> Path<CS, LINE, PR, PV, T>
where
    CS: Stream<EP = CS, T = T> + Result + PartialEq,
    LINE: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    StreamNode<CS, LINE, ResampleNode<CS, PR, PostClipNode<CS, CS, T>, T>, T>:
        Stream<EP = CS, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Constructor.
    pub fn new(context_stream: CS, projection: Projection<CS, LINE, PR, PV, T>) -> Self {
        Self {
            context_stream,
            point_radius: PointRadiusEnum::Val(T::from(4.5_f64).unwrap()),
            projection,
        }
    }

    /// Combines projection, context stream and object.
    pub fn object(&mut self, object: &impl Streamable<T = T>) -> <CS as Result>::Out {
        let mut stream_in = self.projection.stream(self.context_stream.clone());
        object.to_stream(&mut stream_in);
        stream_in.get_endpoint().result()
    }
}

impl<LINE, PR, PV, T> Path<Area<T>, LINE, PR, PV, T>
where
    LINE: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    StreamNode<Area<T>, LINE, ResampleNode<Area<T>, PR, PostClipNode<Area<T>, Area<T>, T>, T>, T>:
        Stream<EP = Area<T>, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Returns the area of the Path
    /// This operation consumes the  Path.
    pub fn area(mut self, object: &impl Streamable<T = T>) -> T
    where
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let stream_dst = Area::default();
        let mut stream_in = self.projection.stream(stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.sink.get_endpoint().result()
    }
}

impl<LINE, PR, PV, T> Path<Bounds<T>, LINE, PR, PV, T>
where
    LINE: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    StreamNode<
        Bounds<T>,
        LINE,
        ResampleNode<Bounds<T>, PR, PostClipNode<Bounds<T>, Bounds<T>, T>, T>,
        T,
    >: Stream<EP = Bounds<T>, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Returns the bounds of the object
    ///
    /// This operation consumes the  Path.
    pub fn bounds(mut self, object: &impl Streamable<T = T>) -> [Coordinate<T>; 2] {
        let stream_dst = Bounds::default();
        let mut stream_in = self.projection.stream(stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.get_endpoint().result()
    }
}

impl<LINE, PR, PV, T> Path<Centroid<T>, LINE, PR, PV, T>
where
    LINE: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    StreamNode<
        Centroid<T>,
        LINE,
        ResampleNode<Centroid<T>, PR, PostClipNode<Centroid<T>, Centroid<T>, T>, T>,
        T,
    >: Stream<EP = Centroid<T>, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Returns the centroid of the object.
    pub fn centroid(mut self, object: &impl Streamable<T = T>) -> Coordinate<T> {
        let stream_dst = Centroid::default();
        let mut stream_in = self.projection.stream(stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.get_endpoint().result()
    }
}

impl<CS, LINE, PR, PV, T> Path<CS, LINE, PR, PV, T>
where
    CS: Stream<EP = CS, T = T> + Result + PartialEq,
    LINE: Line,
    StreamNode<CS, LINE, ResampleNode<CS, PR, PostClipNode<CS, CS, T>, T>, T>:
        Stream<EP = CS, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Sets the context stream.
    pub fn context(mut self, context_stream: CS) -> Self {
        self.context_stream = context_stream;
        self
    }

    #[inline]
    fn point_radius(mut self, input: PointRadiusEnum<T>) -> Self {
        self.point_radius = match input {
            PointRadiusEnum::F(ref _input_fn) => input,
            PointRadiusEnum::Val(_input_value) => input,
        };
        self
    }
}
