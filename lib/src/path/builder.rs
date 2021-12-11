use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use web_sys::CanvasRenderingContext2d;

use crate::clip::buffer::Buffer;
use crate::clip::post_clip_node::PostClipNode;
use crate::clip::Line;
use crate::clip::PointVisible;
use crate::projection::projection::Projection;
use crate::projection::resample::ResampleNode;
use crate::projection::stream_node::StreamNode;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;

use super::context::Context as PathContext;
use super::context_stream::ContextStream;
use super::path::Path;
use super::string::String as PathString;
use super::PointRadiusTrait;

/// Path builder.
#[derive(Debug)]
pub struct Builder<LINE, PR, PV, T>
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
    PV: PointVisible<T = T>,
    T: 'static + AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
{
    pr: T,
    context: Option<Rc<CanvasRenderingContext2d>>,
    context_stream: ContextStream<T>,
    projection: Option<Projection<ContextStream<T>, LINE, PR, PV, T>>,
}

impl<LINE, PR, PV, T> Builder<LINE, PR, PV, T>
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
    PV: PointVisible<T = T>,
    T: AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
{
    /// Constructor.
    pub fn new(context_stream: ContextStream<T>) -> Builder<LINE, PR, PV, T> {
        Self {
            context: None,
            context_stream,
            pr: T::from(4.5_f64).unwrap(),
            projection: None,
        }
    }
}

/// Context related methods.
impl<LINE, PR, PV, T> Builder<LINE, PR, PV, T>
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
    PV: PointVisible<T = T>,
    T: AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
{
    /// Returns the state within the builder.
    pub fn get_context(&self) {
        todo!("must implement");
    }

    /// Programe the builder with the context.
    pub fn context(self, context: CanvasRenderingContext2d) -> Builder<LINE, PR, PV, T> {
        let context = Rc::new(context);
        Builder {
            pr: self.pr,
            context: Some(context.clone()),
            context_stream: ContextStream::Context(PathContext::<T>::new(context)),
            projection: self.projection,
        }
    }

    /// Sets the radius of the displayed point, None implies no point to is drawn.
    pub fn point_radius(mut self, radius: T) -> Self {
        self.pr = radius;
        self.context_stream.point_radius(self.pr);
        self
    }

    /// Returns a Builder from default values.
    pub fn context_pathstring() -> Builder<LINE, PR, PV, T> {
        let context_stream: ContextStream<T> = ContextStream::S(PathString::default());

        Builder::new(context_stream)
    }
}

/// Projection related methods.
impl<LINE, PR, PV, T> Builder<LINE, PR, PV, T>
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
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// From the progammed state generate a new projection.
    #[inline]
    pub fn build(
        self,
        projection: Projection<ContextStream<T>, LINE, PR, PV, T>,
    ) -> Path<LINE, PR, PV, T>
    where
        PR: ProjectionRaw<T>,
    {
        Path::new(self.context_stream, projection)
    }
}
