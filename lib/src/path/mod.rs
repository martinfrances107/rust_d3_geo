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
/// Path String.
pub mod string;

use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::AddAssign;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::buffer::Buffer;
use crate::clip::Bufferable;
use crate::clip::Interpolator;
use crate::clip::LineConnected;
use crate::clip::PointVisible;
use crate::path::area::Area;
use crate::path::bounds::Bounds;
use crate::path::centroid::Centroid;
use crate::projection::projector::Projector;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Streamable;
use crate::Transform;

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
    // TODO must add getter here.
    // There are complication about the mix return type here.
    // Context or PathString
    // fn get_point_radius...
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
pub struct Path<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    CS: Clone,
    I: Clone,
    LC: Clone,
    LB: Clone,
    LU: Clone,
    RC: Clone,
    RU: Clone,
    PCNU: Clone,
    PR: Clone,
    PV: Clone,
    T: CoordFloat + FloatConst,
{
    context_stream: CS,
    point_radius: PointRadiusEnum<T>,
    /// don't store projection stream.
    projection: Projector<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>,
}

impl<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Path<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    CS: Clone,
    I: Clone,
    LC: Clone,
    LB: Clone,
    LU: Clone,
    RC: Clone,
    RU: Clone,
    PR: Clone + Transform<T = T>,
    PV: Clone,
    PCNU: Clone,

    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Constructor.
    pub fn new(
        context_stream: CS,
        projection: Projector<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>,
    ) -> Self {
        Self {
            context_stream,
            point_radius: PointRadiusEnum::Val(T::from(4.5_f64).unwrap()),
            projection,
        }
    }
}

impl<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Path<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    I: Clone,
    LB: Clone,
    LC: Clone,
    LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T> + Debug,
    RC: Clone + Stream<EP = CS, T = T>,
    RU: Clone + Debug,
    PCNU: Clone,
    PV: Clone,
    PR: Clone + Debug,
    CS: Stream<EP = CS, T = T> + Result + PartialEq + Default,
    I: Interpolator<EP = CS, Stream = RC, T = T>,
    LB: Clone + LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: LineConnected<SC = RC> + Stream<EP = CS, T = T>,
    PCNU: Connectable<Output = PCNC, SC = CS>,
    PV: PointVisible<T = T>,
    RU: Connectable<Output = RC, SC = PCNC>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Combines projection, context stream and object.
    pub fn object(&mut self, object: &impl Streamable<T = T>) -> <CS as Result>::Out {
        let mut stream_in = self.projection.stream(self.context_stream.clone());
        object.to_stream(&mut stream_in);
        stream_in.get_endpoint().result()
    }
}

impl<I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Path<Area<T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    I: Interpolator<EP = Area<T>, Stream = RC, T = T>,
    LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: LineConnected<SC = RC> + Stream<EP = Area<T>, T = T>,
    LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T> + Debug,
    RC: Clone + Stream<EP = Area<T>, T = T>,
    RU: Clone + Debug + Connectable<Output = RC, SC = PCNC>,
    PCNU: Connectable<Output = PCNC, SC = Area<T>>,
    PV: Clone + PointVisible<T = T>,
    I: Clone,
    LC: Clone,
    LB: Clone,
    LU: Clone,
    RC: Clone,
    RU: Clone,
    PCNU: Clone,
    PR: Transform<T = T>,
    PV: Clone,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Returns the area of the Path
    /// This operation consumes the  Path.
    pub fn area(mut self, object: &impl Streamable<T = T>) -> T
    where
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let stream_dst = Area::default();
        let mut stream_in = self.projection.stream(stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.0.sink.get_endpoint().result()
    }
}

impl<I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Path<Bounds<T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    I: Interpolator<EP = Bounds<T>, Stream = RC, T = T>,
    LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: LineConnected<SC = RC> + Stream<EP = Bounds<T>, T = T>,
    LU: Clone,
    RU: Clone,
    LU: Clone + Connectable<Output = LC, SC = RC> + Bufferable<Output = LB, T = T> + Debug,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Debug + Connectable<Output = RC, SC = PCNC>,
    PCNU: Connectable<Output = PCNC, SC = Bounds<T>>,
    PCNU: Clone,
    PR: Transform<T = T>,
    PV: Clone + PointVisible<T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Returns the bounds of the object
    ///
    /// This operation consumes the  Path.
    pub fn bounds(mut self, object: &impl Streamable<T = T>) -> [Coordinate<T>; 2] {
        let stream_dst = Bounds::default();
        let mut stream_in = self.projection.stream(stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.get_endpoint().result()
    }
}

impl<LB, LC, LU, I, PCNC, PCNU, PR, PV, RC, RU, T>
    Path<Centroid<T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    I: Interpolator<EP = Centroid<T>, Stream = RC, T = T>,
    LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    LC: LineConnected<SC = RC> + Stream<EP = Centroid<T>, T = T>,
    LU: Bufferable<Output = LB, T = T>
        + Connectable<Output = LC, SC = RC>
        + Connectable<Output = LC, SC = RC>
        + Clone
        + Debug,
    PCNC: Stream<EP = Centroid<T>, T = T>,
    PCNU: Connectable<Output = PCNC, SC = Centroid<T>>,
    PV: PointVisible<T = T>,
    RU: Connectable<Output = RC, SC = PCNC>,
    RC: Clone + Stream<EP = Centroid<T>, T = T>,
    RU: Clone + Debug,
    PCNU: Clone,
    PR: Transform<T = T>,
    PV: Clone,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Returns the centroid of the object.
    pub fn centroid(mut self, object: &impl Streamable<T = T>) -> Coordinate<T> {
        let stream_dst = Centroid::default();
        let mut stream_in = self.projection.stream(stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.get_endpoint().result()
    }
}

impl<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Path<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    CS: Clone,
    I: Clone,
    LB: Clone,
    LC: Clone,
    LU: Clone,
    RC: Clone,
    RU: Clone,
    PV: Clone,
    PR: Clone + Transform<T = T>,
    PCNU: Clone,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Sets the context stream.
    pub fn context(mut self, context_stream: CS) -> Self {
        self.context_stream = context_stream;
        self
    }

    #[inline]
    fn point_radius(mut self, input: PointRadiusEnum<T>) -> Self {
        self.point_radius = match input {
            PointRadiusEnum::F(ref _input_fn) => input,
            PointRadiusEnum::Val(_input_value) => input,
        };
        self
    }
}
