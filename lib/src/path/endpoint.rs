use core::mem;

use geo_types::Coord;

#[cfg(not(test))]
use web_sys::Path2d;

#[cfg(test)]
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

/// Stream path endpoint: Used when rendering to a HTML Canvas element.
///
/// Wraps a Path2d object, and implements STREAM trait.
#[derive(Clone, Debug, PartialEq)]
pub struct Endpoint {
    line: LineState,
    point: PointState,
    radius: f64,
    path2d: Path2d,
}

impl Default for Endpoint {
    #[inline]
    fn default() -> Self {
        Self {
            line: LineState::Init,
            point: PointState::Init,
            radius: 4.5,
            path2d: Path2d::new().unwrap(),
        }
    }
}

impl Endpoint {
    /// Contructor.
    #[inline]
    #[must_use]
    pub const fn new(path2d: Path2d) -> Self {
        Self {
            path2d,
            line: LineState::Init,
            point: PointState::Init,
            radius: 4.5,
        }
    }
}

impl PointRadiusTrait for Endpoint {
    type T = f64;

    fn point_radius(&mut self, val: Self::T) {
        self.radius = val;
    }
}

/// Return path2d, blanking the stored value.
impl Result for Endpoint {
    type Out = Path2d;
    #[inline]
    fn result(&mut self) -> Self::Out {
        let mut out = Path2d::new().unwrap();
        mem::swap(&mut out, &mut self.path2d);
        out
    }
}

impl Stream for Endpoint {
    type EP = Self;
    type T = f64;

    #[inline]
    fn endpoint(&mut self) -> &mut Self {
        self
    }

    fn line_end(&mut self) {
        if LineState::PolygonStarted == self.line {
            self.path2d.close_path();
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
                self.path2d.move_to(p.x, p.y);
                self.point = PointState::Next;
            }
            PointState::Next => {
                self.path2d.line_to(p.x, p.y);
            }
            #[allow(clippy::assertions_on_constants)]
            PointState::Init => {
                self.path2d.move_to(p.x + self.radius, p.y);

                match self
                    .path2d
                    .arc(p.x, p.y, self.radius, 0_f64, std::f64::consts::TAU)
                {
                    Ok(_) => {}
                    Err(_) => {
                        debug_assert!(true, "Suppressing arc failure");
                    }
                };
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
