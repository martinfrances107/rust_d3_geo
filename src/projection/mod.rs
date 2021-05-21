pub mod azimuthal;
pub mod azimuthal_equal_area;
pub mod equirectangular;
mod fit;
pub mod mecator;
pub mod orthographic;
pub mod projection;
pub mod projection_equal;
pub mod projection_mutator;
pub mod resample;
pub mod scale_translate_rotate;
pub mod stereographic;
pub mod stream_transform;
pub mod stream_transform_radians;

mod scale_translate;

use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::Transform;

use azimuthal_equal_area::AzimuthalEqualAreaRaw;
use equirectangular::EquirectangularRaw;
use mecator::MecatorRaw;
use orthographic::OrthographicRaw;
use stereographic::StereographicRaw;

#[derive(Clone, Debug)]
pub enum ProjectionRawEnum<T>
where
    T: CoordFloat + Default,
{
    A(AzimuthalEqualAreaRaw<T>),
    E(EquirectangularRaw<T>),
    O(OrthographicRaw<T>),
    M(MecatorRaw<T>),
    S(StereographicRaw<T>),
}

impl<T> Transform for ProjectionRawEnum<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type TcC = Coordinate<T>;
    fn transform(&self, p: &Self::TcC) -> Self::TcC {
        match self {
            ProjectionRawEnum::A(a) => a.transform(p),
            ProjectionRawEnum::E(e) => e.transform(p),
            ProjectionRawEnum::O(o) => o.transform(p),
            ProjectionRawEnum::M(m) => m.transform(p),
            ProjectionRawEnum::S(s) => s.transform(p),
        }
    }
    fn invert(&self, p: &Self::TcC) -> Self::TcC {
        match self {
            ProjectionRawEnum::A(a) => a.invert(p),
            ProjectionRawEnum::E(e) => e.invert(p),
            ProjectionRawEnum::O(o) => o.invert(p),
            ProjectionRawEnum::M(m) => m.invert(p),
            ProjectionRawEnum::S(s) => s.invert(p),
        }
    }
}
