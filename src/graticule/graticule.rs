use std::mem::swap;

use derivative::*;

use geo::CoordFloat;
use geo::LineString;
use geo::Polygon;

use crate::math::EPSILON;

use rust_d3_array::range::range;

use super::graticule_x;
use super::graticule_y;
use super::CoordFn;

#[derive(Derivative)]
#[derivative(Debug)]
#[allow(non_snake_case)]
/// A graticule is a network of lines.
pub struct Graticule<T>
where
    T: CoordFloat,
{
    x0: T,
    x1: T,
    X0: T,
    X1: T,
    y0: T,
    y1: T,
    Y0: T,
    Y1: T,
    dx: T,
    dy: T,
    DX: T,
    DY: T,
    #[derivative(Debug = "ignore")]
    x: CoordFn<T>,
    #[derivative(Debug = "ignore")]
    y: CoordFn<T>,
    #[derivative(Debug = "ignore")]
    X: CoordFn<T>,
    #[derivative(Debug = "ignore")]
    Y: CoordFn<T>,
    precision: T,
    epsilon: T,
}

impl<T> Default for Graticule<T>
where
    T: 'static + CoordFloat,
{
    fn default() -> Self {
        Self {
            x0: T::nan(),
            x1: T::nan(),
            X0: T::nan(),
            X1: T::nan(),
            y0: T::nan(),
            y1: T::nan(),
            Y0: T::nan(),
            Y1: T::nan(),
            dx: T::from(10).unwrap(),
            dy: T::from(10).unwrap(),
            DX: T::from(90_f64).unwrap(),
            DY: T::from(360_f64).unwrap(),
            x: graticule_x(T::zero(), T::one(), T::from(0.1).unwrap()),
            y: graticule_y(T::zero(), T::one(), T::from(0.1).unwrap()),
            X: graticule_x(T::zero(), T::one(), T::from(0.1).unwrap()),
            Y: graticule_y(T::zero(), T::one(), T::from(0.1).unwrap()),
            precision: T::from(2.5).unwrap(),
            epsilon: T::from(EPSILON).unwrap(),
        }
    }
}

impl<T> Graticule<T>
where
    T: 'static + CoordFloat,
{
    fn generated_lines(self) -> Vec<(T)> {
        let mut a = range(T::ceil(self.X0 / self.DX) * self.DX, self.X1, self.DX);
        a.extend(range(
            T::ceil(self.Y0 / self.DY) * self.DY,
            self.Y1,
            self.DY,
        ));
        a.extend(
            range(T::ceil(self.x0 / self.dx) * self.dx, self.x1, self.dx)
                .iter()
                .filter(|x| T::abs(**x % self.DX) > self.epsilon),
        );
        a.extend(
            range(T::ceil(self.y0 / self.dy) * self.dy, self.y1, self.dy)
                .iter()
                .filter(|y| T::abs(**y % self.DY) > self.epsilon),
        );
        a
    }

    // pub fn lines(self) {
    //     self.generated_lines()
    //         .iter()
    //         .map(|coordinates| LineString::from(coordinates))
    //         .collect::<(T, T)>()
    // }

    pub fn outline(self) -> Polygon<T> {
        let mut c = (self.X)(self.X0);
        c.append(&mut (self.Y)(self.Y1).split_off(1));

        let mut xr = (self.X)(self.X1);
        xr.reverse();
        let mut xr = xr.split_off(1);
        let mut yr = (self.Y)(self.Y0);
        c.append(&mut xr);

        yr.reverse();
        let mut yr = yr.split_off(1);
        c.append(&mut yr);

        Polygon::new(LineString::from(c), vec![])
    }

    /// Returns the extent.
    #[inline]
    pub fn get_extent(self) -> [[T; 2]; 2] {
        self.get_extent_minor()
    }

    /// Sets the extent.
    #[inline]
    pub fn extent(self, param: [[T; 2]; 2]) -> Self {
        self.extent_major(param).extent_minor(param)
    }

    /// Returns the range associated with major ticks.
    #[inline]
    pub fn get_extent_major(&self) -> [[T; 2]; 2] {
        [[self.X0, self.Y0], [self.X1, self.Y1]]
    }

    /// Sets the major extent.
    pub fn extent_major(mut self, param: [[T; 2]; 2]) -> Self {
        self.X0 = param[0][0];
        self.Y0 = param[0][1];
        self.X1 = param[1][0];
        self.Y1 = param[1][1];
        if self.X0 > self.X1 {
            swap(&mut self.X0, &mut self.X1);
        }
        if self.Y0 > self.Y1 {
            swap(&mut self.Y0, &mut self.Y1);
        }
        let p = self.precision;
        self.precision(&p)
    }

    /// Returns the range assoicated with the minor ticks.
    #[inline]
    pub fn get_extent_minor(&self) -> [[T; 2]; 2] {
        [[self.x0, self.y0], [self.x1, self.y1]]
    }

    /// Sets the range associated with minor ticks.
    pub fn extent_minor(mut self, param: [[T; 2]; 2]) -> Self {
        self.x0 = param[0][0];
        self.y0 = param[0][1];
        self.x1 = param[1][0];
        self.y1 = param[1][1];
        if self.x0 > self.x1 {
            swap(&mut self.x0, &mut self.x1);
        }

        if self.y0 > self.y1 {
            swap(&mut self.y0, &mut self.y1);
        }

        let p = self.precision;
        self.precision(&p)
    }

    /// Sets the step for both the major and minor ticks.
    #[inline]
    pub fn step(self, step: [T; 2]) -> Self {
        self.step_major(step).step_minor(step)
    }

    /// Returns the minor step parameters [dx, dy]
    #[inline]
    pub fn get_step_major(&self) -> [T; 2] {
        [self.DX, self.DY]
    }

    /// Sets the x and y major step size.
    pub fn step_major(mut self, step: [T; 2]) -> Self {
        self.DX = step[0];
        self.DY = step[1];
        self
    }

    /// Returns the minor step parameters [dx, dy]
    #[inline]
    pub fn get_step_minor(&self) -> [T; 2] {
        [self.dx, self.dy]
    }

    /// Sets the x and y minor step size.
    pub fn step_minor(mut self, step: [T; 2]) -> Self {
        self.dx = step[0];
        self.dy = step[1];
        self
    }

    /// Returns the precision.
    #[inline]
    pub fn get_precision(&self) -> T {
        self.precision
    }

    pub fn precision(mut self, precision: &T) -> Self {
        self.precision = *precision;
        self.x = graticule_x(self.y0, self.y1, T::from(90_f64).unwrap());
        self.y = graticule_y(self.x0, self.x1, self.precision);
        self.X = graticule_x(self.Y0, self.Y1, T::from(90_f64).unwrap());
        self.Y = graticule_y(self.X0, self.X1, self.precision);
        self
    }
}
