use std::marker::PhantomData;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::float::FloatConst;

use super::projection::Projection;
use super::projection::StreamOrValueMaybe;
use super::projection_mutator::ProjectionMutator;
use super::ProjectionRawEnum;
use crate::Transform;
// use crate::TransformClone;

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
    T: AddAssign + CoordFloat + FloatConst + Default,
{
    pub fn gen_projection_mutator() -> ProjectionMutator<T> {
        // let s: Rc<Box<dyn Transform<TcC = Coordinate<T>>>> =
        //     Rc::new(Box::new(StereographicRaw::default()));
        let s = ProjectionRawEnum::S(StereographicRaw::default());
        let projection = ProjectionMutator::from_projection_raw(s, None);
        projection
            .scale(T::from(250f64).unwrap())
            .clip_angle(StreamOrValueMaybe::Value(T::from(142f64).unwrap()))
    }

    #[inline]
    fn angle(z: T) -> T
    where
        T: CoordFloat + FloatConst + Default,
    {
        // Find a way to optimize this ... need a static of type T with value 2.
        T::from(2).unwrap() * z.atan()
    }

    pub fn azimuthal_invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let z = (p.x * p.x + p.y * p.y).sqrt();
        let c = StereographicRaw::angle(z);
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
        // })
    }
}

// impl<'a, T: CoordFloat + FloatConst> TransformClone<'a> for StereographicRaw<T> {
//     fn box_clone(&'a self) -> Box<dyn TransformClone<'a, TcC = Self::TcC>> {
//         Box::new(self.clone())
//     }
// }

impl<T: AddAssign + CoordFloat + FloatConst + Default> Transform for StereographicRaw<T> {
    type TcC = Coordinate<T>;
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
        self.azimuthal_invert(p)
    }
}
