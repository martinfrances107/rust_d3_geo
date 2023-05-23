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
use geo::LineString;
use geo::Polygon;
use geo_types::Coord;

/// State -- Unconnected.
///
/// A 'blank' stage on the stream path.
///
/// The state before the path is constructed.
#[derive(Clone, Default, Debug)]
pub struct Unconnected;

/// State -- Default Connected.
///
/// Common to many path stages
/// Overriden when the state need to contain more variables.
/// see [Resample](crate::projection::resampler::resample::Resample) and [Clip](crate::clip::clipper::Clipper).
///
/// [Equirectangular](crate::projection::equirectangular::Equirectangular)
#[derive(Clone, Debug)]
pub struct Connected<SINK> {
    /// The next stage on the path,
    pub sink: SINK,
}

impl<SINK> ConnectedState for Connected<SINK> {
    type Sink = SINK;

    #[inline]
    fn sink(&mut self) -> &mut Self::Sink {
        &mut self.sink
    }
}

/// Make connections to a stream path.
pub trait Connectable {
    /// The next stage on the path.
    type Output<SC>;

    /// Connects to the previous path stage.
    fn connect<SC>(&self, sink: SC) -> Self::Output<SC>;
}

/// Things the implement stream need to assert that
/// Whatever specific state they are in, it is to the exclusion
/// on the unconnected state.
pub trait ConnectedState {
    /// The next path node.
    type Sink;

    /// Connects the next object on the path.
    fn sink(&mut self) -> &mut Self::Sink;
}

/// Objects that can be passing to a stream path.
pub trait Streamable {
    /// f32 or f64.
    type T: CoordFloat;

    /// Injects the object to a stream.
    fn to_stream<EP, SINK>(&self, stream: &mut SINK)
    where
        SINK: Stream<EP = EP, T = Self::T>;
}

/// Useful when the stream path is not used and only
/// the transform portion of a projection is needed.
///
/// ```
/// use geo_types::Coord;
/// use d3_geo_rs::Transform;
/// use d3_geo_rs::projection::builder::template::ResampleNoPCNC;
/// use d3_geo_rs::projection::stereographic::Stereographic;
/// use d3_geo_rs::projection::Build;
/// use d3_geo_rs::projection::RawBase as ProjectionRawBase;
/// use d3_geo_rs::stream::DrainStub;
///
/// // The Projector needs a mock endpoint here for the stream path.
/// let p = Stereographic::< f32>::builder::<DrainStub<f32>>().build();
///
/// let transformed_point = p.transform(&Coord{x: 0_f32, y:0_f32});
///
/// ```
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct DrainStub<T> {
    phantom: PhantomData<T>,
}

impl<T> Stream for DrainStub<T>
where
    T: CoordFloat,
{
    type EP = Self;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self {
        self
    }

    fn point(&mut self, _p: &Coord<Self::T>, _m: Option<u8>) {}
}
/// Stream Path API
///
/// Path stages can be connected to perform a sequence of
/// operations where the results can be stored in an endpoint.
pub trait Stream
where
    <Self as Stream>::T: CoordFloat,
{
    /// The End point.
    type EP;
    /// f32 or f64.
    type T;

    /// Returns the end point of the stream.
    fn endpoint(&mut self) -> &mut Self::EP;
    /// Declare the end of a line segment.
    fn line_end(&mut self) {}
    /// Declare the start of a line segment.
    fn line_start(&mut self) {}
    /// Declare a point.
    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>);
    /// Declare the end of a polygon.
    fn polygon_end(&mut self) {}
    /// Declare the start of a polygon.
    fn polygon_start(&mut self) {}
    /// Declare a sphere object.
    fn sphere(&mut self) {}
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
