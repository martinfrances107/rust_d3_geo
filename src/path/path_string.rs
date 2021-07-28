use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

use crate::stream::Stream;

use super::PointRadiusTrait;
use super::{PathResult, PathResultEnum};

#[derive(Clone, Debug)]
enum PointState {
    LineAtStart,
    LineInProgress,
    LineNotInProgress,
}

#[inline]
fn circle<T>(radius: T) -> String
where
    T: CoordFloat + Display,
{
    let two = T::from(2f64).unwrap();
    format!(
        "m0,{radius}a{radius},{radius} 0 1,1 0,{m2r}a{radius},{radius} 0 1,1 0,{p2r}z",
        radius = radius,
        m2r = -two * radius,
        p2r = two * radius
    )
}
#[derive(Debug, Clone)]
pub struct PathString<T>
where
    T: Display,
{
    circle: Option<String>,
    line: bool,
    point: PointState,
    radius: T,
    string: Vec<String>,
}

impl<T> Default for PathString<T>
where
    T: Display + Float,
{
    #[inline]
    fn default() -> Self {
        Self {
            circle: Some(circle(4.5f64)),
            line: false,
            point: PointState::LineNotInProgress,
            radius: T::from(4.5).unwrap(),
            string: Vec::new(),
        }
    }
}

impl<T> PointRadiusTrait for PathString<T>
where
    T: Display + Float,
{
    type PrtT = Option<T>;
    fn point_radius(&mut self, d: Self::PrtT) {
        match d {
            Some(d) => {
                if self.radius != d {
                    self.radius = d;
                    self.circle = None;
                }
            }
            None => {}
        }
    }
}

impl<T> PathResult for PathString<T>
where
    T: CoordFloat + Display + FloatConst,
{
    type Out = Option<PathResultEnum<T>>;
    #[inline]
    fn result(&mut self) -> Option<PathResultEnum<T>> {
        if !self.string.is_empty() {
            let result = self.string.join("");
            self.string = Vec::new();
            return Some(PathResultEnum::String(result));
        } else {
            return None;
        }
    }
}

impl<T> Stream for PathString<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type SC = Coordinate<T>;

    fn sphere(&mut self) {}
    // fn get_dst(&self) -> Self {
    //     self.clone()
    // }

    #[inline]
    fn polygon_start(&mut self) {
        self.line = false;
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.line = true;
    }

    #[inline]
    fn line_start(&mut self) {
        self.point = PointState::LineAtStart;
    }

    fn line_end(&mut self) {
        if !self.line {
            self.string.push(String::from("Z"));
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
                match &self.circle {
                    Some(circle) => {
                        self.string.push(circle.clone());
                    }
                    None => {}
                }
            }
        }
    }
}
