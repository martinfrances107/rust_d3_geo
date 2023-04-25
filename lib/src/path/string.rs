use std::fmt::Display;
use std::string::String as S;

use geo::CoordFloat;
use geo_types::Coord;

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
    T: CoordFloat,
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

    // This is a hot path.
    //
    // Looking at the flamegraph generated while looking at
    // profile_target :-
    //
    // I can get about a 10% speedup by simplifying the code here.
    // The degnerate polygon test fails without the NAN mapping to zero here.
    // RUST lack %g -- so this hand rolled messyness.
    //
    // Previously to emulate %.6g I used :-
    //
    // let x_rounded = (x * 1_000_000_f64).round() / 1_000_000_f64;
    //
    // After testing trim_end_macthes() is faster.
    #[inline]
    fn point(&mut self, p: &Coord<T>, _m: Option<u8>) {
        let x = format!("{:.6}", p.x);
        let x = x.trim_end_matches('0').trim_end_matches('.');
        let y = format!("{:.6}", p.y);
        let y = y.trim_end_matches('0').trim_end_matches('.');

        match self.point {
            PointState::AtLineStart => {
                self.string.push(format!("M{x},{y}"));
                self.point = PointState::LineInProgress;
            }
            PointState::LineInProgress => {
                self.string.push(format!("L{x},{y}"));
            }
            PointState::RenderingPoints => {
                self.string.push(format!("M{x},{y}"));
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
