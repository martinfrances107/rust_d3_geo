pub mod circle;
pub mod clip;

pub mod compare_intersections;
pub mod line_elem;
pub mod stream_node_clip_factory;

pub mod antimeridian;

pub mod buffer;

mod clean;
mod rejoin;

use std::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;

use crate::stream::Stream;
use crate::Debug;

/// Takes a line and cuts into visible segments. Return values used for polygon
/// clipPing: 0 - there were intersections or the line was empty; 1 - no
/// intersections 2 - there were intersections, and the first and last segments
/// should be rejoined.
#[derive(Debug, Clone, Copy)]
pub enum CleanEnum {
    Undefined,
    IntersectionsOrEmpty,
    NoIntersections,
    IntersectionsRejoin,
}

impl Default for CleanEnum {
    fn default() -> CleanEnum {
        CleanEnum::Undefined
    }
}

pub trait Clean {
    /// A clip trait.
    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    fn clean(&self) -> CleanEnum;
}

pub trait ClipTrait: PointVisible + Stream {}

pub trait PointVisible: Clone + Debug
where
    <Self as PointVisible>::T: CoordFloat,
{
    type T;
    fn point_visible(&self, p: &Coordinate<Self::T>, z: Option<u8>) -> bool;
}

pub type PostClipFn<DRAIN> = Rc<dyn Fn(Rc<RefCell<DRAIN>>) -> Rc<RefCell<DRAIN>>>;

pub type InterpolateFn<STREAM, T> =
    Rc<dyn Fn(Option<Coordinate<T>>, Option<Coordinate<T>>, T, Rc<RefCell<STREAM>>)>;

pub trait Line: Clean + Clone + Debug {}
pub trait LineFactory {}
