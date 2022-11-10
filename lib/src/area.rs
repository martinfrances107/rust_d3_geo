use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::stream::Stream;
use crate::stream::Streamable;

/// Stream endpoint: Compute the area.
///
/// DISAMBIGUATION: Lots of code in common with path/area.rs
/// but this is true of the Javascript.
#[derive(Debug, Clone, PartialEq)]
pub struct Area<T> {
    lambda00: T,
    phi00: T,
    lambda0: T,
    cos_phi0: T,
    sin_phi0: T,
    two: T,
    area_ring_sum: T,
    area_sum: T,

    point_fn: PointFn,
    line_end_fn: LineEndFn,
    line_start_fn: LineStartFn,
}

#[derive(Clone, Debug, PartialEq)]
enum PointFn {
    Noop,
    AreaFirst,
    Area,
}
#[derive(Clone, Debug, PartialEq)]
enum LineStartFn {
    Noop,
    AreaRingStart,
}

#[derive(Clone, Debug, PartialEq)]
enum LineEndFn {
    Noop,
    AreaRingEnd,
}

impl<T> Default for Area<T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self {
            lambda00: T::nan(),
            phi00: T::nan(),
            lambda0: T::nan(),
            cos_phi0: T::nan(),
            sin_phi0: T::nan(),
            two: T::from(2_f64).unwrap(),
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
    T: CoordFloat + FloatConst,
{
    /// Calculate the objects associated area.
    pub fn calc(object: &impl Streamable<T = T>) -> T {
        let mut a = Self::default();
        object.to_stream(&mut a);
        a.area_sum * a.two
    }

    fn area_point_first(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        self.point_fn = PointFn::Area;
        self.lambda00 = p.x;
        self.phi00 = p.y;
        self.lambda0 = p.x.to_radians();
        let phi = p.y.to_radians();
        let phi = phi / self.two + T::FRAC_PI_4();
        (self.sin_phi0, self.cos_phi0) = phi.sin_cos();
    }

    #[allow(clippy::similar_names)]
    fn area_point(&mut self, p: &Coordinate<T>, _m: Option<u8>) {
        let lambda = p.x.to_radians();
        let phi = p.y.to_radians();

        let phi = phi / self.two + T::FRAC_PI_4(); // half the angular distance from south pole

        // Spherical excess E for a spherical triangle with vertices: south pole,
        // previous point, current point. Uses a formula derived from Cagnoli’s
        // theorem. See Todhunter, Spherical Trig. (1871), Sec. 103, Eq. (2).
        let d_lambda = lambda - self.lambda0;
        let sd_lambda = if d_lambda >= T::zero() {
            T::one()
        } else {
            -T::one()
        };

        let ad_lambda = sd_lambda * d_lambda;
        let (ad_lambda_sin, ad_lambda_cos) = ad_lambda.sin_cos();
        let (sin_phi, cos_phi) = phi.sin_cos();
        let k = self.sin_phi0 * sin_phi;
        let u = self.cos_phi0 * cos_phi + k * ad_lambda_cos;
        let v = k * sd_lambda * ad_lambda_sin;
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
    T: CoordFloat + FloatConst,
{
    type EP = Self;
    type T = T;

    #[inline]
    fn endpoint<'a>(&mut self) -> &mut Self {
        self
    }

    #[inline]
    fn line_end(&mut self) {
        match self.line_end_fn {
            LineEndFn::AreaRingEnd => self.area_ring_end(),
            LineEndFn::Noop => {}
        }
    }

    #[inline]
    fn line_start(&mut self) {
        match self.line_start_fn {
            LineStartFn::AreaRingStart => self.area_ring_start(),
            LineStartFn::Noop => {}
        }
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self.point_fn {
            PointFn::AreaFirst => {
                self.area_point_first(p, m);
            }
            PointFn::Area => self.area_point(p, m),
            PointFn::Noop => {}
        }
    }
    fn polygon_end(&mut self) {
        let area_ring = self.area_ring_sum;
        if area_ring < T::zero() {
            self.area_sum = self.area_sum + T::TAU() + area_ring;
        } else {
            self.area_sum = self.area_sum + area_ring;
        }

        self.line_start_fn = LineStartFn::Noop;
        self.line_end_fn = LineEndFn::Noop;
        self.point_fn = PointFn::Noop;
    }

    fn polygon_start(&mut self) {
        self.area_ring_sum = T::zero();
        self.line_start_fn = LineStartFn::AreaRingStart;
        self.line_end_fn = LineEndFn::AreaRingEnd;
    }

    #[inline]
    fn sphere(&mut self) {
        self.area_sum = self.area_sum + T::TAU();
    }
}
