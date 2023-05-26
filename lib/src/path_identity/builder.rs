use std::fmt::Debug;
use std::fmt::Display;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::path::endpoint::Endpoint;
use crate::path_identity::Path;
use crate::projection::projector_identity::Projector;
use crate::stream::Stream;
#[cfg(not(test))]
use web_sys::Path2d;

#[cfg(test)]
use crate::path_test_context::Path2d;

use crate::path::endpoint::Endpoint as PathContext;
use crate::path::string::String;
use crate::path::PointRadiusTrait;

/// Path builder.
#[derive(Debug)]
pub struct Builder<CS, T>
where
    T: CoordFloat,
{
    pr: T,
    context: CS,
}

impl<CS, T> Builder<CS, T>
where
    T: CoordFloat + FloatConst,
{
    /// Constructor.
    ///
    /// # Panics
    /// unwrap() is used here but a panic will never happen as 4.5 will always be converted into T.
    pub fn new(context: CS) -> Self {
        Self {
            context,
            pr: T::from(4.5_f64).unwrap(),
        }
    }
}

/// Context related methods.
impl<T> Builder<Endpoint, T>
where
    T: CoordFloat + FloatConst,
{
    /// Programe the builder with the context.
    pub fn context(&mut self, context: Path2d) -> &mut Self {
        self.context = PathContext::new(context);
        self
    }
}

/// Context related methods.
impl<T> Builder<String<T>, T>
where
    T: CoordFloat + Display + FloatConst,
{
    /// Returns a Builder from default values.
    #[inline]
    #[must_use]
    pub fn pathstring() -> Self {
        Self::new(String::default())
    }
}

impl<CS, T> PointRadiusTrait for Builder<CS, T>
where
    CS: PointRadiusTrait<T = T>,
    T: CoordFloat,
{
    type T = T;

    /// Radius of the rendered point.
    #[inline]
    fn point_radius(&mut self, radius: T) {
        self.pr = radius;
        self.context.point_radius(self.pr);
    }
}

/// Projection related methods.
impl<CS, T> Builder<CS, T>
where
    CS: Stream<EP = CS, T = T>,
    T: CoordFloat,
{
    #[inline]
    /// Returns a projectors based on the builder settings.
    pub fn build<PCNC, PCNU: Clone>(
        self,
        projection: Projector<CS, PCNC, PCNU, T>,
    ) -> Path<CS, PCNC, PCNU, T> {
        Path::new(self.context, projection)
    }
}
