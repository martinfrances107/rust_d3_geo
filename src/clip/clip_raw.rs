// use std::cell::RefCell;
// use std::cmp::Ordering;
// use std::fmt::Display;
// use std::ops::AddAssign;
// use std::rc::Rc;

// use geo::CoordFloat;
// use geo::Coordinate;
// use num_traits::AsPrimitive;
// use num_traits::FloatConst;

// use crate::projection::projection_trait::ProjectionTrait;
// use crate::stream::CompareIntersection;
// use crate::stream::Stream;

// use super::antimeridian::ClipAntimeridian;
// use super::circle::ClipCircle;
// use super::rejoin::intersection::Intersection;
// use super::Clip;

// #[derive(Debug)]
// pub enum ClipRaw<A, PR, T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
//     Antimeridian(ClipAntimeridian< PR, T>),
//     Circle(ClipCircle<T>),
// }

// impl<P, PR, T> Default for ClipRaw<A, T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
//     fn default() -> Self {
//         ClipRaw::Antimeridian(ClipAntimeridian::default())
//     }
// }

// impl<A, PR, T> ClipRaw<T> for ClipRaw<A, PR, T>
// where
//     T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
// {
//     type SctC = Coordinate<T>;
//     type SctOC = Option<Coordinate<T>>;
//     type SctT = T;
//     type SctCi = CompareIntersection<T>;

//     // fn point_visible(&self, p: &Self::SctC, m: Option<u8>) -> bool {
//     //     match self {
//     //         ClipRaw::Antimeridian(c) => c.point_visible(p, m),
//     //         ClipRaw::Circle(c) => c.point_visible(p, m),
//     //     }
//     // }

//     fn interpolate(
//         &self,
//         from: Self::SctOC,
//         to: Self::SctOC,
//         direction: Self::SctT,
//         stream: &mut impl Stream,
//     ) {
//         match self {
//             ClipRaw::Antimeridian(c) => c.interpolate(from, to, direction, stream),
//             ClipRaw::Circle(c) => c.interpolate(from, to, direction, stream),
//         };
//     }
// }
