use std::f64::consts::PI;

use geo::CoordFloat;
use geo::Coordinate;

use crate::stream::Stream;
use crate::stream::Streamable;

/// GeoArea distinct from PathArea.
#[derive(Debug, Clone)]
pub struct Area<T>
where
    T: CoordFloat,
{
    lambda00: T,
    phi00: T,
    lambda0: T,
    cos_phi0: T,
    sin_phi0: T,
    tau: T,
    two: T,
    quarter_pi: T,
    area_ring_sum: T,
    area_sum: T,

    point_fn: PointFn,
    line_end_fn: LineEndFn,
    line_start_fn: LineStartFn,
}

#[derive(Clone, Debug)]
enum PointFn {
    Noop,
    AreaFirst,
    Area,
}
#[derive(Clone, Debug)]
enum LineStartFn {
    Noop,
    AreaRingStart,
}

#[derive(Clone, Debug)]
enum LineEndFn {
    Noop,
    AreaRingEnd,
}

impl<T> Default for Area<T>
where
    T: CoordFloat,
{
    fn default() -> Self {
        Self {
            lambda00: T::nan(),
            phi00: T::nan(),
            lambda0: T::nan(),
            cos_phi0: T::nan(),
            sin_phi0: T::nan(),
            tau: T::from(2_f64 * PI).unwrap(),
            two: T::from(2_f64).unwrap(),
            quarter_pi: T::from(PI / 4_f64).unwrap(),
            area_ring_sum: T::nan(),
            area_sum: T::zero(),

            point_fn: PointFn::Noop,
            line_end_fn: LineEndFn::Noop,
            line_start_fn: LineStartFn::Noop,
        }
    }
}

impl<T> Area<T>
where
    T: CoordFloat,
{
    /// Calculate the objects assocated area.
    pub fn calc(object: &impl Streamable<T = T>) -> T {
        let mut a = Area::default();
        object.to_stream(&mut a);
        a.area_sum * a.two
    }

    fn area_point_first(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        // areaStream.point = areaPoint;
        self.point_fn = PointFn::Area;
        // lambda00 = lambda, phi00 = phi;
        self.lambda00 = p.x;
        self.phi00 = p.y;
        // lambda *= radians, phi *= radians;
        self.lambda0 = p.x.to_radians();
        let phi = p.y.to_radians();
        let phi = phi / self.two + self.quarter_pi;
        // lambda0 = lambda, cosPhi0 = cos(phi = phi / 2 + quarterPi), sinPhi0 = sin(phi);
        self.cos_phi0 = phi.cos();
        self.sin_phi0 = phi.sin();
    }

    fn area_point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        // lambda *= radians, phi *= radians;
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();

        let phi = phi / self.two + self.quarter_pi; // half the angular distance from south pole

        // Spherical excess E for a spherical triangle with vertices: south pole,
        // previous point, current point.  Uses a formula derived from Cagnoli’s
        // theorem.  See Todhunter, Spherical Trig. (1871), Sec. 103, Eq. (2).
        let d_lambda = lambda - self.lambda0;
        // let sdLambda = dLambda >= 0 ? 1 : -1,
        let sd_lambda = if d_lambda >= T::zero() {
            T::one()
        } else {
            -T::one()
        };
        let ad_lambda = sd_lambda * d_lambda;
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();
        let k = self.sin_phi0 * sin_phi;
        let u = self.cos_phi0 * cos_phi + k * ad_lambda.cos();
        let v = k * sd_lambda * ad_lambda.sin();
        self.area_ring_sum = self.area_ring_sum.add(v.atan2(u));

        // Advance the previous points.
        self.lambda0 = lambda;
        self.cos_phi0 = cos_phi;
        self.sin_phi0 = sin_phi;
    }

    #[inline]
    fn area_ring_start(&mut self) {
        self.point_fn = PointFn::AreaFirst;
    }

    #[inline]
    fn area_ring_end(&mut self) {
        self.area_point(
            &Coordinate {
                x: self.lambda00,
                y: self.phi00,
            },
            None,
        );
    }
}

impl<T> Stream for Area<T>
where
    T: CoordFloat,
{
    type T = T;

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self.point_fn {
            PointFn::Noop => {}
            PointFn::AreaFirst => {
                self.area_point_first(p, m);
            }
            PointFn::Area => self.area_point(p, m),
        }
    }

    #[inline]
    fn line_start(&mut self) {
        match self.line_start_fn {
            LineStartFn::Noop => {}
            LineStartFn::AreaRingStart => self.area_ring_start(),
        }
    }

    #[inline]
    fn line_end(&mut self) {
        match self.line_end_fn {
            LineEndFn::Noop => {}
            LineEndFn::AreaRingEnd => self.area_ring_end(),
        }
    }
    fn polygon_start(&mut self) {
        self.area_ring_sum = T::zero();
        self.line_start_fn = LineStartFn::AreaRingStart;
        self.line_end_fn = LineEndFn::AreaRingEnd;
    }

    fn polygon_end(&mut self) {
        let area_ring = self.area_ring_sum;
        self.area_sum = if area_ring < T::zero() {
            self.tau + area_ring
        } else {
            area_ring
        };
    }

    #[inline]
    fn sphere(&mut self) {
        self.area_sum = self.area_sum + self.tau;
    }
}
