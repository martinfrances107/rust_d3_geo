use super::stream::Stream;
use super::stream::StreamClone;
use crate::stream::Streamable;
use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

#[derive(Clone)]
pub struct LengthStream<T: CoordFloat + FloatConst> {
    // sphere_fn: fn(&mut Self, f64, f64),
    point_fn: fn(&mut Self, Coordinate<T>),
    line_start_fn: fn(&mut Self),
    line_end_fn: fn(&mut Self),
    length_sum: T,
    lambda0: T,
    sin_phi0: T,
    cos_phi0: T,
}

impl<T: CoordFloat + FloatConst + 'static> Default for LengthStream<T> {
    fn default() -> Self {
        return Self {
            // sphere_fn: Self::noop,
            point_fn: Self::point_noop,
            line_start_fn: Self::length_line_start,
            line_end_fn: Self::line_end_noop,
            length_sum: T::zero(),
            lambda0: T::zero(),
            sin_phi0: T::zero(),
            cos_phi0: T::zero(),
        };
    }
}

impl<T: CoordFloat + FloatConst + 'static> LengthStream<T> {
    pub fn calc(_object: &impl Streamable<SC = Coordinate<T>>) -> T {
        // let mut ls = Box::new(LengthStream::default());
        // object.to_stream(&mut ls);
        panic!("must resole")
        // return ls.length_sum;
    }

    fn length_point_first(&mut self, p: Coordinate<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        self.lambda0 = lambda;
        self.sin_phi0 = phi.sin();
        self.cos_phi0 = phi.cos();
        self.point_fn = Self::length_point;
    }

    fn length_point(&mut self, p: Coordinate<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();

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
    fn point_noop(&mut self, _p: Coordinate<T>) {}
    fn line_end_noop(&mut self) {}
}

impl<T: CoordFloat + FloatConst + 'static> StreamClone for LengthStream<T> {
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(self.clone())
    }
}

impl<T: CoordFloat + FloatConst + 'static> Stream for LengthStream<T> {
    type C = Coordinate<T>;
    fn point(&mut self, p: Coordinate<T>, _z: Option<u8>) {
        (self.point_fn)(self, p);
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
