use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::path::context::Context;
use crate::path::Path;
use crate::projection::Projector;
use crate::stream::Stream;

#[cfg(not(test))]
use web_sys::CanvasRenderingContext2d;

#[cfg(test)]
use crate::path_test_context::CanvasRenderingContext2d;

use super::context::Context as PathContext;
use super::string::String;
use super::PointRadiusTrait;

/// Path builder.
#[derive(Debug)]
pub struct Builder<CS, PROJECTOR, T>
where
    PROJECTOR: Projector<Drain = CS>,
    T: CoordFloat,
{
    p_projector: PhantomData<PROJECTOR>,
    pr: T,
    context_stream: CS,
}

impl<CS, PROJECTOR, T> Builder<CS, PROJECTOR, T>
where
    PROJECTOR: Projector<Drain = CS>,
    T: CoordFloat + FloatConst,
{
    /// Constructor.
    ///
    /// # Panics
    /// unwrap() is used here but a panic will never happen as 4.5 will always be converted into T.
    pub fn new(context_stream: CS) -> Self {
        Self {
            p_projector: PhantomData::<PROJECTOR>,

            context_stream,
            pr: T::from(4.5_f64).unwrap(),
        }
    }
}

/// Context related methods.
impl<PROJECTOR, T> Builder<Context, PROJECTOR, T>
where
    PROJECTOR: Projector<Drain = Context>,
    T: CoordFloat + FloatConst,
{
    /// Programe the builder with the context.
    pub fn context(&mut self, context: CanvasRenderingContext2d) -> &mut Self {
        self.context_stream = PathContext::new(context);
        self
    }
}

/// Context related methods.
impl<PROJECTOR, T> Builder<String<T>, PROJECTOR, T>
where
    PROJECTOR: Projector<Drain = String<T>>,
    T: CoordFloat + Display + FloatConst,
{
    /// Returns a Builder from default values.
    #[inline]
    #[must_use]
    pub fn context_pathstring() -> Self {
        Self::new(String::default())
    }
}

impl<CS, PROJECTOR, T> PointRadiusTrait for Builder<CS, PROJECTOR, T>
where
    PROJECTOR: Projector<Drain = CS>,
    CS: PointRadiusTrait<T = T>,
    T: CoordFloat,
{
    /// f64 or f32.
    type T = T;

    /// From the progammed state generate a new projection.
    #[inline]
    fn point_radius(&mut self, radius: T) {
        self.pr = radius;
        self.context_stream.point_radius(self.pr);
    }
}

/// Projection related methods.
impl<CS, PROJECTOR, T> Builder<CS, PROJECTOR, T>
where
    PROJECTOR: Projector<Drain = CS>,
    CS: Stream<EP = CS, T = T>,
    T: CoordFloat,
{
    #[inline]
    /// Returns a projectors based on the builder settings.
    pub fn build(self, projection: PROJECTOR) -> Path<CS, PROJECTOR, T> {
        Path::new(self.context_stream, projection)
    }
}
