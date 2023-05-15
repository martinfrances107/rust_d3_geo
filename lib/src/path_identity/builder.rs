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
use web_sys::Path2d;

#[cfg(test)]
use crate::path_test_context::Path2d;

use crate::path::context::Context as PathContext;
use crate::path::string::String;
use crate::path::PointRadiusTrait;

/// Path builder.
#[derive(Debug)]
pub struct Builder<CS, PCNC, PCNU, T>
where
    PCNC: Clone,
    PCNU: Clone,
    T: CoordFloat,
{
    p_pcnc: PhantomData<PCNC>,
    p_pcnu: PhantomData<PCNU>,
    pr: T,
    context_stream: CS,
}

impl<CS, PCNC, PCNU, T> Builder<CS, PCNC, PCNU, T>
where
    PCNC: Clone,
    PCNU: Clone,
    T: CoordFloat + FloatConst,
{
    /// Constructor.
    ///
    /// # Panics
    /// unwrap() is used here but a panic will never happen as 4.5 will always be converted into T.
    pub fn new(context_stream: CS) -> Self {
        Self {
            p_pcnc: PhantomData::<PCNC>,
            p_pcnu: PhantomData::<PCNU>,
            context_stream,
            pr: T::from(4.5_f64).unwrap(),
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
    pub fn context(&mut self, context: Path2d) -> &mut Self {
        self.context_stream = PathContext::new(context);
        self
    }
}

/// Context related methods.
impl<PCNC, PCNU, T> Builder<String<T>, PCNC, PCNU, T>
where
    PCNC: Clone,
    PCNU: Clone,
    T: CoordFloat + Display + FloatConst,
{
    /// Returns a Builder from default values.
    #[inline]
    #[must_use]
    pub fn context_pathstring() -> Self {
        Self::new(String::default())
    }
}

impl<CS, PCNC, PCNU, T> PointRadiusTrait for Builder<CS, PCNC, PCNU, T>
where
    PCNC: Clone,
    PCNU: Clone,
    CS: PointRadiusTrait<T = T>,
    T: CoordFloat,
{
    type T = T;

    /// Radius of the rendered point.
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
    #[inline]
    /// Returns a projectors based on the builder settings.
    pub fn build(self, projection: Projector<CS, PCNC, PCNU, T>) -> Path<CS, PCNC, PCNU, T> {
        Path::new(self.context_stream, projection)
    }
}
