// use crate::stream::GeoStream;
use delaunator::Point;

use crate::transform_stream::TransformStream;

struct StreamWrapperState
{
  stream: Box<dyn TransformStream>,
}

/// An object implementing a stream method
pub trait StreamWrapper
{
  /// Returns a projection stream for the specified output stream. Any input geometry is projected before being streamed to the output stream.
  /// A typical projection involves several geometry transformations: the input geometry is first converted to radians, rotated on three axes,
  /// clipped to the small circle or cut along the antimeridian, and lastly projected to the plane with adaptive resampling, scale and translation.
  ///
  /// @param stream An input stream
  fn stream(&mut self, stream: Box<dyn TransformStream>);
}

impl StreamWrapper for StreamWrapperState
{

  fn stream(&mut self, stream: Box<dyn TransformStream>) {
    self.stream = stream;
    // return self.stream;
  }
}
