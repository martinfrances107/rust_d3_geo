use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo_types::Coord;
use num_traits::float::FloatConst;

use crate::math::EPSILON;
use crate::projection::builder::types::BuilderCircleResampleNoClip;
use crate::projection::ScaleSet;
use crate::stream::Stream;
use crate::Transform;

use super::builder::Builder;
use super::ClipAngleSet;
use super::RawBase;

#[inline]
fn angle<T>(z: T) -> T
where
    T: CoordFloat + FloatConst,
{
    z.asin()
}

fn azimuthal_invert<T>(p: &Coord<T>) -> Coord<T>
where
    T: CoordFloat + FloatConst,
{
    let z = (p.x * p.x + p.y * p.y).sqrt();
    let c = angle(z);
    let (sc, cc) = c.sin_cos();

    let ret_x = (p.x * sc).atan2(z * cc);

    let y_out = if z == T::zero() { z } else { p.y * sc / z };
    let ret_y = y_out.asin();

    Coord { x: ret_x, y: ret_y }
}
/// Projection definition.
#[derive(Clone, Copy, Default, Debug)]
pub struct Orthographic<DRAIN, T> {
    p_drain: PhantomData<DRAIN>,
    p_t: PhantomData<T>,
}

impl<DRAIN, T> RawBase for Orthographic<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: AbsDiffEq<Epsilon = T> + CoordFloat + Default + FloatConst,
{
    type Builder = BuilderCircleResampleNoClip<DRAIN, Self, T>;

    #[inline]
    fn builder() -> Self::Builder {
        let mut b = Builder::new(Self::default());
        b.scale_set(T::from(249.5_f64).unwrap());
        b.clip_angle_set(T::from(90_f64 + EPSILON).unwrap())
    }
}

impl<DRAIN, T> Orthographic<DRAIN, T> where T: CoordFloat + FloatConst {}

impl<DRAIN, T> Transform for Orthographic<DRAIN, T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        let (sin_y, cos_y) = p.y.sin_cos();
        Coord {
            x: cos_y * p.x.sin(),
            y: sin_y,
        }
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        azimuthal_invert(p)
    }
}

#[cfg(test)]
mod drag_and_zoom {
    use geo::Coord;

    use crate::projection::RawBase;
    use crate::projection::ScaleSet;
    use crate::projection::TranslateSet;
    use crate::stream::DrainStub;
    use crate::Transform;

    use super::Orthographic;
    use crate::in_delta::point as in_delta_point;

    /// This test is not copied from javascript.
    ///
    /// It was used when problems were identified in
    /// `example/drag_and_zoom`.
    #[test]
    fn drag_and_zoom() {
        let w = 1800_f64;
        let h = 1200_f64;

        let mut b = Orthographic::<DrainStub<_>, _>::builder();
        b.scale_set(w / 1.3_f64 / std::f64::consts::PI);
        b.translate_set(&Coord {
            x: w / 2_f64,
            y: h / 2_f64,
        });

        let start = Coord {
            x: 10_f64,
            y: 10_f64,
        };

        let transformed1 = b.transform(&start);

        let expected = Coord {
            x: 975.370_425_850_078_2_f64,
            y: 523.466_863_842_669_3_f64,
        };

        assert_eq!(transformed1, expected);

        // Back to the start.
        let transformed2 = b.invert(&expected);
        assert!(in_delta_point(transformed2.into(), start.into(), 1e-6));
    }
}
