// use crate::stream::GeoStream;
use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::transform_stream::TransformStream;

struct StreamWrapperState<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  stream: Box<dyn TransformStream<F>>,
}

/// An object implementing a stream method
pub trait StreamWrapper<F>
where
  F: Float + FloatConst + FromPrimitive,
{
  /// Returns a projection stream for the specified output stream. Any input geometry is projected before being streamed to the output stream.
  /// A typical projection involves several geometry transformations: the input geometry is first converted to radians, rotated on three axes,
  /// clipped to the small circle or cut along the antimeridian, and lastly projected to the plane with adaptive resampling, scale and translation.
  ///
  /// @param stream An input stream
  fn stream(&mut self, stream: Box<dyn TransformStream<F>>);
}

impl<F> StreamWrapper<F> for StreamWrapperState<F>
where
  F: Float + FloatConst + FromPrimitive,
{

  fn stream(&mut self, stream: Box<dyn TransformStream<F>>) {
    self.stream = stream;
    // return self.stream;
  }
}
