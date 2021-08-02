use crate::projection::azimuthal_equal_area::AzimuthalEqualArea;
use crate::projection::projection::Projection;
use crate::projection::stereographic::Stereographic;
use crate::stream::Stream;
use crate::Transform;
use num_traits::Float;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::projection::projection_trait::ProjectionTrait;
use crate::projection::scale::Scale;

#[derive(Copy, Clone, Debug)]
pub struct ProjectionGenerator<DRAIN, PR, T>
where
    T: CoordFloat,
{
    phantomT: PhantomData<T>,
    phantomPR: PhantomData<PR>,
    phantomDrain: PhantomData<DRAIN>,
}

/// @todo must implement this trait for every raw projection type
/// Not just AzimuthalEqualArea and Stereographic
trait Generate {
    type Drain;
    type PR;
    type T;
    fn gen<'a>(s: &'a Self::PR) -> Projection<'a, Self::Drain, Self::PR, Self::T>
    where
        <Self as Generate>::Drain: Clone + Default + Stream<SC = Coordinate<<Self as Generate>::T>>,
        <Self as Generate>::PR: Clone + Transform<C = Coordinate<<Self as Generate>::T>>,
        <Self as Generate>::T: AddAssign
            + AsPrimitive<<Self as Generate>::T>
            + Default
            + Debug
            + Display
            + Float
            + FloatConst;
}

impl<T, DRAIN> Generate for ProjectionGenerator<DRAIN, AzimuthalEqualArea<T>, T>
where
    //     DRAIN: 'a + Default + Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type Drain = DRAIN;
    type PR = AzimuthalEqualArea<T>;
    type T = T;
    fn gen<'a>(pr: &'a Self::PR) -> Projection<'a, DRAIN, AzimuthalEqualArea<T>, T>
    where
        <Self as Generate>::Drain: Clone + Default + Stream<SC = Coordinate<<Self as Generate>::T>>,
        <Self as Generate>::PR: Clone + Transform<C = Coordinate<<Self as Generate>::T>>,
        <Self as Generate>::T:
            AddAssign + AsPrimitive<<Self as Generate>::T> + Debug + Display + Float + FloatConst,
    {
        Projection::new(pr, None)
            .scale(T::from(124.75f64).unwrap())
            .clip_angle(T::from(180f64 - 1e-3).unwrap())
    }
}

impl<DRAIN, T> Generate for ProjectionGenerator<DRAIN, Stereographic<T>, T>
where
    // DRAIN: 'a + Default + Stream<SC = Coordinate<T>>,
    T: CoordFloat,
{
    type Drain = DRAIN;
    type PR = Stereographic<T>;
    type T = T;

    fn gen<'a>(s: &'a Self::PR) -> Projection<'a, DRAIN, Stereographic<T>, T>
    where
        <Self as Generate>::Drain: Clone + Default + Stream<SC = Coordinate<<Self as Generate>::T>>,
        <Self as Generate>::PR: Clone + Transform<C = Coordinate<<Self as Generate>::T>>,
        <Self as Generate>::T: AddAssign
            + AsPrimitive<<Self as Generate>::T>
            + Default
            + Debug
            + Display
            + Float
            + FloatConst,
    {
        Projection::new(s, None)
            .scale(T::from(250f64).unwrap())
            // .clip_angle(StreamOrValueMaybe::Value(T::from(142f64).unwrap()))
            .clip_angle(T::from(142f64).unwrap())
    }
}
