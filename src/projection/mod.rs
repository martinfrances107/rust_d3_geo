pub mod azimuthal;
pub mod azimuthal_equal_area;
pub mod center;
pub mod clip_extent;
pub mod equirectangular;
pub mod gnomic;
pub mod mecator;
pub mod orthographic;
pub mod projection;
pub mod projection_equal;
pub mod projection_trait;
pub mod resample;
pub mod scale;
pub mod scale_translate;
pub mod scale_translate_rotate;
pub mod stereographic;
pub mod stream_transform;
pub mod stream_transform_radians;
pub mod translate;

mod fit;

// use std::fmt::Display;
// use std::ops::AddAssign;

// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use crate::stream::Stream;
// use crate::Transform;
// use projection::Projection;

// use azimuthal_equal_area::AzimuthalEqualAreaRaw;
// use equirectangular::EquirectangularRaw;
// use gnomic::GnomicRaw;
// use mecator::MecatorRaw;
// use orthographic::OrthographicRaw;
// use stereographic::StereographicRaw;

// pub trait ProjectionRawTrait: Transform + Clone{
//     // type T: AddAssign + CoordFloat +Display + FloatConst;
//     fn gen_projection_mutator<PR, SD, T>() -> Projection< PR, SD, T>
//     where
//         SD: Stream + Default,
//         PR: Clone +Transform,

// }

// #[derive(Clone, Debug)]
// pub enum ProjectionRawEnum<T>
// where
//     T: CoordFloat + Default,
// {
//     A(AzimuthalEqualAreaRaw<T>),
//     E(EquirectangularRaw<T>),
//     O(OrthographicRaw<T>),
//     G(GnomicRaw<T>),
//     M(MecatorRaw<T>),
//     S(StereographicRaw<T>),
// }

// impl<T> Transform for ProjectionRawEnum<T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst,
// {
//     type C = Coordinate<T>;
//     fn transform(&self, p: &Self::C) -> Self::C {
//         match self {
//             ProjectionRawEnum::A(a) => a.transform(p),
//             ProjectionRawEnum::E(e) => e.transform(p),
//             ProjectionRawEnum::G(g) => g.transform(p),
//             ProjectionRawEnum::O(o) => o.transform(p),
//             ProjectionRawEnum::M(m) => m.transform(p),
//             ProjectionRawEnum::S(s) => s.transform(p),
//         }
//     }
//     fn invert(&self, p: &Self::C) -> Self::C {
//         match self {
//             ProjectionRawEnum::A(a) => a.invert(p),
//             ProjectionRawEnum::E(e) => e.invert(p),
//             ProjectionRawEnum::G(g) => g.invert(p),
//             ProjectionRawEnum::O(o) => o.invert(p),
//             ProjectionRawEnum::M(m) => m.invert(p),
//             ProjectionRawEnum::S(s) => s.invert(p),
//         }
//     }
// }
