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
use crate::path::PointRadiusEnum;
use crate::path::Result;
use crate::projection::projector_identity::transformer::Transformer;
use crate::projection::projector_identity::Projector;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Streamable;

pub mod builder;

/// Projection and context stream applied to a Streamable.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Path<CS, PCNC, PCNU, T>
where
    PCNC: Clone,
    PCNU: Clone,
    T: CoordFloat,
{
    p_pcnc: PhantomData<PCNC>,

    context_stream: CS,
    point_radius: PointRadiusEnum<T>,
    /// don't store projection stream.
    projection: Projector<CS, PCNC, PCNU, T>,
}

impl<CS, PCNC, PCNU, T> Path<CS, PCNC, PCNU, T>
where
    PCNC: Clone,
    PCNU: Clone,
    T: CoordFloat,
{
    /// Constructor.
    pub fn new(context_stream: CS, projection: Projector<CS, PCNC, PCNU, T>) -> Self {
        Self {
            p_pcnc: PhantomData::<PCNC>,
            context_stream,
            point_radius: PointRadiusEnum::Val(T::from(4.5_f64).unwrap()),
            projection,
        }
    }
}

impl<CS, PCNC, PCNU, T> Path<CS, PCNC, PCNU, T>
where
    CS: Clone + Default + PartialEq + Result,
    PCNC: Clone + Stream<EP = CS, T = T>,
    PCNU: Clone + Connectable<Output = PCNC, SC = CS>,
    Transformer<CS, PCNC, Connected<PCNC>, T>: Stream<EP = CS, T = T>,
    T: 'static + CoordFloat + FloatConst,
{
    /// Combines projection, context stream and object.
    pub fn object(&mut self, object: &impl Streamable<T = T>) -> <CS as Result>::Out {
        let mut stream_in = self.projection.stream(&self.context_stream);
        object.to_stream(&mut stream_in);
        stream_in.endpoint().result()
    }
}

impl<PCNC, PCNU, T> Path<Area<T>, PCNC, PCNU, T>
where
    PCNC: Clone + Stream<EP = Area<T>, T = T>,
    PCNU: Clone + Connectable<Output = PCNC, SC = Area<T>>,
    Transformer<Area<T>, PCNC, Connected<PCNC>, T>: Stream<EP = Area<T>, T = T>,
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

        // stream_in.0.sink.endpoint().result()
        todo!();
    }
}

impl<PCNC, PCNU, T> Path<Bounds<T>, PCNC, PCNU, T>
where
    PCNC: Clone + Stream<EP = Bounds<T>, T = T>,
    PCNU: Clone + Connectable<Output = PCNC, SC = Bounds<T>>,
    Transformer<Bounds<T>, PCNC, Connected<PCNC>, T>: Stream<EP = Bounds<T>, T = T>,
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

impl<PCNC, PCNU, T> Path<Centroid<T>, PCNC, PCNU, T>
where
    PCNC: Clone + Stream<EP = Centroid<T>, T = T>,
    PCNU: Clone + Connectable<Output = PCNC, SC = Centroid<T>>,
    Transformer<Centroid<T>, PCNC, Connected<PCNC>, T>: Stream<EP = Centroid<T>, T = T>,

    T: 'static + AddAssign + CoordFloat + FloatConst,
{
    /// Returns the centroid of the object.
    pub fn centroid(mut self, object: &impl Streamable<T = T>) -> Coordinate<T> {
        let stream_dst = Centroid::default();
        let mut stream_in: Transformer<Centroid<T>, PCNC, Connected<PCNC>, T> =
            self.projection.stream(&stream_dst);
        object.to_stream(&mut stream_in);

        stream_in.endpoint().result()
    }
}

impl<CS, PCNC, PCNU, T> Path<CS, PCNC, PCNU, T>
where
    PCNC: Clone + Stream<EP = CS, T = T>,
    PCNU: Clone + Connectable<Output = PCNC, SC = Centroid<T>>,
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
