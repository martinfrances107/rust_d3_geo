use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;
use std::marker::PhantomData;

use super::projection::Projection;
use super::projection::StreamOrValueMaybe;
use super::projection_mutator::ProjectionMutator;
use crate::projection::azimuthal::azimuthal_invert;
use crate::Transform;
use std::rc::Rc;

/// Why the Phantom Data is required here...
///
/// The Transform trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Debug, Default)]
pub struct OrthographicRaw<T>
where
    T: CoordFloat + std::default::Default + 'static,
{
    phantom: PhantomData<T>,
}

impl<T> OrthographicRaw<T>
where
    T: CoordFloat + FloatConst + std::default::Default + 'static,
{
    pub fn gen_projection_mutator<'a>() -> ProjectionMutator<'a, T> {
        let s: Rc<Box<dyn Transform<C = Coordinate<T>>>> =
            Rc::new(Box::new(OrthographicRaw::default()));
        let mut projection = ProjectionMutator::from_projection_raw(s, None);
        projection.scale(T::from(249.5f64).unwrap());
        let angle = T::from(249.5f64).unwrap();
        projection.clip_angle(StreamOrValueMaybe::Value(angle));
        return projection;
    }
}

impl<T: CoordFloat + FloatConst + std::default::Default> Transform for OrthographicRaw<T> {
    type C = Coordinate<T>;
    #[inline]
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        Coordinate {
            x: p.y.cos() * p.x.sin(),
            y: p.y.sin(),
        }
    }

    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let f = Box::new(|z: T| z.asin());
        let g = azimuthal_invert(f);
        return g(p.x, p.y);
    }
}
