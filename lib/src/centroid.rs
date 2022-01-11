use std::ops::AddAssign;

use derivative::*;
use geo::{CoordFloat, Coordinate, Point};
use num_traits::FloatConst;

use crate::math::asin;
use crate::math::EPSILON;
use crate::math::EPSILON2;
use crate::stream::Stream as StreamTrait;
use crate::stream::Streamable;

/// Centroid Stream.
#[allow(non_snake_case)]
#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone, Copy)]
/// Centroid: Point from a streamable object.
///
/// DISAMBIGUATION: Lots of code in common with path/centroid.rs
/// but this is true of the Javascript.
pub struct Centroid<T: CoordFloat> {
    W0: T,
    W1: T,
    X0: T,
    Y0: T,
    Z0: T,
    X1: T,
    Y1: T,
    Z1: T,
    X2: T,
    Y2: T,
    Z2: T,
    lambda00: T,
    phi00: T, // first point
    x0: T,
    y0: T,
    z0: T, // previous point
    #[derivative(Debug = "ignore")]
    point_fn: fn(&mut Self, &Coordinate<T>),
    #[derivative(Debug = "ignore")]
    line_start_fn: fn(&mut Self),
    #[derivative(Debug = "ignore")]
    line_end_fn: fn(&mut Self),
    epsilon: T,
    epsilon2: T,
}

/// The use is nan.
///
/// A) In the JS version these varibles are undefined.
/// The intent is to insists that the values are written before being read.
impl<T: AddAssign + CoordFloat + FloatConst> Default for Centroid<T> {
    fn default() -> Self {
        Self {
            W0: T::nan(),
            W1: T::nan(),
            X0: T::nan(),
            Y0: T::nan(),
            Z0: T::nan(),
            X1: T::nan(),
            Y1: T::nan(),
            Z1: T::nan(),
            X2: T::nan(),
            Y2: T::nan(),
            Z2: T::nan(),
            lambda00: T::nan(),
            phi00: T::nan(),
            x0: T::nan(),
            y0: T::nan(),
            z0: T::nan(),
            point_fn: Self::centroid_point,
            line_start_fn: Self::centroid_line_start,
            line_end_fn: Self::centroid_line_end,
            epsilon: T::from(EPSILON).unwrap(),
            epsilon2: T::from(EPSILON2).unwrap(),
        }
    }
}

impl<T> Centroid<T>
where
    T: AddAssign + CoordFloat + FloatConst,
{
    /// Arithmetic mean of Cartesian vectors.
    fn centroid_point_cartesian(&mut self, x: T, y: T, z: T) {
        self.W0 += T::one();
        self.X0 += (x - self.X0) / self.W0;
        self.Y0 += (y - self.Y0) / self.W0;
        self.Z0 += (z - self.Z0) / self.W0;
    }

    fn centroid_line_end(&mut self) {
        self.point_fn = Self::centroid_point;
    }

    fn centroid_line_point_first(&mut self, p: &Coordinate<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let cos_phi = phi.cos();
        self.x0 = cos_phi * lambda.cos();
        self.y0 = cos_phi * lambda.sin();
        self.z0 = phi.sin();
        self.point_fn = Self::centroid_line_point;
        self.centroid_point_cartesian(self.x0, self.y0, self.z0);
    }

    fn centroid_line_point(&mut self, p: &Coordinate<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let cos_phi = phi.cos();
        let x = cos_phi * lambda.cos();
        let y = cos_phi * lambda.sin();
        let z = phi.sin();
        let w0 = self.y0 * z - self.z0 * y;
        let w1 = self.z0 * x - self.x0 * z;
        let w2 = self.x0 * y - self.y0 * x;
        let w =
            ((w0 * w0 + w1 * w1 + w2 * w2).sqrt()).atan2(self.x0 * x + self.y0 * y + self.z0 * z);
        self.W1 += w;
        self.X1 += w * (self.x0 + x);
        self.x0 = x;
        self.Y1 += w * (self.y0 + y);
        self.y0 = y;
        self.Z1 += w * (self.z0 + z);
        self.z0 = z;
        self.centroid_point_cartesian(self.x0, self.y0, self.z0);
    }

    #[inline]
    fn centroid_line_start(&mut self) {
        self.point_fn = Self::centroid_line_point_first;
    }

    /// Arithmetic mean of Cartesian vectors.
    fn centroid_point(&mut self, p: &Coordinate<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let cos_phi = phi.cos();
        self.centroid_point_cartesian(cos_phi * lambda.cos(), cos_phi * lambda.sin(), phi.sin());
    }

    fn centroid_ring_point_first(&mut self, p: &Coordinate<T>) {
        self.lambda00 = p.x;
        self.phi00 = p.y;
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let cos_phi = phi.cos();
        self.x0 = cos_phi * lambda.cos();
        self.y0 = cos_phi * lambda.sin();
        self.z0 = phi.sin();
        self.point_fn = Self::centroid_ring_point;
        self.centroid_point_cartesian(self.x0, self.y0, self.z0);
    }

    fn centroid_ring_point(&mut self, p: &Coordinate<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let cos_phi = phi.cos();
        let x = cos_phi * lambda.cos();
        let y = cos_phi * lambda.sin();
        let z = phi.sin();
        let cx = self.y0 * z - self.z0 * y;
        let cy = self.z0 * x - self.x0 * z;
        let cz = self.x0 * y - self.y0 * x;
        let m = (cx * cx + cy * cy + cz * cz).sqrt();
        let w = asin(m); // line weight = angle
        let v;
        if m == T::zero() {
            v = T::zero();
        } else {
            v = -w / m;
        } // area weight multiplier

        self.X2 += v * cx;
        self.Y2 += v * cy;
        self.Z2 += v * cz;
        self.W1 += w;
        self.x0 = x;
        self.X1 += w * (self.x0 + x);
        self.y0 = y;
        self.Y1 += w * (self.y0 + y);
        self.z0 = z;
        self.Z1 += w * (self.z0 + z);
        self.centroid_point_cartesian(self.x0, self.y0, self.z0);
    }

    fn centroid_ring_end(&mut self) {
        self.centroid_ring_point(&Coordinate {
            x: self.lambda00,
            y: self.phi00,
        });
        self.point_fn = Self::centroid_point;
    }

    /// See J. E. Brock, The Inertia Tensor for a Spherical Triangle,
    /// J. Applied Mechanics 42, 239 (1975).
    #[inline]
    fn centroid_ring_start(&mut self) {
        self.point_fn = Self::centroid_ring_point_first;
    }

    /// Compute the centroid.
    pub fn centroid(&mut self, d_object: &impl Streamable<T = T>) -> Point<T> {
        self.W0 = T::zero();
        self.W1 = T::zero();
        self.X0 = T::zero();
        self.Y0 = T::zero();
        self.Z0 = T::zero();
        self.X1 = T::zero();
        self.Y1 = T::zero();
        self.Z1 = T::zero();
        self.X2 = T::zero();
        self.Y2 = T::zero();
        self.Z2 = T::zero();
        d_object.to_stream(self);

        let mut x = self.X2;
        let mut y = self.Y2;
        let mut z = self.Z2;
        let mut m = (x * x + y * y + z * z).sqrt();
        // If the area-weighted ccentroid is undefined, fall back to length-weighted ccentroid.
        if m < self.epsilon2 {
            x = self.X1;
            y = self.Y1;
            z = self.Z1;
            // If the feature has zero length, fall back to arithmetic mean of point vectors.
            if self.W1 < self.epsilon {
                x = self.X0;
                y = self.Y0;
                z = self.Z0;
            }
            m = (x * x + y * y + z * z).sqrt();

            // If the feature still has an undefined centroid, then return.
            if m < self.epsilon2 {
                return Point::new(T::nan(), T::nan());
            }
        }

        Point::new(y.atan2(x).to_degrees(), asin(z / m).to_degrees())
    }
}

impl<T> StreamTrait for Centroid<T>
where
    T: AddAssign + CoordFloat + FloatConst,
{
    type T = T;
    type EP = Self;

    fn get_endpoint(self) -> Self {
        self
    }

    #[inline]
    fn line_end(&mut self) {
        (self.line_end_fn)(self);
    }

    #[inline]
    fn line_start(&mut self) {
        (self.line_start_fn)(self);
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<Self::T>, _m: Option<u8>) {
        (self.point_fn)(self, p);
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.line_start_fn = Self::centroid_ring_start;
        self.line_end_fn = Self::centroid_ring_end;
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.line_start_fn = Self::centroid_line_start;
        self.line_end_fn = Self::centroid_line_end;
    }
}
