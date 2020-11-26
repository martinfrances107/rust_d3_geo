// use delaunator::Point;

use super::data_object::DataObject;
use super::stream::convert_obj_to_stream::convert_obj_to_stream;
use super::stream::Stream;

pub struct LengthStream {
    // sphere_fn: fn(&mut Self, f64, f64),
    point_fn: fn(&mut Self, f64, f64),
    line_start_fn: fn(&mut Self),
    line_end_fn: fn(&mut Self),
    length_sum: f64,
    lambda0: f64,
    sin_phi0: f64,
    cos_phi0: f64,
}

impl Default for LengthStream {
    fn default() -> Self {
        return Self {
            // sphere_fn: Self::noop,
            point_fn: Self::point_noop,
            line_start_fn: Self::length_line_start,
            line_end_fn: Self::line_end_noop,
            length_sum: 0f64,
            lambda0: 0f64,
            sin_phi0: 0f64,
            cos_phi0: 0f64,
        };
    }
}

impl LengthStream {
    pub fn calc(object: DataObject) -> f64 {
        let mut ls = LengthStream::default();
        convert_obj_to_stream(&object, &mut ls);
        return ls.length_sum;
    }

    fn length_point_first(&mut self, lambda_p: f64, phi_p: f64) {
        let lambda = lambda_p.to_radians();
        let phi = phi_p.to_radians();
        self.lambda0 = lambda;
        self.sin_phi0 = phi.sin();
        self.cos_phi0 = phi.cos();
        self.point_fn = Self::length_point;
    }

    fn length_point(&mut self, lambda_p: f64, phi_p: f64) {
        let lambda = lambda_p.to_radians();
        let phi = phi_p.to_radians();

        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        let delta = (lambda - self.lambda0).abs();
        let cos_delta = (delta).cos();
        let sin_delta = (delta).sin();

        let x = cos_phi * sin_delta;
        let y = self.cos_phi0 * sin_phi - self.sin_phi0 * cos_phi * cos_delta;
        let z = self.sin_phi0 * sin_phi + self.cos_phi0 * cos_phi * cos_delta;

        self.length_sum = self.length_sum + ((x * x + y * y).sqrt()).atan2(z);
        self.lambda0 = lambda;
        self.sin_phi0 = sin_phi;
        self.cos_phi0 = cos_phi;
    }

    fn length_line_end(&mut self) {
        self.point_fn = Self::point_noop;
        self.line_end_fn = Self::line_end_noop;
    }

    fn length_line_start(&mut self) {
        self.point_fn = Self::length_point_first;
        self.line_end_fn = Self::length_line_end;
    }
    fn point_noop(&mut self, _x: f64, _y: f64) {}
    fn line_end_noop(&mut self) {}
}

impl Stream for LengthStream {
    fn point(&mut self, x: f64, y: f64, _z: Option<f64>) {
        (self.point_fn)(self, x, y);
    }

    fn line_start(&mut self) {
        (self.line_start_fn)(self);
    }

    fn line_end(&mut self) {
        (self.line_end_fn)(self);
    }
    fn polygon_start(&mut self) {}

    fn polygon_end(&mut self) {}
}
