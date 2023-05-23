/// Related the specifics of antimeridian clipping.
pub mod antimeridian;
/// Clipping algorithm need to stores points
/// until the end of the polygon is signalled.
/// and then clipping can calculate the a new clip polygon.
pub mod buffer;
/// Related the specifics of circle clipping.
pub mod circle;
/// Holds the clip struct.
pub mod clipper;
/// Helper function.
pub mod compare_intersection;
/// Interpolator used by Rectangle.
pub mod interpolator;

mod intersection;
/// The state of the line segments??
pub mod line_elem;
/// Rectangle helper function.
mod line_fn;

pub(crate) mod rectangle;

/// Clipping break line into segments which can lasted be reconnected together.
pub mod rejoin;

use geo::CoordFloat;
use geo_types::Coord;

use crate::stream::Stream;

use buffer::Buffer;

/// Clean
///
/// A clip trait.
/// Rejoin first and last segments if there were intersections and the first
/// and last points were visible.
pub trait Clean {
    /// Returns the clean state.
    fn clean(&self) -> u8;
}

/// Can make stream connections to a specfic EP.
/// A buffer.
pub trait Bufferable {
    /// Resultant Line type: Antimeridian or Clip.
    type LINE;
    /// f64 or f32
    type T;
    /// conected buffer as the next path stage.
    fn buffer(&mut self, buffer: Buffer<Self::T>) -> Self::LINE
    where
        Self::T: CoordFloat;
}

/// Clip Stream Node - helper function.
pub trait PointVisible
where
    <Self as PointVisible>::T: CoordFloat,
{
    /// f64 or f32.
    type T;

    /// Is the point visible after clipping?
    fn point_visible(&self, p: &Coord<Self::T>) -> bool;
}

/// Antimeridian or Circle interpolator.
pub trait Interpolator {
    /// f64 or f32.
    type T;
    /// Stream modifier.
    fn interpolate<EP, STREAM>(
        &self,
        to: Option<Coord<Self::T>>,
        from: Option<Coord<Self::T>>,
        direction: Self::T,
        stream: &mut STREAM,
    ) where
        STREAM: Stream<EP = EP, T = Self::T>,
        Self::T: CoordFloat;
}

/// When connected a line can return a mutable sink.
pub trait LineConnected: Clean {
    /// Sink -- When Connected.
    type SINK;
    /// Connects the next node on the path.
    fn sink(&mut self) -> &mut Self::SINK;
}
