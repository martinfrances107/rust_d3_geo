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
/// A Stream Pipeline stages blank.
/// The state before connection.
#[derive(Clone, Default, Debug)]
pub struct Unconnected;

/// State -- Default Connected.
///
/// Common to many pipeline stages
/// Overriden when the state need to contain more variables.
/// see [Resample](crate::projection::resampler::resample::Resample) and [Clip](crate::clip::clip::Clip).
///
/// [Equirectangular](crate::projection::equirectangular::Equirectangular)
#[derive(Clone, Debug)]
pub struct Connected<SINK>
// where
//     SINK: Clone,
{
    /// The next stage in the pipeline,
    pub sink: SINK,
}

impl<SINK> ConnectedState for Connected<SINK>
where
    SINK: Clone,
{
    type Sink = SINK;

    #[inline]
    fn sink(&mut self) -> &mut Self::Sink {
        &mut self.sink
    }
}

/// Make connections to a stream pipeline.
pub trait Connectable {
    /// Represents to final connected state.
    type Output<SC: Clone>;

    /// Connects to previous pipeline stage.
    fn connect<SC: Clone>(&self, sink: SC) -> Self::Output<SC>;
}

/// Things the implement stream need to assert that
/// Whatever specific state they are in,  it is to the exclusion
/// on the unconnected state.
pub trait ConnectedState {
    /// The next pipeline stage type
    type Sink;

    /// Connects the next object in the pipeline.
    fn sink(&mut self) -> &mut Self::Sink;
}

/// Objects that can be passing to a stream pipeline.
pub trait Streamable {
    /// f32 or f64.
    type T: CoordFloat;

    /// Injects the object to a stream.
    fn to_stream<EP, SINK>(&self, stream: &mut SINK)
    where
        SINK: Stream<EP = EP, T = Self::T>;
}

/// Useful when the stream pipeline is not used and only
/// the transform portion of a projection is needed.
///
/// ```
/// use geo_types::Coord;
/// use rust_d3_geo::Transform;
/// use rust_d3_geo::projection::stereographic::Stereographic;
/// use rust_d3_geo::projection::Build;
/// use rust_d3_geo::projection::RawBase as ProjectionRawBase;
/// use rust_d3_geo::stream::DrainStub;
///
/// // The Projector needs a mock endpoint here for the stream pipeline.
/// let p = Stereographic::<DrainStub<f32>, f32>::builder().build();
///
/// let transformed_point = p.transform(&Coord{x: 0_f32, y:0_f32});
///
/// ```
#[derive(Clone, Copy, Debug)]
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

impl<T> Default for DrainStub<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData::default(),
        }
    }
}

/// Stream pipeline API
///
/// Pipeline states can be connected to perform a sequence of
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
