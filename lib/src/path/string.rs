use std::fmt::Display;
use std::string::String as S;

use geo::CoordFloat;
use geo::Coordinate;

use crate::stream::Stream;

use super::PointRadiusTrait;
use super::Result;

#[derive(Clone, Debug, PartialEq)]
enum PointState {
    AtLineStart,
    LineInProgress,
    RenderingPoints,
}

#[derive(Clone, Debug, PartialEq)]
enum LineState {
    PolygonStopped,
    PolygonStarted,
}

/// Stream endpoint: Output SVG path strings.
#[derive(Debug, Clone, PartialEq)]
pub struct String<T> {
    circle: S,
    line: LineState,
    point: PointState,
    radius: T,
    string: Vec<S>,
}

impl<T> Default for String<T>
where
    T: CoordFloat + Display,
{
    #[inline]
    fn default() -> Self {
        Self {
            circle: circle(T::from(4.5_f64).unwrap()),
            line: LineState::PolygonStopped,
            point: PointState::RenderingPoints,
            radius: T::from(4.5).unwrap(),
            string: Vec::new(),
        }
    }
}

impl<T> PointRadiusTrait for String<T>
where
    T: CoordFloat + Display,
{
    type T = T;

    fn point_radius(&mut self, d: Self::T) {
        if self.radius != d {
            self.radius = d;
            self.circle = circle(d);
        }
    }
}

impl<T> Result for String<T>
where
    T: CoordFloat,
{
    type Out = S;

    #[inline]
    fn result(&mut self) -> Self::Out {
        if self.string.is_empty() {
            S::from("")
        } else {
            let result = self.string.join("");
            self.string = Vec::new();
            result
        }
    }
}

impl<T> Stream for String<T>
where
    T: CoordFloat + Display,
{
    type EP = Self;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self {
        self
    }

    fn line_end(&mut self) {
        if self.line == LineState::PolygonStarted {
            self.string.push(S::from("Z"));
        }
        self.point = PointState::RenderingPoints;
    }

    #[inline]
    fn line_start(&mut self) {
        self.point = PointState::AtLineStart;
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        // 6 digits of precision NAN maps to zero!!!
        let x = p.x.to_f64().unwrap_or(0_f64);
        let x_rounded = (x * 1_000_000_f64).round() / 1_000_000_f64;
        let y = p.y.to_f64().unwrap_or(0_f64);
        let y_rounded = (y * 1_000_000_f64).round() / 1_000_000_f64;
        match self.point {
            PointState::AtLineStart => {
                self.string.push(format!("M{},{}", x_rounded, y_rounded));
                self.point = PointState::LineInProgress;
            }
            PointState::LineInProgress => {
                self.string.push(format!("L{},{}", x_rounded, y_rounded));
            }
            PointState::RenderingPoints => {
                self.string.push(format!("M{},{}", x_rounded, y_rounded));
                self.string.push(self.circle.clone());
            }
        }
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.line = LineState::PolygonStopped;
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.line = LineState::PolygonStarted;
    }
}

/// Private helper functions.
#[inline]
fn circle<T>(radius: T) -> S
where
    T: CoordFloat + Display,
{
    let two = T::from(2_f64).unwrap();
    format!(
        "m0,{radius}a{radius},{radius} 0 1,1 0,{m2r}a{radius},{radius} 0 1,1 0,{p2r}z",
        radius = radius,
        m2r = -two * radius,
        p2r = two * radius
    )
}
