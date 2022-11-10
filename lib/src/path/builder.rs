use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;

use geo::CoordFloat;
use num_traits::FloatConst;

use crate::path::context::Context;
use crate::path::Path;
use crate::projection::projector::Projector;
use crate::stream::Stream;
#[cfg(not(test))]
use web_sys::CanvasRenderingContext2d;

#[cfg(test)]
use crate::path_test_context::CanvasRenderingContext2d;

use super::context::Context as PathContext;
use super::string::String;
use super::PointRadiusTrait;

/// Path builder.
#[derive(Debug, Clone)]
pub struct Builder<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    p_pcnc: PhantomData<PCNC>,
    p_rc: PhantomData<RC>,
    pr: T,
    context_stream: CS,
    #[allow(clippy::type_complexity)]
    projection: Option<Projector<CLIPC, CLIPU, CS, PCNU, PR, RC, RU, T>>,
}

impl<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T>
    Builder<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat + FloatConst,
{
    /// Constructor.
    ///
    /// # Panics
    ///  Will never happen as 4.5 will always be converted into T.
    pub fn new(context_stream: CS) -> Self {
        Self {
            p_pcnc: PhantomData::<PCNC>,
            p_rc: PhantomData::<RC>,
            context_stream,
            pr: T::from(4.5_f64).unwrap(),
            projection: None,
        }
    }
}

/// Context related methods.
impl<CLIPC, CLIPU, PCNC, PCNU, PR, RC, RU, T>
    Builder<CLIPC, CLIPU, Context, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat + FloatConst,
{
    /// Programe the builder with the context.
    pub fn context(&mut self, context: CanvasRenderingContext2d) -> &mut Self {
        self.context_stream = PathContext::new(context);
        self
    }
}

/// Context related methods.
impl<CLIPC, CLIPU, PCNC, PCNU, PR, RC, RU, T>
    Builder<CLIPC, CLIPU, String<T>, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat + Display + FloatConst,
{
    /// Returns a Builder from default values.
    #[inline]
    #[must_use]
    pub fn context_pathstring() -> Self {
        Self::new(String::default())
    }
}

impl<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T> PointRadiusTrait
    for Builder<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
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
impl<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T>
    Builder<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    CS: Stream<EP = CS, T = T>,
    T: CoordFloat,
{
    #[inline]
    /// Returns a projectors based on the builder settings.
    pub fn build(
        self,
        projection: Projector<CLIPC, CLIPU, CS, PCNU, PR, RC, RU, T>,
    ) -> Path<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T> {
        Path::new(self.context_stream, projection)
    }
}
