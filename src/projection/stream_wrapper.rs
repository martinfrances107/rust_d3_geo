use crate::transform_stream::TransformStream;

struct StreamWrapperState<T> {
    stream: Box<dyn TransformStream<T>>,
}

/// An object implementing a stream method
pub trait StreamWrapper<T> {
    /// Returns a projection stream for the specified output stream. Any input geometry is projected before being streamed to the output stream.
    /// A typical projection involves several geometry transformations: the input geometry is first converted to radians, rotated on three axes,
    /// clipped to the small circle or cut along the antimeridian, and lastly projected to the plane with adaptive resampling, scale and translation.
    ///
    /// @param stream An input stream
    fn stream(&mut self, stream: Box<dyn TransformStream<T>>);
}

impl<T> StreamWrapper<T> for StreamWrapperState<T> {
    fn stream(&mut self, stream: Box<dyn TransformStream<T>>) {
        self.stream = stream;
        // return self.stream;
    }
}
