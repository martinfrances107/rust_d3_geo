/// Stream node end point calculating area.
pub mod area_stream;
/// Stream node end point calculating bounding boxes.
pub mod bounds_stream;
/// Path builder
pub mod builder;
/// Path context.
pub mod context;
/// A collection of Path endpoints.
pub mod context_stream;
/// Output of a Path builer
pub mod path;
/// Path String.
pub mod string;

use std::collections::VecDeque;
use std::fmt;
use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;

use crate::clip::line_elem::LineElem;

/// The result of the related path endpoint.
#[derive(Clone, Debug)]
pub enum ResultEnum<T>
where
    T: CoordFloat,
{
    /// The result of the Path endpoint.
    Path(Vec<Vec<Coordinate<T>>>),
    /// The buffered output of the path buffer endpoint.
    BufferOutput(VecDeque<Vec<LineElem<T>>>),
    /// The result of the String endpoint.
    String(String),
    /// The result of the Area endpoint.
    Area(T),
    /// The result of the Measure endpoint.
    Measure(T),
    /// The bounding box  of the Bounds endpoint.
    Bounds([Coordinate<T>; 2]),
    /// The centroid of the centroid endpoint.
    Centroid(T),
}

/// Path Result.
pub trait Result {
    /// The output type.
    type Out;
    /// Returns current the end points calculation.
    fn result(&mut self) -> Self::Out;
}

trait PointRadiusTrait {
    type PrtT;
    // TODO must add getter here.
    // There are complication about the mix return type here.
    // Context or PathString .. wrapped in a ContextStream!
    // fn get_point_radius...
    fn point_radius(&mut self, val: Self::PrtT);
}

/// Can be a scalar or a function that outputs a scalar.
pub enum PointRadiusEnum<T> {
    /// Holds a scalr value.
    Val(T),
    /// A function that output a scalar.
    F(Box<dyn Fn() -> T>),
}

impl<T> Debug for PointRadiusEnum<T>
where
    T: CoordFloat,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointRadiusEnum").finish()
    }
}
