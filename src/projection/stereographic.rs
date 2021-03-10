use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use super::projection::Projection;
use super::projection::StreamOrValueMaybe;
use super::projection_mutator::ProjectionMutator;

use crate::projection::azimuthal::azimuthal_invert;
use crate::Transform;
use crate::TransformClone;

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
    T: CoordFloat + FloatConst + Default + 'static,
{
    pub fn gen_projection_mutator<'a>() -> ProjectionMutator<T> {
        // let s: Rc<Box<dyn Transform<TcC = Coordinate<T>>>> =
        //     Rc::new(Box::new(StereographicRaw::default()));
        let s = Box::new(StereographicRaw::default());
        let mut projection = ProjectionMutator::from_projection_raw(s, None);
        projection.scale(T::from(250f64).unwrap());
        projection.clip_angle(StreamOrValueMaybe::Value(T::from(142f64).unwrap()));
        return projection;
    }
}

impl<T: CoordFloat + FloatConst + 'static> TransformClone for StereographicRaw<T> {
    type TcC = Coordinate<T>;
    fn box_clone(&self) -> Box<dyn Transform<TcC = Self::TcC>> {
        Box::new(self.clone())
    }
}

impl<T: CoordFloat + FloatConst + 'static> Transform for StereographicRaw<T> {
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let cy = p.y.cos();
        let k = T::one() + p.x.cos() * cy;
        Coordinate {
            x: cy * p.x.sin() / k,
            y: p.y.sin() / k,
        }
    }

    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let f = Box::new(|z: T| T::from(2).unwrap() * z.atan());
        let g = azimuthal_invert(f);
        g(p.x, p.y)
    }
}
