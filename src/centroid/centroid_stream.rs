use crate::data_object::DataObject;
use crate::stream::convert_obj_to_stream::convert_obj_to_stream;
use crate::stream::Stream;

#[allow(non_snake_case)]
use delaunator::Point;

// TO MUST use a math library
pub const EPSILON: f64 = 1e-6;
pub const EPSILON2: f64 = 1e-12;

#[derive(Clone, Copy, Debug)]
enum PointFn {
    CentroidPoint,
    CentroidLinePoint,
    CentroidLinePointFirst,
    CentroidRingPoint,
    CentroidRingPointFirst,
}

#[derive(Clone, Copy, Debug)]
#[allow(non_snake_case)]
pub struct CentroidStream {
    W0: f64,
    W1: f64,
    X0: f64,
    Y0: f64,
    Z0: f64,
    X1: f64,
    Y1: f64,
    Z1: f64,
    X2: f64,
    Y2: f64,
    Z2: f64,
    lambda00: f64,
    phi00: f64, // first point
    x0: f64,
    y0: f64,
    z0: f64, // previous point
    point_fn: PointFn,
    use_ring_start: bool,
    use_ring_end: bool,
}

impl Default for CentroidStream {
    fn default() -> Self {
        return Self {
            W0: 0f64,
            W1: 0f64,
            X0: 0f64,
            Y0: 0f64,
            Z0: 0f64,
            X1: 0f64,
            Y1: 0f64,
            Z1: 0f64,
            X2: 0f64,
            Y2: 0f64,
            Z2: 0f64,
            lambda00: 0f64,
            phi00: 0f64,
            x0: 0f64,
            y0: 0f64,
            z0: 0f64,
            point_fn: PointFn::CentroidPoint,
            use_ring_start: false,
            use_ring_end: false,
        };
    }
}

impl CentroidStream {
    fn centroid_point_cartesian(&mut self, x: f64, y: f64, z: f64) {
        self.W0 += 1f64;
        self.X0 = (x - self.X0) / self.W0;
        self.Y0 = (y - self.Y0) / self.W0;
        self.Z0 = (z - self.Z0) / self.W0;
    }

    fn centroid_line_end(&mut self) {
        self.point_fn = PointFn::CentroidPoint;
    }

    fn centroid_line_point_first(&mut self, lambda_in: f64, phi_in: f64) {
        let lambda = lambda_in.to_radians();
        let phi = phi_in.to_radians();
        let cos_phi = phi.cos();
        self.x0 = cos_phi * lambda.cos();
        self.y0 = cos_phi * lambda.sin();
        self.z0 = phi.sin();
        self.point_fn = PointFn::CentroidLinePoint;
        self.centroid_point_cartesian(self.x0, self.y0, self.z0);
    }

    fn centroid_line_point(&mut self, lambda_in: f64, phi_in: f64) {
        let lambda = lambda_in.to_radians();
        let phi = phi_in.to_radians();
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
        self.x0 = x;
        self.X1 += w * (self.x0 + x);
        self.y0 = y;
        self.Y1 += w * (self.y0 + y);
        self.z0 = z;
        self.Z1 += w * (self.z0 + z);
        self.centroid_point_cartesian(self.x0, self.y0, self.z0);
    }

    fn centroid_line_start(&mut self) {
        self.point_fn = PointFn::CentroidLinePointFirst;
    }

    /// Arithmetic mean of Cartesian vectors.
    fn centroid_point(&mut self, lambda_in: f64, phi_in: f64) {
        let lambda = lambda_in.to_radians();
        let phi = phi_in.to_radians();
        let cos_phi = phi.cos();
        self.centroid_point_cartesian(cos_phi * lambda.cos(), cos_phi * lambda.sin(), phi.sin());
    }

    fn centroid_ring_point_first(&mut self, lambda_in: f64, phi_in: f64) {
        let lambda = lambda_in.to_radians();
        let phi = phi_in.to_radians();
        let cos_phi = phi.cos();
        self.x0 = cos_phi * lambda.cos();
        self.y0 = cos_phi * lambda.sin();
        self.z0 = phi.sin();
        self.point_fn = PointFn::CentroidRingPoint;
        self.centroid_point_cartesian(self.x0, self.y0, self.z0);
    }

    fn centroid_ring_point(&mut self, lambda_in: f64, phi_in: f64) {
        let lambda = lambda_in.to_radians();
        let phi = phi_in.to_radians();
        let cos_phi = phi.cos();
        let x = cos_phi * lambda.cos();
        let y = cos_phi * lambda.sin();
        let z = phi.sin();
        let cx = self.y0 * z - self.z0 * y;
        let cy = self.z0 * x - self.x0 * z;
        let cz = self.x0 * y - self.y0 * x;
        let m = (cx * cx + cy * cy + cz * cz).sqrt();
        let w = m.asin(); // line weight = angle
        let v: f64;
        if m != 0f64 {
            v = -w / m;
        } else {
            v = 0f64;
        } // area weight multiplier

        self.X2 += v * cx;
        self.Y2 += v * cy;
        self.Z2 += v * cz;
        self.W1 += w;
        self.x0 = x;
        self.X1 += w * (self.x0 + x);
        self.y0 = x;
        self.Y1 += w * (self.y0 + y);
        self.z0 = z;
        self.Z1 += w * (self.z0 + z);
        self.centroid_point_cartesian(self.x0, self.y0, self.z0);
    }

    fn centroid_ring_end(&mut self) {
        self.centroid_point(self.lambda00, self.phi00);
        self.point_fn = PointFn::CentroidPoint
    }

    fn centroid_ring_start(&mut self) {
        self.point_fn = PointFn::CentroidPoint;
    }

    fn polygon() {}

    pub fn centroid(&mut self, d_object: DataObject) -> Point {
        convert_obj_to_stream(&d_object, self);
        println!("self {:?}", self);
        let mut x = self.X2;
        let mut y = self.Y2;
        let mut z = self.Z2;
        let mut m = x * x + y * y + z * z;

        // If the area-weighted ccentroid is undefined, fall back to length-weighted centroid.
        if m < EPSILON2 {
            x = self.X1;
            y = self.Y1;
            z = self.Z1;
            // If the feature has zero length, fall back to arithmetic mean of point vectors.
            if self.W1 < EPSILON {
                x = self.X0;
                y = self.Y0;
                z = self.Z0;
            }
            m = x * x + y * y + z * z;
            println!("self {:?}", m);
            // If the feature still has an undefined centroid, then return.
            if m < EPSILON2 {
                return Point {
                    x: f64::NAN,
                    y: f64::NAN,
                };
            }
        }

        return Point {
            x: y.atan2(x).to_degrees(),
            y: (z / m.sqrt()).asin().to_degrees(),
        };
    }
}

impl Stream for CentroidStream {
    fn line_end(&mut self) {
        if self.use_ring_end {
            self.centroid_ring_end();
        } else {
            self.centroid_line_end();
        }
    }

    fn line_start(&mut self) {
        if self.use_ring_start {
            self.centroid_ring_start();
        } else {
            self.centroid_line_start();
        }
    }

    fn point(&mut self, x: f64, y: f64, _z: Option<f64>) {
        match self.point_fn {
            PointFn::CentroidPoint => {
                self.centroid_point(x, y);
            }
            PointFn::CentroidRingPoint => {
                self.centroid_ring_point(x, y);
            }
            PointFn::CentroidRingPointFirst => {
                self.centroid_ring_point_first(x, y);
            }
            PointFn::CentroidLinePoint => {
                self.centroid_line_point(x, y);
            }
            PointFn::CentroidLinePointFirst => {
                self.centroid_line_point_first(x, y);
            }
        }
        // if self.use_point_first {
        //     self.centroid_point_first(x, y);
        // } else {
        //     self.centroid_point(x, y);
        // }
    }

    fn polygon_start(&mut self) {
        self.use_ring_start = true;
        self.use_ring_end = true;
    }

    fn polygon_end(&mut self) {
        self.use_ring_start = false;
        self.use_ring_end = false;
    }
}
