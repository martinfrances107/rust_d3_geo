use std::cell::RefCell;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use web_sys::CanvasRenderingContext2d;

use crate::clip::PointVisible;
use crate::projection::projection::Projection;
use crate::projection::Raw as ProjectionRaw;

use super::context::Context as PathContext;
use super::context_stream::ContextStream;
use super::path::Path;
use super::string::String as PathString;
use super::PointRadiusTrait;

/// Path builder.
#[derive(Debug)]
pub struct Builder<PR, PV, T>
where
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
{
    pr: Option<T>,
    context: Option<Rc<CanvasRenderingContext2d>>,
    context_stream: Rc<RefCell<ContextStream<T>>>,
    projection: Option<Projection<ContextStream<T>, PR, PV, T>>,
}

impl<PR, PV, T> Builder<PR, PV, T>
where
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
{
    /// Constructor.
    pub fn new(context_stream: Rc<RefCell<ContextStream<T>>>) -> Builder<PR, PV, T> {
        Self {
            context: None,
            context_stream,
            pr: Some(T::from(4.5_f64).unwrap()),
            projection: None,
        }
    }
}

/// Context related methods.
impl<PR, PV, T> Builder<PR, PV, T>
where
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
{
    /// Returns the state within the builder.
    pub fn get_context(&self) {
        todo!("must implement");
    }

    /// Programe the builder with the context.
    pub fn context(self, context: CanvasRenderingContext2d) -> Builder<PR, PV, T> {
        let context = Rc::new(context);
        Builder {
            pr: self.pr,
            context: Some(context.clone()),
            context_stream: Rc::new(RefCell::new(ContextStream::C(PathContext::<T>::new(
                context,
            )))),
            projection: self.projection,
        }
    }

    /// Sets the radius of the displayed point, None implies no point to is drawn.
    pub fn point_radius(mut self, radius: Option<T>) -> Self {
        self.pr = radius;
        self.context_stream.borrow_mut().point_radius(self.pr);
        self
    }

    /// Returns a Builder from default values.
    pub fn context_pathstring() -> Builder<PR, PV, T> {
        let context_stream: Rc<RefCell<ContextStream<T>>> =
            Rc::new(RefCell::new(ContextStream::S(PathString::default())));

        Builder::new(context_stream)
    }
}

/// Projection related methods.
impl<PR, PV, T> Builder<PR, PV, T>
where
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// From the progammed state generate a new projection.
    #[inline]
    pub fn build(self, projection: Rc<Projection<ContextStream<T>, PR, PV, T>>) -> Path<PR, PV, T>
    where
        PR: ProjectionRaw<T>,
    {
        Path::new(self.context_stream, projection)
    }
}
