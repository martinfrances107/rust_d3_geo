use crate::stream::Stream;
use crate::stream::StreamClone;
use crate::stream::Streamable;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate, Point};
use num_traits::FloatConst;

// TO MUST use a math library
pub const EPSILON: f64 = 1e-6;
pub const EPSILON2: f64 = 1e-12;

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct CentroidStream<T: CoordFloat> {
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
    point_fn: fn(&mut Self, Coordinate<T>),
    line_start_fn: fn(&mut Self),
    line_end_fn: fn(&mut Self),
}

impl<T: CoordFloat + FloatConst + AddAssign + 'static> Default for CentroidStream<T> {
    fn default() -> Self {
        return Self {
            W0: T::zero(),
            W1: T::zero(),
            X0: T::zero(),
            Y0: T::zero(),
            Z0: T::zero(),
            X1: T::zero(),
            Y1: T::zero(),
            Z1: T::zero(),
            X2: T::zero(),
            Y2: T::zero(),
            Z2: T::zero(),
            lambda00: T::zero(),
            phi00: T::zero(),
            x0: T::zero(),
            y0: T::zero(),
            z0: T::zero(),
            point_fn: Self::centroid_point,
            line_start_fn: Self::centroid_line_start,
            line_end_fn: Self::centroid_line_end,
        };
    }
}

impl<T: CoordFloat + FloatConst + AddAssign + 'static> CentroidStream<T> {
    fn centroid_point_cartesian(&mut self, x: T, y: T, z: T) {
        self.W0 += T::one();
        self.X0 += (x - self.X0) / self.W0;
        self.Y0 += (y - self.Y0) / self.W0;
        self.Z0 += (z - self.Z0) / self.W0;
    }

    fn centroid_line_end(&mut self) {
        self.point_fn = Self::centroid_point;
    }

    fn centroid_line_point_first(&mut self, p: Coordinate<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let cos_phi = phi.cos();
        self.x0 = cos_phi * lambda.cos();
        self.y0 = cos_phi * lambda.sin();
        self.z0 = phi.sin();
        self.point_fn = Self::centroid_line_point;
        self.centroid_point_cartesian(self.x0, self.y0, self.z0);
    }

    fn centroid_line_point(&mut self, p: Coordinate<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let cos_phi = phi.cos();
        let x = cos_phi * lambda.cos();
        let y = cos_phi * lambda.sin();
        let z = phi.sin();
        let w0 = self.y0 * z - self.z0 * y;
        let w1 = self.z0 * x - self.x0 * z;
        let w2 = self.x0 * y - self.y0 * x;
        // let  w = atan2(sqrt((w = y0 * z - z0 * y) * w + (w = z0 * x - x0 * z) * w + (w = x0 * y - y0 * x) * w), x0 * x + y0 * y + z0 * z);
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
    fn centroid_point(&mut self, p: Coordinate<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let cos_phi = phi.cos();
        self.centroid_point_cartesian(cos_phi * lambda.cos(), cos_phi * lambda.sin(), phi.sin());
    }

    fn centroid_ring_point_first(&mut self, p: Coordinate<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        let cos_phi = phi.cos();
        self.x0 = cos_phi * lambda.cos();
        self.y0 = cos_phi * lambda.sin();
        self.z0 = phi.sin();
        self.point_fn = Self::centroid_ring_point;
        self.centroid_point_cartesian(self.x0, self.y0, self.z0);
    }

    fn centroid_ring_point(&mut self, p: Coordinate<T>) {
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
        let w = m.asin(); // line weight = angle
        let v;
        if m != T::zero() {
            v = -w / m;
        } else {
            v = T::zero();
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

    pub fn centroid_ring_end(&mut self) {
        self.centroid_point(Coordinate {
            x: self.lambda00,
            y: self.phi00,
        });
        self.point_fn = Self::centroid_point;
    }

    #[inline]
    fn centroid_ring_start(&mut self) {
        self.point_fn = Self::centroid_ring_point_first;
    }

    pub fn centroid(&self, _d_object: &impl Streamable<SC = Coordinate<T>>) -> Point<T> {
        // d_object.to_stream(self as &dyn Stream<C = Coordinate<T>>);

        let mut x = self.X2;
        let mut y = self.Y2;
        let mut z = self.Z2;
        let mut m = (x * x + y * y + z * z).sqrt();
        // If the area-weighted ccentroid is undefined, fall back to length-weighted ccentroid.
        if m < T::from(EPSILON2).unwrap() {
            x = self.X1;
            y = self.Y1;
            z = self.Z1;
            // If the feature has zero length, fall back to arithmetic mean of point vectors.
            if self.W1 < T::from(EPSILON).unwrap() {
                x = self.X0;
                y = self.Y0;
                z = self.Z0;
            }
            m = (x * x + y * y + z * z).sqrt();

            // If the feature still has an undefined centroid, then return.
            if m < T::from(EPSILON2).unwrap() {
                return Point::new(T::nan(), T::nan());
            }
        }

        return Point::new(y.atan2(x).to_degrees(), (z / m).asin().to_degrees());
    }
}
impl<T: CoordFloat + FloatConst + AddAssign + 'static> StreamClone for CentroidStream<T> {
    // type C = Coordinate<T>;
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    fn box_clone(&self) -> Self::RetType {
        Box::new(self.clone())
    }
}
impl<T: CoordFloat + FloatConst + AddAssign + 'static> Stream for CentroidStream<T> {
    type C = Coordinate<T>;
    //
    #[inline]
    fn line_end(&mut self) {
        (self.line_end_fn)(self);
    }

    #[inline]
    fn line_start(&mut self) {
        (self.line_start_fn)(self);
    }

    #[inline]
    fn point(&mut self, p: Self::C, _m: Option<u8>) {
        (self.point_fn)(self, p);
    }

    fn polygon_start(&mut self) {
        self.line_start_fn = Self::centroid_ring_start;
        self.line_end_fn = Self::centroid_ring_end;
    }

    fn polygon_end(&mut self) {
        self.line_start_fn = Self::centroid_line_start;
        self.line_end_fn = Self::centroid_line_end;
    }
}
