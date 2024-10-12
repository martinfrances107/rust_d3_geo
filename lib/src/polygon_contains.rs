use geo::coords_iter::CoordsIter;
use geo::CoordFloat;
use geo::LineString;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::cartesian::cartesian;
use crate::cartesian::cross;
use crate::cartesian::normalize_in_place;
use crate::math::EPSILON2;
use crate::EPSILON;

#[inline]
fn longitude(point: &Coord<f64>) -> f64 {
    if point.x.abs() <= f64::PI() {
        point.x
    } else {
        point.x.signum()
            * ((point.x.abs() + f64::PI()) % f64::TAU() - f64::PI())
    }
}

/// Determines whether a point is inside the polygon.
///
/// # Panics
/// `unwrap()` is used here but a panic will never happen as EPSILON will always be converted into T.
pub fn polygon_contains<T>(polygon: &[LineString<T>], point: &Coord<T>) -> bool
where
    T: CoordFloat + FloatConst,
{
    let lambda = longitude(&Coord {
        x: point.x.to_f64().unwrap(),
        y: point.y.to_f64().unwrap(),
    });
    let mut phi = point.y.to_f64().unwrap();
    let sin_phi = phi.sin();
    let (sin_lambda, cos_lambda) = lambda.sin_cos();
    let normal = [sin_lambda, -cos_lambda, 0_f64];
    let mut angle = 0_f64;

    let mut sum = 0_f64;
    let mut winding = 0_i32;

    if sin_phi == 1_f64 {
        phi = f64::FRAC_PI_2() + EPSILON;
    } else if sin_phi == -1_f64 {
        phi = -f64::FRAC_PI_2() - EPSILON;
    }

    for ring in polygon {
        let m = ring.coords_count();
        if m == 0 {
            continue;
        };

        let mut point0 = Coord {
            x: ring[m - 1].x.to_f64().unwrap(),
            y: ring[m - 1].y.to_f64().unwrap(),
        };
        let mut lambda0 = longitude(&point0);
        let phi0 = point0.y / 2_f64 + f64::FRAC_PI_4();
        let (mut sin_phi0, mut cos_phi0) = phi0.sin_cos();

        let mut j = 0;
        loop {
            if j >= m {
                break;
            }
            let point1 = Coord {
                x: ring[j].x.to_f64().unwrap(),
                y: ring[j].y.to_f64().unwrap(),
            };
            let lambda1 = longitude(&point1);
            let phi1 = point1.y / 2_f64 + f64::FRAC_PI_4();
            let (sin_phi1, cos_phi1) = phi1.sin_cos();
            let delta = lambda1 - lambda0;
            let sign = delta.signum();
            let abs_delta = sign * delta;
            let antimeridian = abs_delta > f64::PI();
            let k = sin_phi0 * sin_phi1;

            sum += (k * sign * abs_delta.sin())
                .atan2(cos_phi0.mul_add(cos_phi1, k * abs_delta.cos()));
            angle += if antimeridian {
                sign.mul_add(f64::TAU(), delta)
            } else {
                delta
            };
            // Are the longitudes either side of the point’s meridian (lambda),
            // and are the latitudes smaller than the parallel (phi)?
            if antimeridian ^ (lambda0 >= lambda) ^ (lambda1 >= lambda) {
                let mut arc = cross(&cartesian(&point0), &cartesian(&point1));
                normalize_in_place(&mut arc);
                let mut intersection = cross(&normal, &arc);
                normalize_in_place(&mut intersection);
                let phi_arc = if antimeridian ^ (delta >= 0_f64) {
                    -(intersection[2].asin())
                } else {
                    intersection[2].asin()
                };
                if phi > phi_arc
                    || (phi == phi_arc && (arc[0] != 0_f64 || arc[1] != 0_f64))
                {
                    if antimeridian ^ (delta >= 0_f64) {
                        winding += 1;
                    } else {
                        winding -= 1;
                    };
                }
            }
            // Loop is about the restart.
            j += 1;
            lambda0 = lambda1;
            sin_phi0 = sin_phi1;
            cos_phi0 = cos_phi1;
            point0 = point1;
        }
    }

    // First, determine whether the South pole is inside or outside:
    //
    // It is inside if:
    // * the polygon winds around it in a clockwise direction.
    // * the polygon does not (cumulatively) wind around it, but has a negative
    //   (counter-clockwise) area.
    //
    // Second, count the (signed) number of times a segment crosses a lambda
    // from the point to the South pole. If it is zero, then the point is the
    // same side as the South pole.
    let is_winding_odd = winding & 1 == 1;

    let is_south_pole_inside =
        angle < -EPSILON || (angle < EPSILON && sum < -EPSILON2);

    is_south_pole_inside ^ is_winding_odd
}

#[cfg(test)]
mod polygon_precision {

    // This has no equivalent in the javascript version
    //
    /// This test addresses a longstanding issue ...
    ///
    /// javascript uses a Addr class to improve precision in this
    /// `polygon_contains`.
    ///
    /// I dicovered artifacts appearing/flashing in `examples/globe/rotating_wgpu`
    /// traced to this function. When the angle values are close zero
    /// this was occassionaly tripping up the computation of `is_south_pole_inside`
    /// [ -EPSILON<angle<EPSILON ]
    ///
    /// This issue only appears when streaming `MultiPolygons/Polygons`
    ///
    /// `polygon_contains()` was returning TRUE not FALSE.
    ///
    /// I tracked down the issue by pulling code from examples/ring
    /// and then simplifying the polygon to the simplests geometry that still
    /// flashed
    ///
    /// "A simple square (closed ring described by 5 points)"
    ///
    /// ( the flashing occurred when the normal for the ring pointed directly away
    /// from the observer).
    use geo::LineString;
    use geo_types::Coord;

    use crate::polygon_contains::polygon_contains;

    #[test]
    fn hidden_square() {
        let ring = [LineString::<f32>::from(vec![
            Coord {
                x: 2.697_413_4,
                y: 0.702_794_6,
            },
            Coord {
                x: 2.817_812_2,
                y: 0.935_154,
            },
            Coord {
                x: -3.127_537_3,
                y: 0.815_508_5,
            },
            Coord {
                x: 2.983_517_2,
                y: 0.606_184_96,
            },
            Coord {
                x: 2.697_413_2,
                y: 0.702_794_7,
            },
        ])];

        let point = Coord::<f32> {
            x: -3.141_592_7,
            y: -1.570_796_4,
        };
        assert!(!polygon_contains(&ring, &point));
    }
}
