use std::fmt::Debug;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::float::FloatConst;

use crate::clip::antimeridian::ClipAntimeridianC;
use crate::math::EPSILON;
use crate::projection::builder::types::BuilderAntimeridianResampleNoClip;
use crate::projection::ScaleSet;
use crate::Transform;

use super::builder::template::ResampleNoPCNC;
use super::builder::Builder;
use super::BuilderTrait;
use super::RawBase;

/// Equirectangular
/// Used to define a projection builder.
#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct EqualEarth<T> {
    A1: T,
    A2: T,
    A3: T,
    A4: T,
    M: T,
    three: T,
    seven: T,
    nine: T,
    epsilon: T,
    iterations: u8,
}

impl<T> Default for EqualEarth<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            A1: T::from(1.340_264_f64).unwrap(),
            A2: T::from(-0.081_106_f64).unwrap(),
            A3: T::from(0.008_93_f64).unwrap(),
            A4: T::from(0.003_796).unwrap(),
            M: T::from(3_f64.sqrt() / 2_f64).unwrap(),
            three: T::from(3_f64).unwrap(),
            seven: T::from(7_f64).unwrap(),
            nine: T::from(9_f64).unwrap(),
            epsilon: T::from(EPSILON).unwrap(),
            iterations: 12,
        }
    }
}

impl<T> RawBase for EqualEarth<T>
where
    T: CoordFloat + Default + FloatConst,
{
    type Builder<DRAIN: Clone> = BuilderAntimeridianResampleNoClip<DRAIN, Self, T>;

    #[inline]
    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN> {
        let mut b = Builder::new(Self::default());
        b.scale_set::<ClipAntimeridianC<ResampleNoPCNC<DRAIN, Self, T>, T>>(
            T::from(177.158_f64).unwrap(),
        );
        b
    }
}

impl<T> Transform for EqualEarth<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        let l = (self.M * p.y.sin()).asin();
        let l2 = l * l;
        let l6 = l2 * l2 * l2;
        Coord {
            x: p.x * l.cos()
                / (self.M
                    * (self.A1
                        + self.three * self.A2 * l2
                        + l6 * (self.seven * self.A3 + self.nine * self.A4 * l2))),
            y: l * (self.A1 + self.A2 * l2 + l6 * (self.A3 + self.A4 * l2)),
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        let mut l = p.y;
        let mut l2 = l * l;
        let mut l6 = l2 * l2 * l2;

        for _ in 0..self.iterations {
            let fy = l * (self.A1 + self.A2 * l2 + l6 * (self.A3 + self.A4 * l2)) - p.y;
            let fpy = self.A1
                + self.three * self.A2 * l2
                + l6 * (self.seven * self.A3 + self.nine * self.A4 * l2);
            let delta = fy / fpy;
            l = l - delta;
            l2 = l * l;
            l6 = l2 * l2 * l2;
            if delta.abs() < self.epsilon {
                break;
            }
        }

        Coord {
            x: self.M
                * p.x
                * (self.A1
                    + self.three * self.A2 * l2
                    + l6 * (self.seven * self.A3 + self.nine * self.A4 * l2))
                / l.cos(),
            y: (l.sin() / self.M).asin(),
        }
    }
}
