pub mod equirectangular;
pub mod orthographic;
pub mod projection;
pub mod projection_equal;
pub mod projection_mutator;
pub mod resample;
pub mod scale_translate_rotate;
pub mod stereographic;
pub mod stream_transform;
pub mod stream_transform_radians;

mod azimuthal;
mod scale_translate;

use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::Transform;

use equirectangular::EquirectangularRaw;
use orthographic::OrthographicRaw;
use stereographic::StereographicRaw;

#[derive(Clone, Debug)]
pub enum ProjectionRawEnum<T>
where
    T: CoordFloat + Default,
{
    E(EquirectangularRaw<T>),
    O(OrthographicRaw<T>),
    S(StereographicRaw<T>),
}

impl<T> Transform for ProjectionRawEnum<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    type TcC = Coordinate<T>;
    fn transform(&self, p: &Self::TcC) -> Self::TcC {
        match self {
            ProjectionRawEnum::E(e) => e.transform(p),
            ProjectionRawEnum::O(o) => o.transform(p),
            ProjectionRawEnum::S(s) => s.transform(p),
        }
    }
    fn invert(&self, p: &Self::TcC) -> Self::TcC {
        match self {
            ProjectionRawEnum::E(e) => e.invert(p),
            ProjectionRawEnum::O(o) => o.invert(p),
            ProjectionRawEnum::S(s) => s.invert(p),
        }
    }
}
