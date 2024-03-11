use core::fmt::Debug;
use core::fmt::Display;

use geo::CoordFloat;

use crate::path::endpoint::Endpoint;
use crate::path::Path;
use crate::projection::projector_albers_usa::multidrain::Multidrain;
use crate::projection::projector_albers_usa::multidrain::Unpopulated;
use crate::projection::Projector;

#[cfg(not(test))]
use web_sys::Path2d;

#[cfg(test)]
use crate::path_test_context::Path2d;

use super::endpoint::Endpoint as PathContext;
use super::string::String;
use super::PointRadiusTrait;

/// Path builder.
#[derive(Debug)]
pub struct Builder<CS, T>
where
    T: CoordFloat,
{
    pr: T,
    /// Either a [`PathString`](crate::path::string::String) or [Path2d].
    /// Rendering to a SVG Path element or a HTML Canvas element.
    pub context: CS,
}

impl<CS, T> Builder<CS, T>
where
    T: CoordFloat,
{
    /// Constructor.
    ///
    /// # Panics
    /// `unwrap()` is used here but a panic will never happen as 4.5 will always be converted into T.
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
    T: CoordFloat,
{
    /// Programe the path builder with the context.
    pub fn context(&mut self, context: Path2d) -> &mut Self {
        self.context = PathContext::new(context);
        self
    }
}

/// Context related methods.
impl<T> Builder<String<T>, T>
where
    T: CoordFloat + Display,
{
    /// Returns a Builder from default values.
    #[inline]
    #[must_use]
    pub fn pathstring() -> Self {
        Self::new(String::default())
    }
}

/// Context related methods.
impl<T> Builder<Multidrain<3, String<T>, Unpopulated>, T>
where
    T: CoordFloat + Display,
{
    /// Returns a Builder from default values.
    #[inline]
    #[must_use]
    pub fn albers_pathstring() -> Self {
        let md = Multidrain::new(String::<T>::default());
        Self::new(md)
    }
}

impl<CS, T> PointRadiusTrait for Builder<CS, T>
where
    CS: PointRadiusTrait<T = T>,
    T: CoordFloat,
{
    /// f64 or f32.
    type T = T;

    /// From the progammed state generate a new projection.
    #[inline]
    fn point_radius(&mut self, radius: T) {
        self.pr = radius;
        self.context.point_radius(self.pr);
    }
}

/// Projection related methods.
impl<CS, T> Builder<CS, T>
where
    T: CoordFloat,
{
    #[inline]
    /// Returns a projectors based on the builder settings.
    pub fn build<PROJECTOR, TRANSFORMER>(
        self,
        projection: PROJECTOR,
    ) -> Path<CS, PROJECTOR, T, TRANSFORMER>
    where
        PROJECTOR: Projector<EP = CS, Transformer = TRANSFORMER>,
    {
        Path::new(self.context, projection)
    }
}
