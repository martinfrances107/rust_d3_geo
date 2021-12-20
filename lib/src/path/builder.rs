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
use crate::path::context::Context;
use crate::path::Result;
use crate::projection::projection::Projection;
use crate::projection::resample::ResampleNode;
use crate::projection::stream_node::StreamNode;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Stream;

use super::context::Context as PathContext;
use super::path::Path;
use super::string::String;
use super::PointRadiusTrait;

/// Path builder.
#[derive(Debug)]
pub struct Builder<CS, LINE, PR, PV, T>
where
    CS: Stream<EP = CS, T = T>,
    LINE: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    StreamNode<CS, LINE, CS, T>: Stream<EP = CS, T = T>,
    StreamNode<CS, LINE, ResampleNode<CS, PR, PostClipNode<CS, CS, T>, T>, T>:
        Stream<EP = CS, T = T>,
{
    pr: T,
    context: Option<Rc<CanvasRenderingContext2d>>,
    context_stream: CS,
    projection: Option<Projection<CS, LINE, PR, PV, T>>,
}

impl<CS, LINE, PR, PV, T> Builder<CS, LINE, PR, PV, T>
where
    CS: Stream<EP = CS, T = T>,
    LINE: Line,
    StreamNode<CS, LINE, ResampleNode<CS, PR, PostClipNode<CS, CS, T>, T>, T>:
        Stream<EP = CS, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
    StreamNode<CS, LINE, CS, T>: Stream<EP = CS, T = T>,
{
    /// Constructor.
    pub fn new(context_stream: CS) -> Builder<CS, LINE, PR, PV, T> {
        Self {
            context: None,
            context_stream,
            pr: T::from(4.5_f64).unwrap(),
            projection: None,
        }
    }
}

/// Context related methods.
impl<LINE, PR, PV, T> Builder<Context<T>, LINE, PR, PV, T>
where
    LINE: Line,

    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
    StreamNode<String<T>, LINE, String<T>, T>: Stream<EP = String<T>, T = T>,

    StreamNode<String<T>, LINE, String<T>, T>: Stream<T = T>,
    StreamNode<
        String<T>,
        LINE,
        ResampleNode<String<T>, PR, PostClipNode<String<T>, String<T>, T>, T>,
        T,
    >: Stream<EP = String<T>, T = T>,
    StreamNode<Context<T>, LINE, Context<T>, T>: Stream<EP = Context<T>, T = T>,
    StreamNode<
        Context<T>,
        LINE,
        ResampleNode<Context<T>, PR, PostClipNode<Context<T>, Context<T>, T>, T>,
        T,
    >: Stream<EP = Context<T>, T = T>,
{
    /// Returns the state within the builder.
    // pub fn get_context(&self) {
    //     todo!("must implement");
    // }

    /// Programe the builder with the context.
    pub fn context(
        self,
        context: CanvasRenderingContext2d,
    ) -> Builder<Context<T>, LINE, PR, PV, T> {
        let context = Rc::new(context);
        Builder {
            pr: self.pr,
            context: Some(context.clone()),
            context_stream: PathContext::<T>::new(context),
            projection: self.projection,
        }
    }
}

/// Context related methods.
impl<CS, LINE, PR, PV, T> Builder<CS, LINE, PR, PV, T>
where
    CS: Stream<EP = CS, T = T> + PointRadiusTrait<T = T>,
    LINE: Line,
    StreamNode<
        PathContext<T>,
        LINE,
        ResampleNode<PathContext<T>, PR, PostClipNode<PathContext<T>, PathContext<T>, T>, T>,
        T,
    >: Stream<EP = CS, T = T>,
    StreamNode<CS, LINE, ResampleNode<CS, PR, PostClipNode<CS, CS, T>, T>, T>:
        Stream<EP = CS, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
    StreamNode<String<T>, LINE, String<T>, T>: Stream<EP = String<T>, T = T>,
    StreamNode<CS, LINE, CS, T>: Stream<EP = CS, T = T>,

    StreamNode<String<T>, LINE, String<T>, T>: Stream<T = T>,
    StreamNode<
        String<T>,
        LINE,
        ResampleNode<String<T>, PR, PostClipNode<String<T>, String<T>, T>, T>,
        T,
    >: Stream<EP = String<T>, T = T>,
    StreamNode<Context<T>, LINE, Context<T>, T>: Stream<EP = Context<T>, T = T>,
{
    /// Returns a Builder from default values.
    pub fn context_pathstring() -> Builder<String<T>, LINE, PR, PV, T> {
        let context_stream = String::default();

        Builder::new(context_stream)
    }
}

impl<CS, LINE, PR, PV, T> PointRadiusTrait for Builder<CS, LINE, PR, PV, T>
where
    CS: Stream<EP = CS, T = T> + PointRadiusTrait<T = T> + Result<T = T> + PartialEq,
    LINE: Line,
    StreamNode<CS, LINE, ResampleNode<CS, PR, PostClipNode<CS, CS, T>, T>, T>:
        Stream<EP = CS, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    StreamNode<CS, LINE, CS, T>: Stream<EP = CS, T = T>,
{
    type T = T;
    /// From the progammed state generate a new projection.
    #[inline]
    fn point_radius(&mut self, radius: T) {
        self.pr = radius;
        self.context_stream.point_radius(self.pr);
    }
}

/// Projection related methods.
impl<CS, LINE, PR, PV, T> Builder<CS, LINE, PR, PV, T>
where
    CS: Stream<EP = CS, T = T> + Result<T = T> + PartialEq,
    LINE: Line,
    StreamNode<CS, LINE, ResampleNode<CS, PR, PostClipNode<CS, CS, T>, T>, T>:
        Stream<EP = CS, T = T>,
    StreamNode<Buffer<T>, LINE, Buffer<T>, T>: Stream<EP = Buffer<T>, T = T>,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    StreamNode<CS, LINE, CS, T>: Stream<EP = CS, T = T>,
{
    /// From the progammed state generate a new projection.
    #[inline]
    pub fn build(self, projection: Projection<CS, LINE, PR, PV, T>) -> Path<CS, LINE, PR, PV, T>
    where
        PR: ProjectionRaw<T>,
    {
        Path::new(self.context_stream, projection)
    }
}
