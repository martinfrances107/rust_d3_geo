use std::fmt::Display;

use crate::stream::Stream;
use geo::CoordFloat;
use num_traits::{Float, FloatConst};

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

pub struct PathString<T> {
    circle: Option<String>,
    line: Option<f64>,
    point: Option<f64>,
    radius: T,
    string: Vec<String>,
}

impl<T> Default for PathString<T>
where
    T: Float,
{
    #[inline]
    fn default() -> Self {
        Self {
            circle: Some(circle(4.5f64)),
            line: None,
            point: None,
            radius: T::from(4.5).unwrap(),
            string: Vec::new(),
        }
    }
}

impl<T> PathString<T>
where
    T: Float,
{
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    fn point_radians(&mut self, d: Option<T>) {
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
    #[inline]
    fn result(&mut self) -> Option<String> {
        if self.string.is_empty() {
            let result = self.string.join(",");
            self.string = Vec::new();
            return Some(result);
        } else {
            return None;
        }
    }
}

impl<T> Stream<T> for PathString<T>
where
    T: CoordFloat + FloatConst + std::fmt::Display,
{
    #[inline]
    fn polygon_start(&mut self) {
        self.line = Some(0f64);
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.line = Some(f64::nan());
    }

    #[inline]
    fn line_start(&mut self) {
        self.point = Some(0f64);
    }

    fn line_end(&mut self) {
        match self.line {
            Some(line) => {
                if line == 0f64 {
                    self.string.push(String::from("Z"));
                }
            }
            None => {}
        }

        self.point = Some(1f64);
    }

    #[inline]
    fn point(&mut self, x: T, y: T, _z: Option<u8>) {
        match self.point {
            Some(0f64) => {
                self.string.push(format!("M{},{},", x, y));
                self.point = Some(1f64);
            }
            Some(1f64) => {
                self.string.push(format!("L{},{},", x, y));
            }
            _ => {
                if self.circle.is_none() {
                    self.circle = Some(circle(self.radius));
                }
                self.string.push(format!("M{},{}", x, y));
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
