extern crate web_sys;

use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use web_sys::CanvasRenderingContext2d;

// use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;

use super::PathResult;
use super::PathResultEnum;
use super::PointRadiusTrait;

// use super::RenderingContext2d;
#[derive(Clone, Debug)]
pub struct PathContext<T>
where
    T: AsPrimitive<T>,
{
    line: Option<T>,
    point: Option<T>,
    radius: T,
    context: CanvasRenderingContext2d,
}

impl<T> PathContext<T>
where
    T: AsPrimitive<T> + CoordFloat,
{
    #[inline]
    pub fn new(context: CanvasRenderingContext2d) -> Self {
        Self {
            context,
            line: None,
            point: None,
            radius: T::from(4.5).unwrap(),
        }
    }
}

impl<T> PointRadiusTrait for PathContext<T>
where
    T: AsPrimitive<T>,
{
    type PrtT = Option<T>;
    fn point_radius(&mut self, val: Self::PrtT) {
        match val {
            Some(val) => {
                self.radius = val;
            }
            None => {
                panic!("Calling None here is not defined in the JS version.")
            }
        }
    }
}

impl<T> PathResult for PathContext<T>
where
    T: AsPrimitive<T> + CoordFloat,
{
    type Out = Option<PathResultEnum<T>>;
    #[inline]
    fn result(&mut self) -> Option<PathResultEnum<T>> {
        None
    }
}

impl<T> Stream for PathContext<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst + AsPrimitive<T>,
{
    type SC = Coordinate<T>;

    fn sphere(&mut self) {}

    #[inline]
    fn polygon_start(&mut self) {
        self.line = Some(T::zero());
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.line = Some(T::nan());
    }

    #[inline]
    fn line_start(&mut self) {
        self.point = Some(T::zero());
    }

    fn line_end(&mut self) {
        match self.line {
            Some(line) => {
                if line.is_zero() {
                    self.context.close_path();
                }
            }
            None => {}
        }
        self.point = Some(T::nan());
    }

    fn point(&mut self, p: &Coordinate<T>, _z: Option<u8>) {
        match self.point {
            Some(point) => {
                if point == T::zero() {
                    self.context
                        .move_to(p.x.to_f64().unwrap(), p.y.to_f64().unwrap());
                    self.point = Some(T::one());
                } else if point == T::one() {
                    self.context
                        .line_to(p.x.to_f64().unwrap(), p.y.to_f64().unwrap());
                } else {
                    self.context
                        .move_to(p.x.to_f64().unwrap(), p.y.to_f64().unwrap());
                    self.context
                        .arc(
                            p.x.to_f64().unwrap(),
                            p.y.to_f64().unwrap(),
                            self.radius.to_f64().unwrap(),
                            0f64,
                            std::f64::consts::TAU,
                        )
                        .expect("error writing arc to context");
                }
            }
            None => {}
        }
    }
}
