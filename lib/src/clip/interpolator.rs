use std::cmp::Ordering;
use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::math::EPSILON;
use crate::stream::Stream;

use super::Interpolator as InterpolatorTrait;

/// Interpolator used only by Rectangle.
#[derive(Clone, Debug)]
pub(super) struct Interpolator<T> {
    x0: T,
    y0: T,
    x1: T,
    y1: T,
    epsilon: T,
}

impl<T> Interpolator<T>
where
    T: CoordFloat + FloatConst,
{
    pub(super) fn new(x0: T, y0: T, x1: T, y1: T) -> Self {
        Self {
            x0,
            y0,
            x1,
            y1,
            epsilon: T::from(EPSILON).unwrap(),
        }
    }
}

impl<T> Interpolator<T>
where
    T: CoordFloat,
{
    #[inline]
    fn corner(&self, p: &Coordinate<T>, direction: &T) -> i8 {
        if (p.x - self.x0).abs() < self.epsilon {
            if direction > &T::zero() {
                0
            } else {
                3
            }
        } else if (p.x - self.x1).abs() < self.epsilon {
            if direction > &T::zero() {
                2
            } else {
                1
            }
        } else if (p.y - self.y0).abs() < self.epsilon {
            // Returns 1 or 0.
            (*direction > T::zero()).into()
        } else if direction > &T::zero() {
            3
        } else {
            2
        }
    }

    // Warning from JS a, b are LineElem.
    pub fn compare_point(&self, a: &Coordinate<T>, b: &Coordinate<T>) -> Ordering {
        let ca = self.corner(a, &T::one());
        let cb = self.corner(b, &T::one());
        if ca == cb {
            let diff = match ca {
                0 => b.y - a.y,
                1 => a.x - b.x,
                2 => a.y - b.y,
                _ => b.x - a.x,
            };
            if diff > T::zero() {
                Ordering::Greater
            } else if diff < T::zero() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        } else if (ca - cb) > 0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl<T> InterpolatorTrait for Interpolator<T>
where
    T: CoordFloat,
{
    type T = T;

    fn interpolate<EP, STREAM>(
        &self,
        from: Option<Coordinate<Self::T>>,
        to: Option<Coordinate<Self::T>>,
        direction: Self::T,
        stream: &mut STREAM,
    ) where
        STREAM: Stream<EP = EP, T = T>,
    {
        let direction_i8: i8 = T::to_i8(&direction).unwrap();
        match (to, from) {
            (Some(to), Some(from)) => {
                let mut a = self.corner(&from, &direction);
                let a1 = self.corner(&to, &direction);
                let cp = self.compare_point(&from, &to) == Ordering::Less;
                let is_direction = direction > T::zero();
                if a != a1 || (cp != is_direction) {
                    loop {
                        let p = Coordinate {
                            x: if a == 0 || a == 3 { self.x0 } else { self.x1 },
                            y: if a > 1 { self.y1 } else { self.y0 },
                        };
                        stream.point(&p, None);

                        a = (a + direction_i8 + 4) % 4;
                        if a == a1 {
                            break;
                        }
                    }
                } else {
                    stream.point(&to, None);
                }
            }
            (Some(_to), None) => {
                panic!("DO I ever get here");
            }
            _ => {
                panic!("did not expect only from and no to .. or Nothing at all Does the JS version get here?");
            }
        }
    }
}
