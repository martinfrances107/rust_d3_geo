use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::Transform;

// use super::projection::Projection;
// use super::projection::StreamOrValueMaybe;
// use super::ProjectionRawTrait;

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug, Default)]
pub struct OrthographicRaw<T>
where
    T: CoordFloat + Default,
{
    phantom: PhantomData<T>,
}

// impl<T> ProjectionRawTrait for OrthographicRaw<T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
//     // #[inline]
//     // fn gen_projection_mutator() -> Projection<OrthographicRaw<T>, T> {
//     //     Projection::new(OrthographicRaw::default(), None)
//     //         .scale(T::from(249.5f64).unwrap())
//     //         .clip_angle(StreamOrValueMaybe::Value(T::from(90f64 + 1e-6f64).unwrap()))
//     // }
// }

// impl<T> ProjectionRawTrait for OrthographicRaw<T>
// // where
// //     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
// }

impl<T> OrthographicRaw<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    #[inline]
    fn angle(z: T) -> T {
        z.asin()
    }

    pub fn azimuthal_invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let z = (p.x * p.x + p.y * p.y).sqrt();
        let c = OrthographicRaw::angle(z);
        let sc = c.sin();
        let cc = c.cos();

        let ret_x = (p.x * sc).atan2(z * cc);
        let y_out;
        if z == T::zero() {
            y_out = z;
        } else {
            y_out = p.y * sc / z;
        }
        let ret_y = y_out.asin();

        Coordinate { x: ret_x, y: ret_y }
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Transform
    for OrthographicRaw<T>
{
    type C = Coordinate<T>;
    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        Coordinate {
            x: p.y.cos() * p.x.sin(),
            y: p.y.sin(),
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        self.azimuthal_invert(p)
    }
}
