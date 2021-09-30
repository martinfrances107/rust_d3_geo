/// Related the specifics of antimeridian clipping.
pub mod antimeridian;
/// Related the specifics of circle clipping.
pub mod circle;
/// Holds the clip struct.
pub mod clip;
/// Helper function.
pub mod compare_intersection;
/// The state of the line segments??
pub mod line_elem;
/// Factory takes in complex definition and output a stream pipeline node element.
pub mod stream_node_clip_factory;

/// Clipping algorithm need to stores points
/// until the end of the polygon is signalled
/// and then clipping can calculate the a new clip polygon.
pub mod buffer;

pub(crate) mod rectangle;

mod intersection;
/// Rectangle helper function.
mod line;
/// Clipping break line into segments which can lasted be reconnected together.
pub(crate) mod rejoin;

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
pub enum CleanState {
    /// Initial state.
    Undefined,
    /// There were not intersections or the line was empty.
    IntersectionsOrEmpty,
    /// There were no intersections and the first and last segments should be rejoined.
    NoIntersections,
    /// Intesections Rejoin.
    IntersectionsRejoin,
}

impl Default for CleanState {
    fn default() -> CleanState {
        CleanState::Undefined
    }
}

/// Clean
///
/// A clip trait.
/// Rejoin first and last segments if there were intersections and the first
/// and last points were visible.
pub trait Clean {
    /// Returns the clean state.
    fn clean(&self) -> CleanState;
}

/// ClipTrait
///
///  Related to a stream node pipeline stage.
pub trait ClipTrait: PointVisible + Stream {}

/// Clip Stream Node - helper function.
pub trait PointVisible: Clone + Debug
where
    <Self as PointVisible>::T: CoordFloat,
{
    /// f64 or f32
    type T;
    /// Is the point visible after clipping?
    fn point_visible(&self, p: &Coordinate<Self::T>, z: Option<u8>) -> bool;
}

// /// A stage in the projector pipeline.
// pub(crate) type PostClipFn<DRAIN> = Rc<dyn Fn(Rc<RefCell<DRAIN>>) -> Rc<RefCell<DRAIN>>>;

/// Resample Stream Node - helper function.
pub(crate) type InterpolateFn<STREAM, T> =
    Rc<dyn Fn(Option<Coordinate<T>>, Option<Coordinate<T>>, T, Rc<RefCell<STREAM>>)>;

/// Part of the clipping definition.
pub trait Line: Clean + Clone + Debug {}

/// Line, part of the clipping function.
pub(crate) trait LineFactory {}
