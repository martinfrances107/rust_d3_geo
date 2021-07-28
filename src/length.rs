use std::fmt::Display;
use std::ops::AddAssign;

use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::stream::Streamable;

use super::stream::Stream;

use derivative::Derivative;
use geo::{CoordFloat, Coordinate};

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct LengthStream<T: CoordFloat + FloatConst> {
    // sphere_fn: fn(&mut Self, f64, f64),
    #[derivative(Debug = "ignore")]
    point_fn: fn(&mut Self, &Coordinate<T>),
    #[derivative(Debug = "ignore")]
    line_start_fn: fn(&mut Self),
    #[derivative(Debug = "ignore")]
    line_end_fn: fn(&mut Self),
    length_sum: T,
    lambda0: T,
    sin_phi0: T,
    cos_phi0: T,
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Default
    for LengthStream<T>
{
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

// impl<T> Streamable for LengthStream<T>
// where
// T: AddAssign + AsPrimitive<T> + CoordFloat +Display + FloatConst{
//     type SD=Self;
//     type T=T;
//     fn to_stream<SD: Stream>(stream: &mut SD){

//     }
// }
impl<T> LengthStream<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pub fn calc(object: &impl Streamable<T = T>) -> T {
        let mut ls: LengthStream<T> = LengthStream::default();
        object.to_stream(&mut ls);
        return ls.length_sum;
    }

    fn length_point_first(&mut self, p: &Coordinate<T>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();
        self.lambda0 = lambda;
        self.sin_phi0 = phi.sin();
        self.cos_phi0 = phi.cos();
        self.point_fn = Self::length_point;
    }

    fn length_point(&mut self, p: &Coordinate<T>) {
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
    fn point_noop(&mut self, _p: &Coordinate<T>) {}
    fn line_end_noop(&mut self) {}
}

impl<T> Stream for LengthStream<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SC = Coordinate<T>;
    // fn get_dst(&self) -> Self {
    //     self.clone()
    // }
    fn sphere(&mut self) {}
    fn point(&mut self, p: &Coordinate<T>, _z: Option<u8>) {
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
