use std::fmt::Debug;
use std::ops::AddAssign;

use geo::CoordFloat;
use geo::Point;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::math::EPSILON;
use crate::math::EPSILON2;
use crate::stream::Stream as StreamTrait;
use crate::stream::Streamable;

/// Centroid Stream.
#[allow(non_snake_case)]
#[derive(Clone)]
/// Stream endpoint: Computes the centroid.
///
/// DISAMBIGUATION: Lots of code in common with path/centroid.rs
/// but this is true of the Javascript.
pub struct Centroid<T>
where
    T: CoordFloat,
{
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
    epsilon: T,
    epsilon2: T,
    point_fn: fn(&mut Self, &Coord<T>),
    line_start_fn: fn(&mut Self),
    line_end_fn: fn(&mut Self),
}

impl<T> Debug for Centroid<T>
where
    T: CoordFloat,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Area<T>")
            .field(&self.W0)
            .field(&self.W1)
            .field(&self.X0)
            .field(&self.Y0)
            .field(&self.Z0)
            .field(&self.X1)
            .field(&self.Y1)
            .field(&self.Z1)
            .field(&self.X2)
            .field(&self.Y2)
            .field(&self.Z2)
            .field(&self.lambda00)
            .field(&self.phi00)
            .field(&self.x0)
            .field(&self.y0)
            .field(&self.z0)
            .finish()
    }
}
/// The use is nan.
///
/// A) In the JS version these varibles are undefined.
/// The intent is to insists that the values are written before being read.
impl<T> Default for Centroid<T>
where
    T: AddAssign + CoordFloat + FloatConst,
{
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
    /// Compute the centroid.
    pub fn calc(&mut self, d_object: &impl Streamable<T = T>) -> Point<T> {
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

        Point::new(y.atan2(x).to_degrees(), (z / m).asin().to_degrees())
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

    #[inline]
    fn centroid_line_end(&mut self) {
        self.point_fn = Self::centroid_point;
    }

    fn centroid_line_point_first(&mut self, p: &Coord<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let (sin_phi, cos_phi) = phi.sin_cos();
        let (lambda_sin, lambda_cos) = lambda.sin_cos();
        self.x0 = cos_phi * lambda_cos;
        self.y0 = cos_phi * lambda_sin;
        self.z0 = sin_phi;
        self.point_fn = Self::centroid_line_point;
        self.centroid_point_cartesian(self.x0, self.y0, self.z0);
    }

    fn centroid_line_point(&mut self, p: &Coord<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let (sin_lambda, cos_lambda) = lambda.sin_cos();
        let (phi_sin, phi_cos) = phi.sin_cos();
        let x = phi_cos * cos_lambda;
        let y = phi_cos * sin_lambda;
        let z = phi_sin;
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
    fn centroid_point(&mut self, p: &Coord<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let (sin_phi, cos_phi) = phi.sin_cos();
        let (sin_lambda, cos_lambda) = lambda.sin_cos();
        self.centroid_point_cartesian(cos_phi * cos_lambda, cos_phi * sin_lambda, sin_phi);
    }

    fn centroid_ring_point_first(&mut self, p: &Coord<T>) {
        self.lambda00 = p.x;
        self.phi00 = p.y;
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let (sin_phi, cos_phi) = phi.sin_cos();
        let (sin_lambda, cos_lambda) = lambda.sin_cos();
        self.x0 = cos_phi * cos_lambda;
        self.y0 = cos_phi * sin_lambda;
        self.z0 = sin_phi;
        self.point_fn = Self::centroid_ring_point;
        self.centroid_point_cartesian(self.x0, self.y0, self.z0);
    }

    fn centroid_ring_point(&mut self, p: &Coord<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let (sin_phi, cos_phi) = phi.sin_cos();
        let (sin_lambda, cos_lambda) = lambda.sin_cos();
        let x = cos_phi * cos_lambda;
        let y = cos_phi * sin_lambda;
        let z = sin_phi;
        let cx = self.y0 * z - self.z0 * y;
        let cy = self.z0 * x - self.x0 * z;
        let cz = self.x0 * y - self.y0 * x;
        let m = (cx * cx + cy * cy + cz * cz).sqrt();
        let w = m.asin(); // line weight = angle

        let v: T = if m == T::zero() { T::zero() } else { -w / m }; // area weight multiplier

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
        self.centroid_ring_point(&Coord {
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
}

impl<T> StreamTrait for Centroid<T>
where
    T: AddAssign + CoordFloat + FloatConst,
{
    type EP = Self;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self {
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
    fn point(&mut self, p: &Coord<Self::T>, _m: Option<u8>) {
        (self.point_fn)(self, p);
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.line_start_fn = Self::centroid_line_start;
        self.line_end_fn = Self::centroid_line_end;
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.line_start_fn = Self::centroid_ring_start;
        self.line_end_fn = Self::centroid_ring_end;
    }
}
