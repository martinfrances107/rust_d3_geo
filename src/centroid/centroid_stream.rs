use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::stream::Streamable;

use std::fmt::Display;
use std::ops::AddAssign;

use derivative::Derivative;
use geo::{CoordFloat, Coordinate, Point};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// TODO MUST use a math library
pub const EPSILON: f64 = 1e-6;
pub const EPSILON2: f64 = 1e-12;

#[allow(non_snake_case)]
#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct CentroidStream<T: CoordFloat + Default> {
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
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Default
    for CentroidStream<T>
{
    fn default() -> Self {
        return Self {
            W0: T::default(),
            W1: T::default(),
            X0: T::default(),
            Y0: T::default(),
            Z0: T::default(),
            X1: T::default(),
            Y1: T::default(),
            Z1: T::default(),
            X2: T::default(),
            Y2: T::default(),
            Z2: T::default(),
            lambda00: T::default(),
            phi00: T::default(),
            x0: T::default(),
            y0: T::default(),
            z0: T::default(),
            point_fn: Self::centroid_point,
            line_start_fn: Self::centroid_line_start,
            line_end_fn: Self::centroid_line_end,
        };
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + FloatConst + Default + Display>
    CentroidStream<T>
{
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
        self.centroid_ring_point(&Coordinate {
            x: self.lambda00,
            y: self.phi00,
        });
        self.point_fn = Self::centroid_point;
    }

    #[inline]
    fn centroid_ring_start(&mut self) {
        self.point_fn = Self::centroid_ring_point_first;
    }

    pub fn centroid(&mut self, d_object: &impl Streamable<T, SC = Coordinate<T>>) -> Point<T> {
        d_object.to_stream(self);

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

impl<T: CoordFloat + FloatConst + AddAssign + AsPrimitive<T> + Default + Display> Stream<T>
    for CentroidStream<T>
{
    type C = Coordinate<T>;

    fn sphere(&mut self) {}

    #[inline]
    fn line_end(&mut self) {
        (self.line_end_fn)(self);
    }

    #[inline]
    fn line_start(&mut self) {
        (self.line_start_fn)(self);
    }

    #[inline]
    fn point(&mut self, p: &Self::C, _m: Option<u8>) {
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

    #[inline]
    fn get_dst(&self) -> StreamDst<T> {
        StreamDst::CS(self.clone())
    }
}
