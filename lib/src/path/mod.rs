/// Stream node end point calculating area.
pub mod area;
/// Stream node end point calculating bounding boxes.
pub mod bounds;
/// Path builder
pub mod builder;
/// Path centroid;
pub mod centroid;
/// Path context.
pub mod context;
/// Output of a Path builer.
pub mod path;
/// Path String.
pub mod string;

use std::fmt;
use std::fmt::Debug;

use geo::CoordFloat;

/// Path Result.
pub trait Result {
    /// Output type for Result.
    type Out;

    /// Returns current the end points calculation.
    fn result(&mut self) -> Self::Out;
}

/// Point Radius Trait.
pub trait PointRadiusTrait {
    /// f64 or f32
    type T;
    // TODO must add getter here.
    // There are complication about the mix return type here.
    // Context or PathString
    // fn get_point_radius...
    fn point_radius(&mut self, val: Self::T);
}

/// Can be a scalar or a function that outputs a scalar.
pub enum PointRadiusEnum<T> {
    /// Holds a scalr value.
    Val(T),
    /// A function that output a scalar.
    F(Box<dyn Fn() -> T>),
}

#[cfg(not(tarpaulin_include))]
impl<T> Debug for PointRadiusEnum<T>
where
    T: CoordFloat,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointRadiusEnum").finish()
    }
}
