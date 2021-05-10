mod intersect;
pub mod line;

use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::CompareIntersection;
use crate::stream::Stream;
use crate::stream::{Clean, CleanEnum};

use super::clip::Clip;
use super::clip_base::ClipBase;
use super::line_elem::LineElem;
use super::ClipTraitRaw;
use crate::clip::clip_raw::ClipRaw;

#[derive(Clone, Default, Debug)]
pub struct ClipAntimeridian<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    pub base: ClipBase<T>,
}

impl<T> ClipAntimeridian<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    #[inline]
    pub fn gen_clip() -> Clip<T> {
        let start = LineElem {
            p: Coordinate {
                x: -T::PI(),
                y: -T::PI() / T::from(2u8).unwrap(),
            },
            m: None,
        };
        Clip::new(ClipRaw::Antimeridian(ClipAntimeridian::default()), start)
    }
}

impl<T> Clean for ClipAntimeridian<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    /// A clip trait.
    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    fn clean(&self) -> CleanEnum {
        panic!("must relate code to enum")
    }
}

impl<T> ClipTraitRaw<T> for ClipAntimeridian<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type SctC = Coordinate<T>;
    type SctOC = Option<Coordinate<T>>;
    type SctT = T;
    type SctCi = CompareIntersection<T>;

    #[inline]
    fn point_visible(&self, _p: &Coordinate<T>, _z: Option<u8>) -> bool {
        true
    }

    fn interpolate(
        &self,
        from: Option<Coordinate<T>>,
        to: Option<Coordinate<T>>,
        direction: T,
        stream: &mut impl Stream<T, C = Coordinate<T>>,
    ) {
        let phi: T;
        match from {
            None => {
                phi = direction * T::FRAC_PI_2();
                stream.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: phi,
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: T::zero(),
                        y: phi,
                    },
                    None,
                );
                stream.point(&Coordinate { x: T::PI(), y: phi }, None);
                stream.point(
                    &Coordinate {
                        x: T::PI(),
                        y: T::zero(),
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: T::PI(),
                        y: -phi,
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: T::zero(),
                        y: -phi,
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: -phi,
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: T::zero(),
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: phi,
                    },
                    None,
                );
            }
            Some(from) => {
                let to = to.unwrap();
                if (from.x - to.x).abs() > T::epsilon() {
                    let lambda = if from.x < to.x { T::PI() } else { -T::PI() };

                    phi = direction * lambda / T::from(2).unwrap();
                    stream.point(&Coordinate { x: -lambda, y: phi }, None);
                    stream.point(
                        &Coordinate {
                            x: T::zero(),
                            y: phi,
                        },
                        None,
                    );
                    stream.point(&Coordinate { x: lambda, y: phi }, None);
                } else {
                    stream.point(&to, None);
                }
            }
        }
    }
}
