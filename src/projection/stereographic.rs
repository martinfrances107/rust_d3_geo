use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::Transform;

use super::azimuthal::azimuthal_invert;
// use super::projection::Projection;
// use super::projection::StreamOrValueMaybe;
// use super::ProjectionRawTrait;
/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug, Default)]
pub struct StereographicRaw<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

impl<T> StereographicRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    // #[inline]
    // pub fn gen_projection_mutator() -> Projection<StereographicRaw<T>, T> {
    //     Projection::new(StereographicRaw::default(), None)
    //         .scale(T::from(250f64).unwrap())
    //         .clip_angle(StreamOrValueMaybe::Value(T::from(142f64).unwrap()))
    // }

    #[inline]
    fn z(z: T) -> T
    where
        T: CoordFloat + Default + FloatConst,
    {
        // Find a way to optimize this ... need a static of type T with value 2.
        T::from(2).unwrap() * z.atan()
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Transform
    for StereographicRaw<T>
{
    type C = Coordinate<T>;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let cy = p.y.cos();
        let k = T::one() + p.x.cos() * cy;
        Coordinate {
            x: cy * p.x.sin() / k,
            y: p.y.sin() / k,
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        azimuthal_invert(p, Self::z)
    }
}
