extern crate web_sys;

use geo::{CoordFloat, Coordinate};
use web_sys::CanvasRenderingContext2d;

use crate::stream::Stream;
use num_traits::{AsPrimitive, FloatConst};

use super::{PathResult, PathResultEnum};

// use super::RenderingContext2d;
#[derive(Clone, Debug)]
pub struct PathContext<T> {
    line: Option<T>,
    point: Option<f64>,
    radius: T,
    context: CanvasRenderingContext2d,
}

impl<T> PathContext<T>
where
    T: CoordFloat,
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

    // fn point_radians(&mut self, r: T) -> PathContext<T> {
    //     self.radius = r;
    //     self
    // }
}

impl<T> PathResult<T> for PathContext<T>
where
    T: CoordFloat,
{
    #[inline]
    fn result(&mut self) -> Option<PathResultEnum<T>> {
        None
    }
}

impl<T> Stream<T> for PathContext<T>
where
    T: CoordFloat + FloatConst + AsPrimitive<f64>,
{
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
        self.point = Some(0f64);
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
        self.point = Some(std::f64::NAN);
    }

    fn point(&mut self, p: Coordinate<T>, _z: Option<u8>) {
        match self.point {
            Some(point) => {
                if point == 0f64 {
                    self.context.move_to(p.x.as_(), p.y.as_());
                    self.point = Some(1f64);
                } else if point == 1f64 {
                    self.context.line_to(p.x.as_(), p.y.as_());
                } else {
                    self.context.move_to(p.x.as_(), p.y.as_());
                    self.context
                        .arc(
                            p.x.as_(),
                            p.y.as_(),
                            self.radius.as_(),
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
