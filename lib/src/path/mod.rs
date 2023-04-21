/// Stream node end point calculating area.
pub mod area;
/// Stream node end point calculating bounding boxes.
pub mod bounds;
/// Path builder
pub mod builder;
/// Path centroid.
pub mod centroid;
/// Path context.
pub mod context;
/// Measure the perimeters of polyogns, lengths of lines.
pub mod measure;
/// Path String.
pub mod string;

mod tests;

use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::path::area::Area;
use crate::path::bounds::Bounds;
use crate::path::centroid::Centroid;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Projector;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Streamable;

use self::measure::Measure;

/// Path Result.
pub trait Result {
    /// Output type for Result.
    type Out;

    /// Returns current the end points calculation.
    fn result(&mut self) -> Self::Out;
}

/// Point Radius Trait.
pub trait PointRadiusTrait {
    /// f64 or f32.
    type T;

    /// Sets the radius of a rendered point.
    fn point_radius(&mut self, val: Self::T);
}

/// Can be a scalar or a function that outputs a scalar.
pub enum PointRadiusEnum<T> {
    /// Holds a scalr value.
    Val(T),
    /// A function that output a scalar.
    F(Box<dyn Fn() -> T>),
}

#[cfg(not(tarpaulin_include))]
impl<T> Debug for PointRadiusEnum<T>
where
    T: CoordFloat,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointRadiusEnum").finish()
    }
}

/// Projection and context stream applied to a Streamable.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Path<CS, PROJECTOR, T, TRANSFORMER>
where
    PROJECTOR: Projector<EP = CS, Transformer = TRANSFORMER>,
    T: CoordFloat,
{
    context_stream: CS,
    point_radius: PointRadiusEnum<T>,
    projection: PROJECTOR,
}

// impl<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T> Path<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T>
impl<CS, PROJECTOR, T, TRANSFORMER> Path<CS, PROJECTOR, T, TRANSFORMER>
where
    PROJECTOR: Projector<EP = CS, Transformer = TRANSFORMER>,
    T: CoordFloat,
{
    /// Constructor.
    ///
    /// # Panics
    /// unwrap() is used here but a panic will never happen as 4.5 will always be converted into T.
    pub fn new(context_stream: CS, projection: PROJECTOR) -> Self {
        Self {
            context_stream,
            point_radius: PointRadiusEnum::Val(T::from(4.5_f64).unwrap()),
            projection,
        }
    }
}

impl<CS, PROJECTOR, T, TRANSFORMER> Path<CS, PROJECTOR, T, TRANSFORMER>
where
    CS: Clone + Default + Result,
    PROJECTOR: Projector<EP = CS, Transformer = TRANSFORMER>,
    TRANSFORMER: Stream<EP = CS, T = T>,
    T: CoordFloat + FloatConst,
{
    /// Combines projection, context stream and object.
    pub fn object(&mut self, object: &impl Streamable<T = T>) -> <CS as Result>::Out {
        let mut stream_in = self.projection.stream(&self.context_stream);
        object.to_stream(&mut stream_in);
        stream_in.endpoint().result()
    }
}

impl<CLIPC, PROJECTOR, T>
    Path<
        Measure<T>,
        PROJECTOR,
        T,
        StreamTransformRadians<Connected<RotatorRadians<Connected<CLIPC>, T>>>,
    >
where
    CLIPC: Stream<EP = Measure<T>, T = T>,
    PROJECTOR: Projector<
        EP = Measure<T>,
        Transformer = StreamTransformRadians<Connected<RotatorRadians<Connected<CLIPC>, T>>>,
    >,
    T: AddAssign + CoordFloat,
{
    /// Returns the area of the Path
    /// This operation consumes the  Path.
    pub fn measure(mut self, object: &impl Streamable<T = T>) -> T
    where
        T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let stream_dst = Measure::<T>::default();
        let mut stream_in = self.projection.stream(&stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.0.sink.endpoint().result()
    }
}

impl<CLIPC, PROJECTOR, T>
    Path<
        Area<T>,
        PROJECTOR,
        T,
        StreamTransformRadians<Connected<RotatorRadians<Connected<CLIPC>, T>>>,
    >
where
    CLIPC: Stream<EP = Area<T>, T = T>,
    PROJECTOR: Projector<
        EP = Area<T>,
        Transformer = StreamTransformRadians<Connected<RotatorRadians<Connected<CLIPC>, T>>>,
    >,

    T: CoordFloat,
{
    /// Returns the area of the Path
    /// This operation consumes the  Path.
    pub fn area(mut self, object: &impl Streamable<T = T>) -> T
    where
        T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let stream_dst = Area::<T>::default();
        let mut stream_in = self.projection.stream(&stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.0.sink.endpoint().result()
    }
}

impl<CLIPC, PROJECTOR, T>
    Path<
        Bounds<T>,
        PROJECTOR,
        T,
        StreamTransformRadians<Connected<RotatorRadians<Connected<CLIPC>, T>>>,
    >
where
    CLIPC: Stream<EP = Bounds<T>, T = T>,
    PROJECTOR: Projector<
        EP = Bounds<T>,
        Transformer = StreamTransformRadians<Connected<RotatorRadians<Connected<CLIPC>, T>>>,
    >,
    T: CoordFloat + FloatConst,
{
    /// Returns the bounds of the object
    ///
    /// This operation consumes the  Path.
    pub fn bounds(mut self, object: &impl Streamable<T = T>) -> [Coord<T>; 2] {
        let stream_dst = Bounds::<T>::default();
        let mut stream_in = self.projection.stream(&stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.endpoint().result()
    }
}

impl<PROJECTOR, T, TRANSFORMER> Path<Centroid<T>, PROJECTOR, T, TRANSFORMER>
where
    PROJECTOR: Projector<EP = Centroid<T>, Transformer = TRANSFORMER>,
    TRANSFORMER: Stream<EP = Centroid<T>, T = T>,
    T: AddAssign + CoordFloat + FloatConst,
{
    /// Returns the centroid of the object.
    pub fn centroid(mut self, object: &impl Streamable<T = T>) -> Coord<T> {
        let stream_dst = Centroid::<T>::default();
        let mut stream_in = self.projection.stream(&stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.endpoint().result()
    }
}

impl<CS, PROJECTOR, T, TRANSFORMER> Path<CS, PROJECTOR, T, TRANSFORMER>
where
    PROJECTOR: Projector<EP = CS, Transformer = TRANSFORMER>,
    T: CoordFloat,
{
    /// Sets the context stream.
    pub fn context(&mut self, context_stream: CS) -> &mut Self {
        self.context_stream = context_stream;
        self
    }
}
