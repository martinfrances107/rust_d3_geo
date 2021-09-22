use std::rc::Rc;

use geo::{CoordFloat, Coordinate};
use web_sys::CanvasRenderingContext2d;

use crate::stream::Stream;

use super::PointRadiusTrait;
use super::Result;
use super::ResultEnum;

#[derive(Clone, Debug)]
enum PointState {
    Init,
    LineStart,
    Next,
}

#[derive(Clone, Debug)]
enum LineState {
    Init,
    PolygonStarted,
}

/// Path Context.
#[derive(Clone, Debug)]
pub struct Context<T>
where
    T: CoordFloat,
{
    line: LineState,
    point: PointState,
    radius: T,
    context: Rc<CanvasRenderingContext2d>,
}

impl<T> Context<T>
where
    T: CoordFloat,
{
    /// Contructor.
    #[inline]
    pub fn new(context: Rc<CanvasRenderingContext2d>) -> Self {
        Self {
            context,
            line: LineState::Init,
            point: PointState::Init,
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
        self.line = LineState::PolygonStarted;
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.line = LineState::Init;
    }

    #[inline]
    fn line_start(&mut self) {
        self.point = PointState::LineStart;
    }

    fn line_end(&mut self) {
        if let LineState::PolygonStarted = self.line {
            self.context.close_path();
        }

        self.point = PointState::Init;
    }

    fn point(&mut self, p: &Coordinate<T>, _z: Option<u8>) {
        dbg!(&self.point);
        match self.point {
            PointState::LineStart => {
                self.context
                    .move_to(p.x.to_f64().unwrap(), p.y.to_f64().unwrap());
                self.point = PointState::Next;
            }
            PointState::Next => {
                self.context
                    .line_to(p.x.to_f64().unwrap(), p.y.to_f64().unwrap());
            }
            PointState::Init => {
                self.context.move_to(
                    p.x.to_f64().unwrap() + self.radius.to_f64().unwrap(),
                    p.y.to_f64().unwrap(),
                );
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
    }
}
