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
pub(crate) mod rejoin;

use geo::CoordFloat;
use geo::Coordinate;

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
    type Output;
    /// f64 or f32
    type T;
    /// conected buffer as the next pipeline stage.
    fn buffer(&mut self, buffer: Buffer<Self::T>) -> Self::Output
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
    fn point_visible(&self, p: &Coordinate<Self::T>) -> bool;
}

/// Antimeridian or Circle interpolator.
pub trait Interpolator {
    /// f64 or f32.
    type T;
    /// Stream modifier.
    fn interpolate<EP, STREAM>(
        &self,
        to: Option<Coordinate<Self::T>>,
        from: Option<Coordinate<Self::T>>,
        direction: Self::T,
        stream: &mut STREAM,
    ) where
        STREAM: Stream<EP = EP, T = Self::T>,
        Self::T: CoordFloat;
}

/// When connected a line can return a mutable sink.
pub trait LineConnected: Clean {
    /// Sink -- When Connected.
    type SC;
    /// Connects the next object in the pipeline.
    fn sink(&mut self) -> &mut Self::SC;
}
