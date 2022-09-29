use geo::Coordinate;

#[cfg(not(any(test)))]
use web_sys::CanvasRenderingContext2d;

#[cfg(any(test))]
use crate::path_test_context::CanvasRenderingContext2d;

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
pub struct Context {
    line: LineState,
    point: PointState,
    radius: f64,
    context: Option<CanvasRenderingContext2d>,
}

impl Default for Context {
    #[inline]
    fn default() -> Self {
        Self {
            line: LineState::Init,
            point: PointState::Init,
            radius: 4.5,
            context: None,
        }
    }
}

impl Context {
    /// Contructor.
    #[inline]
    pub fn new(context: CanvasRenderingContext2d) -> Self {
        Self {
            context: Some(context),
            line: LineState::Init,
            point: PointState::Init,
            radius: 4.5,
        }
    }
}

impl PointRadiusTrait for Context {
    type T = f64;

    fn point_radius(&mut self, val: Self::T) {
        self.radius = val;
    }
}

/// Reach into the mock and return a record of all activity.
#[cfg(test)]
impl Result for Context {
    type Out = Vec<String>;
    #[inline]
    fn result(&mut self) -> Self::Out {
        match &mut self.context {
            Some(context) => context.result(),
            None => vec![],
        }
    }
}

/// Stub, In production code the API calls change the canvas directly.
#[cfg(not(test))]
impl Result for Context {
    type Out = Vec<String>;
    #[inline]
    fn result(&mut self) -> Self::Out {
        vec![]
    }
}

impl Stream for Context {
    type EP = Self;
    type T = f64;

    #[inline]
    fn endpoint(&mut self) -> &mut Self {
        self
    }

    fn line_end(&mut self) {
        if LineState::PolygonStarted == self.line {
            if let Some(c) = &mut self.context {
                c.close_path();
            }
        }

        self.point = PointState::Init;
    }

    #[inline]
    fn line_start(&mut self) {
        self.point = PointState::LineStart;
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<f64>, _z: Option<u8>) {
        match self.point {
            PointState::LineStart => {
                if let Some(c) = &mut self.context {
                    c.move_to(p.x, p.y);
                }
                self.point = PointState::Next;
            }
            PointState::Next => {
                if let Some(c) = &mut self.context {
                    c.line_to(p.x, p.y);
                }
            }
            PointState::Init => {
                if let Some(c) = &mut self.context {
                    c.move_to(p.x + self.radius, p.y);
                    c.arc(p.x, p.y, self.radius, 0_f64, std::f64::consts::TAU);
                }
            }
        }
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.line = LineState::Init;
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.line = LineState::PolygonStarted;
    }

    fn sphere(&mut self) {}
}
