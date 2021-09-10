use std::cell::RefCell;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use web_sys::CanvasRenderingContext2d;

use crate::clip::Line;
use crate::clip::PointVisible;
use crate::projection::projection::Projection;
use crate::projection::Raw as ProjectionRaw;

use super::context::Context as PathContext;
use super::context_stream::ContextStream;
use super::path::Path;
use super::string::String as PathString;
use super::PointRadiusEnum;

/// Path builder.
#[derive(Debug)]
pub struct Builder<L, PR, PV, T>
where
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: CoordFloat + Display + FloatConst,
{
    point_radius: PointRadiusEnum<T>,
    context: Option<Rc<CanvasRenderingContext2d>>,
    context_stream: Rc<RefCell<ContextStream<T>>>,
    projection: Option<Projection<ContextStream<T>, L, PR, PV, T>>,
}

impl<L, PR, PV, T> Builder<L, PR, PV, T>
where
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: CoordFloat + Display + FloatConst,
{
    /// Constructor.
    pub fn new(context_stream: Rc<RefCell<ContextStream<T>>>) -> Builder<L, PR, PV, T> {
        Self {
            context: None,
            context_stream,
            point_radius: PointRadiusEnum::Val(T::from(4.5_f64).unwrap()),
            projection: None,
        }
    }
}

/// Context related methods.
impl<L, PR, PV, T> Builder<L, PR, PV, T>
where
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: CoordFloat + Display + FloatConst,
{
    /// Returns the state within the builder.
    pub fn get_context(&self) {
        todo!("must implement");
    }

    /// Programe the builder with the context.
    pub fn context(self, context: CanvasRenderingContext2d) -> Builder<L, PR, PV, T> {
        let context = Rc::new(context);
        Builder {
            point_radius: self.point_radius,
            context: Some(context.clone()),
            context_stream: Rc::new(RefCell::new(ContextStream::C(PathContext::<T>::new(
                context,
            )))),
            projection: self.projection,
        }
    }

    /// Returns a Builder from default values.
    pub fn context_pathstring() -> Builder<L, PR, PV, T> {
        let context_stream: Rc<RefCell<ContextStream<T>>> =
            Rc::new(RefCell::new(ContextStream::S(PathString::default())));

        Builder::new(context_stream)
    }
}

/// Projection related methods.
impl<L, PR, PV, T> Builder<L, PR, PV, T>
where
    // DRAIN: Stream<T = T> + Default,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// From the progammed state generate a new projection.
    #[inline]
    pub fn build(self, projection: Projection<ContextStream<T>, L, PR, PV, T>) -> Path<L, PR, PV, T>
    where
        PR: ProjectionRaw<T>,
    {
        Path::new(self.context_stream, projection)
    }
}
