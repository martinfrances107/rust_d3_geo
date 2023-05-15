use std::vec::Vec;

use geo_types::Coord;
#[cfg(not(any(test)))]
use web_sys::Path2d;

#[cfg(any(test))]
use crate::path_test_context::Path2d;
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

/// Stream endpoint: Output to a HTML canvas element
#[derive(Clone, Debug, PartialEq)]
pub struct Context {
    line: LineState,
    point: PointState,
    radius: f64,
    pub path2d: Option<Path2d>,
}

impl Default for Context {
    #[inline]
    fn default() -> Self {
        Self {
            line: LineState::Init,
            point: PointState::Init,
            radius: 4.5,
            path2d: None,
        }
    }
}

impl Context {
    /// Contructor.
    #[inline]
    #[must_use]
    pub const fn new(path_string: Path2d) -> Self {
        Self {
            path2d: Some(path_string),
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
        self.path2d.as_mut().map_or_else(Vec::new, Path2d::result)
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
            if let Some(c) = &mut self.path2d {
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
    fn point(&mut self, p: &Coord<f64>, _z: Option<u8>) {
        match self.point {
            PointState::LineStart => {
                if let Some(c) = &mut self.path2d {
                    c.move_to(p.x, p.y);
                }
                self.point = PointState::Next;
            }
            PointState::Next => {
                if let Some(c) = &mut self.path2d {
                    c.line_to(p.x, p.y);
                }
            }
            #[allow(clippy::assertions_on_constants)]
            PointState::Init => {
                if let Some(c) = &mut self.path2d {
                    c.move_to(p.x + self.radius, p.y);

                    match c.arc(p.x, p.y, self.radius, 0_f64, std::f64::consts::TAU) {
                        Ok(_) => {}
                        Err(_) => {
                            debug_assert!(true, "Suppressing arc failure");
                        }
                    };
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
