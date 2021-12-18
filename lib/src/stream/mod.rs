mod feature_collection;
mod geometry;
mod geometry_collection;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;

use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use geo::LineString;
use geo::Polygon;

/// to_stream()
pub trait Streamable {
    /// f32 or f64.
    type T: CoordFloat;
    /// Injects the object to a stream.
    fn to_stream<
        EP: Clone + Debug + Stream<EP = EP, T = Self::T>,
        SD: Stream<EP = EP, T = Self::T>,
    >(
        &self,
        stream: &mut SD,
    );
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
    type EP = Self;

    #[inline]
    fn get_endpoint(self) -> Self {
        self
    }
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

/// Stream pipeline API
/// Default implmentation is a no-op.
pub trait Stream: Clone + Debug
where
    <Self as Stream>::T: CoordFloat,
{
    /// f32 or f64.
    type T;
    /// The End point.
    type EP;

    /// Returns the end point of the stream.
    fn get_endpoint(self) -> Self::EP;

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
fn stream_line<EP, S, T>(ls: &LineString<T>, stream: &mut S, closed: usize)
where
    S: Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    let n = ls.0.len() - closed;
    stream.line_start();
    for c in &ls.0[0..n] {
        stream.point(c, None);
    }
    stream.line_end();
}

fn stream_polygon<EP, S, T>(polygon: &Polygon<T>, stream: &mut S)
where
    S: Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    stream.polygon_start();
    stream_line(polygon.exterior(), stream, 1);
    for p in polygon.interiors() {
        stream_line(p, stream, 1);
    }
    stream.polygon_end();
}
