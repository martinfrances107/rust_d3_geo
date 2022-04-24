use geo::{CoordFloat, Coordinate};
use web_sys::CanvasRenderingContext2d;

use crate::stream::Stream;

use super::PointRadiusTrait;
use super::Result;

#[derive(Clone, Debug, PartialEq)]
enum PointState {
    Init,
    LineStart,
    Next,
}

#[derive(Clone, Debug, PartialEq)]
enum LineState {
    Init,
    PolygonStarted,
}

/// Path Context.
#[derive(Clone, Debug, PartialEq)]
pub struct Context<T>
// where
//     T: CoordFloat,
{
    line: LineState,
    point: PointState,
    radius: T,
    context: Option<CanvasRenderingContext2d>,
}

impl<T> Default for Context<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            line: LineState::Init,
            point: PointState::Init,
            radius: T::from(4.5).unwrap(),
            context: None,
        }
    }
}
impl<T> Context<T>
where
    T: CoordFloat,
{
    /// Contructor.
    #[inline]
    pub fn new(context: CanvasRenderingContext2d) -> Self {
        Self {
            context: Some(context),
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
    type T = T;

    fn point_radius(&mut self, val: Self::T) {
        self.radius = val;
    }
}

impl<'a, T> Result for Context<T>
where
    T: CoordFloat,
{
    type Out = ();
    #[inline]
    fn result(&mut self) {}
}

impl<T> Stream for Context<T>
where
    T: CoordFloat,
{
    type EP = Self;
    type T = T;

    #[inline]
    fn get_endpoint(&mut self) -> &mut Self {
        self
    }

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
        if LineState::PolygonStarted == self.line {
            if let Some(c) = &self.context {
                c.close_path();
            }
        }

        self.point = PointState::Init;
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, _z: Option<u8>) {
        match self.point {
            PointState::LineStart => {
                if let Some(c) = &self.context {
                    c.move_to(p.x.to_f64().unwrap(), p.y.to_f64().unwrap());
                }
                self.point = PointState::Next;
            }
            PointState::Next => {
                if let Some(c) = &self.context {
                    c.line_to(p.x.to_f64().unwrap(), p.y.to_f64().unwrap());
                }
            }
            PointState::Init => {
                if let Some(c) = &self.context {
                    c.move_to(
                        p.x.to_f64().unwrap() + self.radius.to_f64().unwrap(),
                        p.y.to_f64().unwrap(),
                    );
                    c.arc(
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
}
