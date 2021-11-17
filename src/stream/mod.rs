mod feature_collection;
mod geometry;
mod geometry_collection;
mod line;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;
mod rect;
mod triangle;

use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use geo::LineString;
use geo::Polygon;

/// Applies to DataObjects
pub trait Streamable {
    /// f32 or f64.
    type T: CoordFloat;
    /// Injects the object to a stream.
    fn to_stream<SD: Stream<T = Self::T>>(&self, stream: &mut SD);
}

/// Stub is useful only the transform portion of a projection is needed.
/// TODO must add example to doc.
#[derive(Clone, Copy, Debug)]
pub struct StreamDrainStub<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

impl<T> Stream for StreamDrainStub<T>
where
    T: CoordFloat,
{
    type T = T;
    fn point(&mut self, _p: &Coordinate<T>, _m: Option<u8>) {}
    fn sphere(&mut self) {}
    fn line_start(&mut self) {}
    fn line_end(&mut self) {}
    fn polygon_start(&mut self) {}
    fn polygon_end(&mut self) {}
}

impl<T> Default for StreamDrainStub<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            phantom: PhantomData::default(),
        }
    }
}

/// Is a node in the stream pipeline.
/// Default implmentation is a no-op.
pub trait Stream: Clone + Debug
where
    <Self as Stream>::T: CoordFloat,
{
    /// f32 or f64.
    type T;

    /// Declare a point.
    fn point(&mut self, _p: &Coordinate<Self::T>, _m: Option<u8>) {}
    /// Declare a sphere object.
    fn sphere(&mut self) {}
    /// Declare the start of a line segments.
    fn line_start(&mut self) {}
    /// Declare the end of a line segments.
    fn line_end(&mut self) {}
    /// Declare the start of a polygon.
    fn polygon_start(&mut self) {}
    /// Declare the end of a polygon.
    fn polygon_end(&mut self) {}
}

/// TODO Generics - Need to come back and refactor to take LineElem<T>
/// or Coordinates. As the JS allow for.
fn stream_line<T, S>(ls: &LineString<T>, stream: &mut S, closed: usize)
where
    S: Stream<T = T>,
    T: CoordFloat,
{
    let n = ls.0.len() - closed;
    stream.line_start();
    for c in &ls.0[0..n] {
        stream.point(c, None);
    }
    stream.line_end();
}

fn stream_polygon<S, T>(polygon: &Polygon<T>, stream: &mut S)
where
    S: Stream<T = T>,
    T: CoordFloat,
{
    stream.polygon_start();
    stream_line(polygon.exterior(), stream, 1);
    for p in polygon.interiors() {
        stream_line(p, stream, 1);
    }
    stream.polygon_end();
}
