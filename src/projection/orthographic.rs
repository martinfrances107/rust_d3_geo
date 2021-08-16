use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use num_traits::AsPrimitive;

use crate::clip::circle::interpolate::generate as gen_interpolate;
use crate::clip::circle::line::Line;
use crate::clip::circle::pv::PV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::stream::Stream;
use crate::Transform;

use super::builder::Builder;
use super::scale::Scale;
use super::Raw;

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Copy, Debug)]
pub struct Orthographic<T>
where
    T: CoordFloat,
{
    phantom: PhantomData<T>,
}

impl<T> Default for Orthographic<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Orthographic {
            phantom: PhantomData::<T>,
        }
    }
}

impl<T> Raw for Orthographic<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type T = T;
}

impl<T> Orthographic<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    #[inline]
    pub fn gen_projection_builder<DRAIN>() -> Builder<DRAIN, Line<T>, Orthographic<T>, PV<T>, T>
    where
        DRAIN: Stream<SC = Coordinate<T>>,
    {
        Builder::new(
            StreamNodeClipFactory::new(
                gen_interpolate(T::one()),
                Line::<T>::default(),
                PV::default(),
            ),
            Orthographic::default(),
        )
        .scale(T::from(249.5_f64).unwrap())
        .clip_angle(T::from(90_f64 + 1e-6_f64).unwrap())
    }
}

// impl<T> ProjectionRawTrait for OrthographicRaw<T>
// // where
// //     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
// }

impl<T> Orthographic<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    #[inline]
    fn angle(z: T) -> T {
        z.asin()
    }

    pub fn azimuthal_invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let z = (p.x * p.x + p.y * p.y).sqrt();
        let c = Orthographic::angle(z);
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

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Transform
    for Orthographic<T>
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
