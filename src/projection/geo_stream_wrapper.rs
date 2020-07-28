use crate::stream::GeoStream;

/// An object implementing a stream method
pub trait GeoStreamWrapper<F> {
  /// Returns a projection stream for the specified output stream. Any input geometry is projected before being streamed to the output stream.
  /// A typical projection involves several geometry transformations: the input geometry is first converted to radians, rotated on three axes,
  /// clipped to the small circle or cut along the antimeridian, and lastly projected to the plane with adaptive resampling, scale and translation.
  ///
  /// @param stream An input stream
  fn stream(stream: dyn GeoStream<F>) -> dyn GeoStream<F>;

}