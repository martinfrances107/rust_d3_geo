use std::fmt::Display;
use std::string::String as S;

use geo::{CoordFloat, Coordinate};

use crate::stream::Stream;

use super::PointRadiusTrait;
use super::{Result, ResultEnum};

#[derive(Clone, Debug)]
enum PointState {
    LineAtStart,
    LineInProgress,
    LineNotInProgress,
}

#[derive(Clone, Debug)]
enum LineState {
    Init,
    Started,
}

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

/// PathString.
#[derive(Debug, Clone)]
pub struct String<T>
where
    T: CoordFloat,
{
    circle: Option<S>,
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
            circle: Some(circle(T::from(4.5_f64).unwrap())),
            line: LineState::Init,
            point: PointState::LineNotInProgress,
            radius: T::from(4.5).unwrap(),
            string: Vec::new(),
        }
    }
}

impl<T> PointRadiusTrait for String<T>
where
    T: CoordFloat,
{
    type PrtT = Option<T>;
    fn point_radius(&mut self, d: Self::PrtT) {
        if let Some(d) = d {
            if self.radius != d {
                self.radius = d;
                self.circle = None;
            }
        }
    }
}

impl<T> Result for String<T>
where
    T: CoordFloat,
{
    type Out = Option<ResultEnum<T>>;
    #[inline]
    fn result(&mut self) -> Option<ResultEnum<T>> {
        if self.string.is_empty() {
            None
        } else {
            let result = self.string.join("");
            self.string = Vec::new();
            Some(ResultEnum::String(result))
        }
    }
}

impl<T> Stream for String<T>
where
    T: CoordFloat + Display,
{
    type T = T;

    #[inline]
    fn polygon_start(&mut self) {
        self.line = LineState::Started;
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.line = LineState::Init;
    }

    #[inline]
    fn line_start(&mut self) {
        self.point = PointState::LineAtStart;
    }

    fn line_end(&mut self) {
        if let LineState::Started = self.line {
            self.string.push(S::from("Z"));
        }
        self.point = PointState::LineNotInProgress;
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        match self.point {
            PointState::LineAtStart => {
                self.string.push(format!("M{},{}", p.x, p.y));
                self.point = PointState::LineInProgress;
            }
            PointState::LineInProgress => {
                self.string.push(format!("L{},{}", p.x, p.y));
            }
            PointState::LineNotInProgress => {
                if self.circle.is_none() {
                    self.circle = Some(circle(self.radius));
                }
                self.string.push(format!("M{},{}", p.x, p.y));
                self.string.push(self.circle.as_ref().unwrap().clone());
            }
        }
    }
}
