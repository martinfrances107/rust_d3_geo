use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use web_sys::CanvasRenderingContext2d;

use crate::path::context::Context;
use crate::projection::projector::Projector;
use crate::projection::ProjectionRawBase;

use super::context::Context as PathContext;
use super::string::String;
use super::Path;
use super::PointRadiusTrait;

/// Path builder.
#[derive(Debug, Clone)]
pub struct Builder<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    T: CoordFloat,
{
    pr: T,
    context_stream: CS,
    projection: Option<Projector<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>>,
}

impl<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Builder<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    T: AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
{
    /// Constructor.
    pub fn new(context_stream: CS) -> Self {
        Self {
            context_stream,
            pr: T::from(4.5_f64).unwrap(),
            projection: None,
        }
    }
}

/// Context related methods.
impl<'a, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Builder<Context<T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    T: AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
{
    /// Returns the state within the builder.
    // pub fn get_context(&self) {
    //     todo!("must implement");
    // }

    /// Programe the builder with the context.
    pub fn context(self, context: CanvasRenderingContext2d) -> Self {
        Builder {
            pr: self.pr,
            context_stream: PathContext::<T>::new(context),
            projection: self.projection,
        }
    }
}

/// Context related methods.
impl<'a, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Builder<String<T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    T: AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
{
    /// Returns a Builder from default values.
    pub fn context_pathstring() -> Self {
        let context_stream = String::default();

        Builder::new(context_stream)
    }
}

impl<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> PointRadiusTrait
    for Builder<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    CS: PointRadiusTrait<T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
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
impl<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Builder<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// From the progammed state generate a new projection.
    #[inline]
    pub fn build(
        self,
        projection: Projector<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>,
    ) -> Path<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    where
        PR: ProjectionRawBase<T>,
    {
        Path::new(self.context_stream, projection)
    }
}
