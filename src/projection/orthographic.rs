use crate::clip::antimeridian::interpolate::Interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::projection::builder::Builder;
use crate::projection::Raw;
use crate::stream::Stream;
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
// use crate::projection::projection_trait::ProjectionTrait;
use crate::projection::scale::Scale;
// use crate::stream::Stream;
// use crate::Transform;

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
    pub fn gen_projection_mutator<'a, DRAIN>(
    ) -> Builder<DRAIN, Interpolate<T>, Line<T>, Orthographic<T>, PV<T>, T>
    where
        DRAIN: Stream<SC = Coordinate<T>>,
    {
        Builder::new(
            StreamNodeClipFactory::new(Interpolate::default(), Line::default(), PV::default()),
            Orthographic::default(),
            // None,
        )
        .scale(T::from(249.5_f64).unwrap())
        // .clip_angle(StreamOrValueMaybe::Value(T::from(90f64 + 1e-6f64).unwrap()))
        // .clip_angle(T::from(90_f64 + 1e-6_f64).unwrap())
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
