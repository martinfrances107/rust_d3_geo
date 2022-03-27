mod feature_collection;
mod geometry;
mod geometry_collection;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;

// use crate::clip::Interpolator;
use std::fmt::Debug;
use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;
use geo::LineString;
use geo::Polygon;

/// State -- Unconnected.
///
/// A Stream Pipeline stages blank.
/// The state before connection.
#[derive(Clone, Default, Debug)]
pub struct Unconnected;
// impl ConnectionState for Unconnected {}

/// State -- Default Connected.
///
/// Common to many pipeline stages
/// Overriden when the state need to contain more variables.
/// see Resample and Clip.
#[derive(Clone, Debug)]
pub struct Connected<SINK> {
    pub sink: SINK,
}

// impl<SINK> ConnectionState for Connected<SINK> where SINK: Clone + Debug {}
impl<SINK> ConnectedState for Connected<SINK> {
    type Sink = SINK;
    #[inline]
    fn get_sink(&mut self) -> &mut Self::Sink {
        &mut self.sink
    }
}

/// Can make connections to a stream pipeline.
pub trait Connectable {
    /// Represents to final connected state.
    type Output;

    /// The type passed to the function connect().
    type SC;
    /// Connects to previous pipeline stage.
    fn connect(self, sink: Self::SC) -> Self::Output;
}

// Base for Unconnected or Connected State.
// pub trait ConnectionState: Debug + Clone {}

// Marker trait.
/// Things the implement stream need to assert that
/// Whatever specific state they are in,  it is to the exclusion
/// on the unconnected state.
pub trait ConnectedState {
    type Sink;
    fn get_sink(&mut self) -> &mut Self::Sink;
}

/// to_stream()
pub trait Streamable {
    /// f32 or f64.
    type T: CoordFloat;
    /// Injects the object to a stream.
    fn to_stream<EP, SINK>(&self, stream: &mut SINK)
    where
        SINK: Stream<EP = EP, T = Self::T>;
}

/// Stub is useful only the transform portion of a projection is needed.
/// TODO must add example to doc.
#[derive(Clone, Copy, Debug)]
pub struct StreamDrainStub<T> {
    phantom: PhantomData<T>,
}

impl<T> Stream for StreamDrainStub<T>
where
    T: CoordFloat,
{
    type T = T;
    type EP = Self;

    #[inline]
    fn get_endpoint(&mut self) -> &mut Self {
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
pub trait Stream
where
    <Self as Stream>::T: CoordFloat,
{
    /// f32 or f64.
    type T;
    /// The End point.
    type EP;

    /// Returns the end point of the stream.
    fn get_endpoint(&mut self) -> &mut Self::EP;

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

fn stream_line<EP, S, T>(ls: &LineString<T>, stream: &mut S, closed: usize)
where
    S: Stream<EP = EP, T = T>,
    T: CoordFloat,
{
    let n = ls.0.len() - closed;
    stream.line_start();
    for c in ls.0.iter().take(n) {
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
