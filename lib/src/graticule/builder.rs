use core::fmt::Debug;
use core::mem::swap;

use geo::CoordFloat;
use geo::LineString;
use geo::Polygon;
use geo_types::Coord;

use crate::math::EPSILON;
use crate::range::range;

use super::graticule_x;
use super::graticule_y;
use super::CoordFn;

#[allow(non_snake_case)]
/// A graticule is a network of lines used for plotting, scaling.
pub struct Builder<T>
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

    x: CoordFn<T>,
    y: CoordFn<T>,
    X: CoordFn<T>,
    Y: CoordFn<T>,

    epsilon: T,
    precision: T,
    t90: T,
}

impl<T> Debug for Builder<T>
where
    T: CoordFloat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Builder<T>")
            .field(&self.x0)
            .field(&self.x0)
            .field(&self.X0)
            .field(&self.X1)
            .field(&self.y0)
            .field(&self.y1)
            .field(&self.Y0)
            .field(&self.Y1)
            .field(&self.dx)
            .field(&self.dy)
            .field(&self.dy)
            .field(&self.DX)
            .field(&self.DY)
            .finish()
    }
}

impl<T> Default for Builder<T>
where
    T: 'static + CoordFloat,
{
    #[allow(clippy::similar_names)]
    fn default() -> Self {
        let epsilon = T::from(EPSILON).unwrap();
        let t10 = T::from(10_f64).unwrap();
        let t80 = T::from(80_f64).unwrap();
        let t90 = T::from(90_f64).unwrap();
        let t180 = T::from(180_f64).unwrap();
        let t360 = T::from(360_f64).unwrap();
        let mut out = Self {
            x0: T::nan(),
            x1: T::nan(),
            X0: T::nan(),
            X1: T::nan(),
            y0: T::nan(),
            y1: T::nan(),
            Y0: T::nan(),
            Y1: T::nan(),
            dx: t10,
            dy: t10,
            DX: t90,
            DY: t360,
            x: graticule_x(T::zero(), T::one(), T::from(0.1).unwrap()),
            y: graticule_y(T::zero(), T::one(), T::from(0.1).unwrap()),
            X: graticule_x(T::zero(), T::one(), T::from(0.1).unwrap()),
            Y: graticule_y(T::zero(), T::one(), T::from(0.1).unwrap()),

            precision: T::from(2.5).unwrap(),
            epsilon,
            t90: T::from(90_f64).unwrap(),
        };

        out.extent_major_set([[-t180, -t90 + epsilon], [t180, t90 - epsilon]])
            .extent_minor_set([[-t180, -t80 - epsilon], [t180, t80 + epsilon]]);

        out
    }
}

impl<T> Builder<T>
where
    T: 'static + CoordFloat,
{
    /// Outputs an iterator, which depends on the builder settings.
    pub fn generated_lines(&self) -> impl Iterator<Item = Vec<Coord<T>>> + '_ {
        let range1 =
            range(T::ceil(self.X0 / self.DX) * self.DX, self.X1, self.DX)
                .into_iter()
                .map(&self.X);

        let range2 =
            range(T::ceil(self.Y0 / self.DY) * self.DY, self.Y1, self.DY)
                .into_iter()
                .map(&self.Y);

        let range3 =
            range(T::ceil(self.x0 / self.dx) * self.dx, self.x1, self.dx)
                .into_iter()
                .filter(|x| (*x % self.DX).abs() > self.epsilon)
                .map(&self.x);

        let range4 =
            range(T::ceil(self.y0 / self.dy) * self.dy, self.y1, self.dy)
                .into_iter()
                .filter(|y| (*y % self.DY).abs() > self.epsilon)
                .map(&self.y);

        range1.chain(range2).chain(range3).chain(range4)
    }

    /// Returns an Iterator covering all the generated lines.
    pub fn lines(&self) -> impl Iterator<Item = LineString<T>> + '_ {
        self.generated_lines().map(LineString)
    }

    /// Generates the outline.
    pub fn outline(&self) -> Polygon<T> {
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
    pub const fn extent(&self) -> [[T; 2]; 2] {
        self.extent_minor()
    }

    /// Sets the extent.
    #[inline]
    pub fn extent_set(&mut self, param: [[T; 2]; 2]) -> &mut Self {
        self.extent_major_set(param).extent_minor_set(param)
    }

    /// Returns the range associated with major ticks.
    #[inline]
    pub const fn extent_major(&self) -> [[T; 2]; 2] {
        [[self.X0, self.Y0], [self.X1, self.Y1]]
    }

    /// Sets the major extent.
    pub fn extent_major_set(&mut self, param: [[T; 2]; 2]) -> &mut Self {
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
        self.precision_set(&p)
    }

    /// Returns the range associated with the minor ticks.
    #[inline]
    pub const fn extent_minor(&self) -> [[T; 2]; 2] {
        [[self.x0, self.y0], [self.x1, self.y1]]
    }

    /// Sets the range associated with minor ticks.
    pub fn extent_minor_set(&mut self, param: [[T; 2]; 2]) -> &mut Self {
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
        self.precision_set(&p)
    }

    /// Sets the step for both the major and minor ticks.
    #[inline]
    pub fn step_set(&mut self, step: [T; 2]) -> &mut Self {
        self.step_major_set(step).step_minor_set(step)
    }

    /// Returns the minor step parameters [dx, dy]
    #[inline]
    pub const fn step_major(&self) -> [T; 2] {
        [self.DX, self.DY]
    }

    /// Sets the x and y major step size.
    pub fn step_major_set(&mut self, step: [T; 2]) -> &mut Self {
        self.DX = step[0];
        self.DY = step[1];
        self
    }

    /// Returns the minor step parameters [dx, dy]
    #[inline]
    pub const fn step_minor(&self) -> [T; 2] {
        [self.dx, self.dy]
    }

    /// Sets the x and y minor step size.
    pub fn step_minor_set(&mut self, step: [T; 2]) -> &mut Self {
        self.dx = step[0];
        self.dy = step[1];
        self
    }

    #[inline]
    /// Returns the current precision.
    pub const fn precision(&self) -> T {
        self.precision
    }

    /// Sets the precision for this graticule, in degrees.(Default: 2.5°)
    ///
    /// # Panics
    /// `unwrap()` is used here but a panic will never happen as 90 will always be converted into T.
    pub fn precision_set(&mut self, precision: &T) -> &mut Self {
        self.precision = *precision;
        self.x = graticule_x(self.y0, self.y1, self.t90);
        self.y = graticule_y(self.x0, self.x1, self.precision);
        self.X = graticule_x(self.Y0, self.Y1, self.t90);
        self.Y = graticule_y(self.X0, self.X1, self.precision);
        self
    }
}
