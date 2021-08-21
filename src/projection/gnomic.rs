use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use crate::clip::antimeridian::interpolate::generate as gen_interpolate;
use crate::clip::circle::line::Line;
use crate::clip::circle::pv::PV;
use crate::clip::stream_node_clip_factory::StreamNodeClipFactory;
use crate::stream::Stream;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::builder::Builder;
use super::scale::Scale;
use super::Raw;

/// Why the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Copy, Clone, Debug)]
pub struct Gnomic<T>
where
    T: CoordFloat + FloatConst,
{
    phantom: PhantomData<T>,
}

impl<T> Default for Gnomic<T>
where
    T: CoordFloat + FloatConst,
{
    fn default() -> Self {
        Gnomic {
            phantom: PhantomData::<T>,
            // lambda: T::zero(),
            // phi: T::zero(),
        }
    }
}

impl<T> Raw<T> for Gnomic<T>
where
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;
}

impl<T> Gnomic<T>
where
    T: 'static + CoordFloat + FloatConst,
{
    pub fn gen_projection_builder<DRAIN>() -> Builder<DRAIN, Line<T>, Gnomic<T>, PV<T>, T>
    where
        DRAIN: Stream<T = T>,
    {
        let g = Gnomic::default();
        Builder::new(
            StreamNodeClipFactory::new(gen_interpolate(), Line::<T>::default(), PV::<T>::default()),
            g,
        )
        .scale(T::from(144.049_f64).unwrap())
        .clip_angle(T::from(60_f64).unwrap())
    }

    #[inline]
    fn atan(z: T) -> T
    where
        T: CoordFloat + FloatConst,
    {
        // Find a way to optimize this ... need a static of type T with value 2.
        z.atan()
    }
}

impl<T> Transform for Gnomic<T>
where
    T: 'static + CoordFloat + FloatConst,
{
    type T = T;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let cy = p.y.cos();
        let k = p.x.cos() * cy;
        Coordinate {
            x: cy * p.x.sin() / k,
            y: p.y.sin() / k,
        }
    }

    #[inline]
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        azimuthal_invert(p, Self::atan)
    }
}
