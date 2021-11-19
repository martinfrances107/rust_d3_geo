use std::ops::AddAssign;

use derivative::*;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::Result;
use super::ResultEnum;

#[allow(non_snake_case)]
#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
/// TODO Enforce positive area for exterior, negative area for interior?
pub struct Centroid<T>
where
    T: CoordFloat,
{
    X0: T,
    Y0: T,
    Z0: T,
    X1: T,
    Y1: T,
    Z1: T,
    X2: T,
    Y2: T,
    Z2: T,
    x00: T,
    y00: T,
    x0: T,
    y0: T,

    #[derivative(Debug = "ignore")]
    point_fn: fn(&mut Self, &Coordinate<T>),
    #[derivative(Debug = "ignore")]
    line_start_fn: fn(&mut Self),
    #[derivative(Debug = "ignore")]
    line_end_fn: fn(&mut Self),

    // Generic constant
    frac_1_2: T,
    three: T,
}

impl<T> Default for Centroid<T>
where
    T: AddAssign<T> + CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self {
            X0: T::zero(),
            Y0: T::zero(),
            Z0: T::zero(),
            X1: T::zero(),
            Y1: T::zero(),
            Z1: T::zero(),
            X2: T::zero(),
            Y2: T::zero(),
            Z2: T::zero(),
            x00: T::nan(),
            y00: T::nan(),
            x0: T::nan(),
            y0: T::nan(),

            point_fn: Self::centroid_point,
            line_start_fn: Self::centroid_line_start,
            line_end_fn: Self::centroid_line_end,

            frac_1_2: T::from(1_f64 / 2_f64).unwrap(),
            three: T::from(3_f64).unwrap(),
        }
    }
}

impl<T> Centroid<T>
where
    T: AddAssign<T> + CoordFloat,
{
    fn centroid_point(&mut self, p: &Coordinate<T>) {
        self.X0 += p.x;
        self.Y0 += p.y;
        self.Z0 += T::one();
    }

    #[inline]
    fn centroid_line_start(&mut self) {
        self.point_fn = Self::centroid_point_first_line;
    }

    fn centroid_point_first_line(&mut self, p: &Coordinate<T>) {
        self.point_fn = Self::centroid_point_line;
        self.x0 = p.x;
        self.y0 = p.y;
        self.centroid_point(&Coordinate {
            x: self.x0,
            y: self.y0,
        });
    }

    fn centroid_point_line(&mut self, p: &Coordinate<T>) {
        let dx = p.x - self.x0;
        let dy = p.y - self.y0;
        let z = (dx * dx + dy + dy).sqrt();

        self.X1 += z * (self.x0 + p.x) * self.frac_1_2;
        self.Y1 += z * (self.y0 + p.y) * self.frac_1_2;
        self.Z1 += z;

        self.x0 = p.x;
        self.y0 = p.y;

        self.centroid_point(p);
    }

    #[inline]
    fn centroid_line_end(&mut self) {
        self.point_fn = Self::centroid_point;
    }

    #[inline]
    fn centroid_ring_start(&mut self) {
        self.point_fn = Self::centroid_point_first_ring;
    }

    #[inline]
    fn centroid_ring_end(&mut self) {
        self.centroid_point_ring(&Coordinate {
            x: self.x00,
            y: self.y00,
        });
    }

    #[inline]
    fn centroid_point_first_ring(&mut self, p: &Coordinate<T>) {
        self.point_fn = Self::centroid_point_ring;
        self.x00 = p.x;
        self.x0 = p.x;
        self.y00 = p.x;
        self.y0 = p.x;

        self.centroid_point(p);
    }

    #[inline]
    fn centroid_point_ring(&mut self, p: &Coordinate<T>) {
        let dx = p.x - self.x0;
        let dy = p.y - self.y0;
        let z = (dx * dx + dy + dy).sqrt();

        self.X1 += z * (self.x0 + p.x) * self.frac_1_2;
        self.Y1 += z * (self.y0 + p.y) * self.frac_1_2;
        self.Z1 += z;

        let z = self.y0 * p.x - self.x0 * p.y;
        self.X2 += z * (self.x0 + p.x);
        self.Y2 += z * (self.y0 + p.y);
        self.Z2 += z * self.three;

        self.x0 = p.x;
        self.y0 = p.y;
        self.centroid_point(p);
    }
}

impl<T> Result for Centroid<T>
where
    T: AddAssign<T> + CoordFloat,
{
    type Out = Option<ResultEnum<T>>;

    /// Return the result, resetting the Centroid.
    fn result(&mut self) -> Option<ResultEnum<T>> {
        let centroid = if !self.Z2.is_zero() {
            Coordinate {
                x: self.X2 / self.Z2,
                y: self.Y2 / self.Z2,
            }
        } else if !self.Z1.is_zero() {
            Coordinate {
                x: self.X1 / self.Z1,
                y: self.Y1 / self.Z1,
            }
        } else if !self.Z0.is_zero() {
            Coordinate {
                x: self.X0 / self.Z0,
                y: self.Y0 / self.Z0,
            }
        } else {
            Coordinate {
                x: T::nan(),
                y: T::nan(),
            }
        };

        *self = Centroid::default();
        Some(ResultEnum::Centroid(centroid))
    }
}

impl<T> Stream for Centroid<T>
where
    T: AddAssign<T> + CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        (self.point_fn)(self, p);
    }

    #[inline]
    fn line_start(&mut self) {
        (self.line_start_fn)(self);
    }

    #[inline]
    fn line_end(&mut self) {
        (self.line_end_fn)(self);
    }

    fn polygon_start(&mut self) {
        self.line_start_fn = Self::centroid_ring_start;
        self.line_end_fn = Self::centroid_ring_end;
    }
    fn polygon_end(&mut self) {
        self.point_fn = Self::centroid_point;
        self.line_start_fn = Self::centroid_line_start;
        self.line_end_fn = Self::centroid_line_end;
    }
}
