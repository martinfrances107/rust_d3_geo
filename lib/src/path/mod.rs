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
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::path::area::Area;
use crate::path::bounds::Bounds;
use crate::path::centroid::Centroid;
use crate::projection::projector::Projector;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Streamable;

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
pub struct Path<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    p_pcnc: PhantomData<PCNC>,
    p_rc: PhantomData<RC>,
    context_stream: CS,
    point_radius: PointRadiusEnum<T>,
    /// don't store projection stream.
    projection: Projector<CLIPC, CLIPU, CS, PCNU, PR, RC, RU, T>,
}

impl<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T> Path<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
{
    /// Constructor.
    pub fn new(
        context_stream: CS,
        projection: Projector<CLIPC, CLIPU, CS, PCNU, PR, RC, RU, T>,
    ) -> Self {
        Self {
            p_pcnc: PhantomData::<PCNC>,
            p_rc: PhantomData::<RC>,
            context_stream,
            point_radius: PointRadiusEnum::Val(T::from(4.5_f64).unwrap()),
            projection,
        }
    }
}

impl<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T> Path<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPU: Clone + Connectable<Output = CLIPC, SC = RC>,
    CLIPC: Clone + Stream<EP = CS, T = T>,
    CS: Clone + Default + PartialEq + Result,
    PCNU: Clone + Connectable<Output = PCNC, SC = CS>,
    RC: Clone + Stream<EP = CS, T = T>,
    RU: Clone + Connectable<Output = RC, SC = PCNC>,
    T: 'static + CoordFloat + FloatConst,
{
    /// Combines projection, context stream and object.
    pub fn object(&mut self, object: &impl Streamable<T = T>) -> <CS as Result>::Out {
        let mut stream_in = self.projection.stream(&self.context_stream);
        object.to_stream(&mut stream_in);
        stream_in.endpoint().result()
    }
}

impl<CLIPC, CLIPU, PCNC, PCNU, PR, RC, RU, T> Path<CLIPC, CLIPU, Area<T>, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone + Stream<EP = Area<T>, T = T>,
    CLIPU: Clone + Connectable<Output = CLIPC, SC = RC>,
    PCNU: Clone + Connectable<Output = PCNC, SC = Area<T>>,
    RC: Clone + Stream<EP = Area<T>, T = T>,
    RU: Clone + Connectable<Output = RC, SC = PCNC>,
    T: CoordFloat,
{
    /// Returns the area of the Path
    /// This operation consumes the  Path.
    pub fn area(mut self, object: &impl Streamable<T = T>) -> T
    where
        T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let stream_dst = Area::default();
        let mut stream_in = self.projection.stream(&stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.0.sink.endpoint().result()
    }
}

impl<CLIPC, CLIPU, PCNC, PCNU, PR, RC, RU, T>
    Path<CLIPC, CLIPU, Bounds<T>, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone + Stream<EP = Bounds<T>, T = T>,
    CLIPU: Clone + Connectable<Output = CLIPC, SC = RC>,
    PCNU: Clone + Connectable<Output = PCNC, SC = Bounds<T>>,
    RC: Clone + Stream<EP = Bounds<T>, T = T>,
    RU: Clone + Connectable<Output = RC, SC = PCNC> + Debug,
    T: 'static + CoordFloat + FloatConst,
{
    /// Returns the bounds of the object
    ///
    /// This operation consumes the  Path.
    pub fn bounds(mut self, object: &impl Streamable<T = T>) -> [Coordinate<T>; 2] {
        let stream_dst = Bounds::default();
        let mut stream_in = self.projection.stream(&stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.endpoint().result()
    }
}

impl<CLIPC, CLIPU, PCNC, PCNU, PR, RC, RU, T>
    Path<CLIPC, CLIPU, Centroid<T>, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone + Stream<EP = Centroid<T>, T = T>,
    CLIPU: Clone + Connectable<Output = CLIPC, SC = RC>,
    PCNC: Stream<EP = Centroid<T>, T = T>,
    PCNU: Clone + Connectable<Output = PCNC, SC = Centroid<T>>,
    RC: Clone + Stream<EP = Centroid<T>, T = T>,
    RU: Clone + Connectable<Output = RC, SC = PCNC>,
    T: 'static + AddAssign + CoordFloat + FloatConst,
{
    /// Returns the centroid of the object.
    pub fn centroid(mut self, object: &impl Streamable<T = T>) -> Coordinate<T> {
        let stream_dst = Centroid::default();
        let mut stream_in = self.projection.stream(&stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.endpoint().result()
    }
}

impl<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T> Path<CLIPC, CLIPU, CS, PCNC, PCNU, PR, RC, RU, T>
where
    CLIPC: Clone,
    CLIPU: Clone,
    T: CoordFloat,
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
