extern crate web_sys;

use geo::{CoordFloat, Coordinate};
use std::rc::Rc;

use web_sys::CanvasRenderingContext2d;

use crate::stream::Stream;

use super::PointRadiusTrait;
use super::Result;
use super::ResultEnum;

#[derive(Clone, Debug)]
pub struct Context<T>
where
    T: CoordFloat,
{
    line: Option<T>,
    point: Option<T>,
    radius: T,
    context: Rc<CanvasRenderingContext2d>,
}

impl<T> Context<T>
where
    T: CoordFloat,
{
    #[inline]
    pub fn new(context: Rc<CanvasRenderingContext2d>) -> Self {
        Self {
            context,
            line: None,
            point: None,
            radius: T::from(4.5).unwrap(),
        }
    }
}

impl<T> PointRadiusTrait for Context<T>
where
    T: CoordFloat,
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

impl<T> Result for Context<T>
where
    T: CoordFloat,
{
    type Out = Option<ResultEnum<T>>;
    #[inline]
    fn result(&mut self) -> Option<ResultEnum<T>> {
        None
    }
}

impl<T> Stream for Context<T>
where
    T: CoordFloat,
{
    type T = T;

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
                            0_f64,
                            std::f64::consts::TAU,
                        )
                        .expect("error writing arc to context");
                }
            }
            None => {}
        }
    }
}
