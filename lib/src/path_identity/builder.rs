use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::path::context::Context;
use crate::path_identity::Path;
use crate::projection::projector_identity::Projector;
use crate::stream::Stream;
#[cfg(not(test))]
use web_sys::CanvasRenderingContext2d;

#[cfg(test)]
use crate::path_test_context::CanvasRenderingContext2d;

use crate::path::context::Context as PathContext;
use crate::path::string::String;
use crate::path::PointRadiusTrait;

/// Path builder.
#[derive(Debug, Clone)]
pub struct Builder<CS, PCNC, PCNU, T>
where
    PCNU: Clone,
    PCNC: Clone,
    T: CoordFloat,
{
    p_pcnc: PhantomData<PCNC>,
    pr: T,
    context_stream: CS,
    projection: Option<Projector<CS, PCNC, PCNU, T>>,
}

impl<CS, PCNC, PCNU, T> Builder<CS, PCNC, PCNU, T>
where
    PCNU: Clone,
    PCNC: Clone,
    T: CoordFloat + FloatConst,
{
    /// Constructor.
    pub fn new(context_stream: CS) -> Self {
        Self {
            p_pcnc: PhantomData::<PCNC>,
            context_stream,
            pr: T::from(4.5_f64).unwrap(),
            projection: None,
        }
    }
}

/// Context related methods.
impl<PCNC, PCNU, T> Builder<Context, PCNC, PCNU, T>
where
    PCNC: Clone,
    PCNU: Clone,
    T: CoordFloat + FloatConst,
{
    /// Programe the builder with the context.
    pub fn context(self, context: CanvasRenderingContext2d) -> Self {
        Builder {
            p_pcnc: PhantomData::<PCNC>,
            pr: self.pr,
            context_stream: PathContext::new(context),
            projection: self.projection,
        }
    }
}

/// Context related methods.
impl<PCNC, PCNU, T> Builder<String<T>, PCNC, PCNU, T>
where
    PCNU: Clone,
    PCNC: Clone,
    T: CoordFloat + Display + FloatConst,
{
    /// Returns a Builder from default values.
    #[inline]
    pub fn context_pathstring() -> Self {
        Builder::new(String::default())
    }
}

impl<CS, PCNC, PCNU, T> PointRadiusTrait for Builder<CS, PCNC, PCNU, T>
where
    PCNC: Clone,
    PCNU: Clone,
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
impl<CS, PCNC, PCNU, T> Builder<CS, PCNC, PCNU, T>
where
    CS: Stream<EP = CS, T = T>,
    PCNC: Clone,
    PCNU: Clone,
    T: CoordFloat,
{
    /// From the progammed state generate a new projection.
    #[inline]
    pub fn build(self, projection: Projector<CS, PCNC, PCNU, T>) -> Path<CS, PCNC, PCNU, T> {
        Path::new(self.context_stream, projection)
    }
}
