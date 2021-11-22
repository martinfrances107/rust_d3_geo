/// Related the specifics of antimeridian clipping.
pub mod antimeridian;
/// Clipping algorithm need to stores points
/// until the end of the polygon is signalled.
/// and then clipping can calculate the a new clip polygon.
pub mod buffer;
/// Related the specifics of circle clipping.
pub mod circle;
/// Holds the clip struct.
pub mod clip;
/// Helper function.
pub mod compare_intersection;
mod intersection;
/// Wrapper for line primitives.
pub mod line;
/// The state of the line segments??
pub mod line_elem;
/// Rectangle helper function.
mod line_fn;
mod line_node;
/// A Stream pipeline stage.
pub mod post_clip;
/// A stream pipeline stage.
pub mod post_clip_node;
pub(crate) mod rectangle;
/// Clipping break line into segments which can lasted be reconnected together.
pub(crate) mod rejoin;
/// Factory takes in complex definition and output a stream pipeline node element.
pub mod stream_node_clip_factory;
/// Generate line stream node.
pub mod stream_node_line_factory;
/// Generate post clip stream node.
pub mod stream_node_post_clip_factory;

use std::fmt::Debug;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;

use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::circle::line::Line as LineCircle;
use crate::stream::Stream;

/// Internal clip state.
///
/// As the clip state machine enters ring_end() This state is used to direct
/// the clean up.
#[derive(Debug, Clone, Copy)]
pub(super) enum CleanState {
    /// There were not intersections or the line was empty.
    IntersectionsOrEmpty,
    /// There were no intersections and the first and last segments should be rejoined.
    NoIntersections,
    /// There were intersections, and the first and last segments should be rejoined.
    IntersectionsRejoin,
}

/// Clean
///
/// A clip trait.
/// Rejoin first and last segments if there were intersections and the first
/// and last points were visible.
pub(super) trait Clean {
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
    /// f64 or f32.
    type T;

    /// Is the point visible after clipping?
    fn point_visible(&self, p: &Coordinate<Self::T>) -> bool;
}

// /// A stage in the projector pipeline.
// pub(crate) type PostClipFn<DRAIN> = Rc<dyn Fn(Rc<RefCell<DRAIN>>) -> Rc<RefCell<DRAIN>>>;

/// Resample Stream Node - helper function.
pub(crate) type InterpolateFn<STREAM, T> =
    Rc<dyn Fn(Option<Coordinate<T>>, Option<Coordinate<T>>, T, &mut STREAM)>;

/// Part of the clipping definition.
trait Line: Clean + Clone + Debug {}

/// Line, part of the clipping function.
pub(crate) trait LineFactory {}
