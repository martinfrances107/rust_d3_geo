pub mod bounds_stream;
pub mod path;
pub mod path_area_stream;
pub mod path_context;
pub mod path_context_stream;
pub mod path_string;

use std::collections::VecDeque;
use std::default::Default;
use std::fmt;
use std::fmt::Display;
use std::ops::AddAssign;

// use derivative::Derivative;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use web_sys::CanvasRenderingContext2d;

use crate::clip::line_elem::LineElem;
use crate::stream::Stream;

#[derive(Clone, Debug)]
pub enum PathResultEnum<T>
where
    T: CoordFloat,
{
    Path(Vec<Vec<Coordinate<T>>>),
    ClipBufferOutput(VecDeque<Vec<LineElem<T>>>),
    String(String),
    Area(T),
    Measure(T),
    Bounds([Coordinate<T>; 2]),
    Centroid(T),
}

pub trait PathResult // where
{
    type Out;
    fn result(&mut self) -> Self::Out;
}

trait PointRadiusTrait {
    type PrtT;
    // TODO must add getter here.
    // There are complication about the mix return type here.
    // PathContext or PathString .. wrapped in a PathContextStream!
    // fn get_point_radius...
    fn point_radius(&mut self, val: Self::PrtT);
}

enum PointRadiusEnum<T> {
    Val(T),
    F(Box<dyn Fn() -> T>),
}

impl<T> fmt::Debug for PointRadiusEnum<T>
where
    T: CoordFloat + FloatConst,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointRadiusEnum").finish()
    }
}

trait PathTrait: PointRadiusTrait // where
//     T: CoordFloat + FloatConst,
{
    type PtDo;
    type PtPRE;
    fn area(&self, _d: Self::PtDo) -> Option<String> {
        // Stream(d, self.projection_stream);
        // PathArea::result();
        None
    }
    fn measure(&self, d: Self::PtDo) -> Self::PtPRE;

    fn bounds(&self, d: Self::PtDo) -> Self::PtPRE;

    fn centroid(&self, d: Self::PtDo) -> Self::PtPRE;

    fn projection(&self, d: Self::PtDo) -> Self::PtPRE;

    fn context_get(&self) -> CanvasRenderingContext2d;
    fn context(&self);
    // fn point_radius_get(&self);
    // fn point_radius_set(&self);
    // fn point_radius(&self);
    // fn result(&self);
}

trait PathStreamTrait<T>: Stream + PathTrait + PathResult
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
}
