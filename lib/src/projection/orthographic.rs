use core::fmt::Debug;
use core::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo_types::Coord;
use num_traits::float::FloatConst;

use crate::math::EPSILON;
use crate::Transform;

use super::azimuthal::azimuthal_invert;
use super::builder::types::BuilderCircleResampleNoClip;
use super::builder::Builder;
use super::BuilderTrait;
use super::ClipAngleSet;
use super::RawBase;
use super::ScaleSet;

#[inline]
fn angle<T>(z: T) -> T
where
    T: CoordFloat,
{
    z.asin()
}

/// Projection definition. ``Orthographic::builder()`` returns a builder.
#[derive(Clone, Default, Debug)]
pub struct Orthographic<T> {
    p_t: PhantomData<T>,
}

impl<T> RawBase for Orthographic<T>
where
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + Default + FloatConst,
{
    type Builder<DRAIN: Clone> = BuilderCircleResampleNoClip<DRAIN, Self, T>;

    #[inline]
    fn builder<DRAIN: Clone>() -> Self::Builder<DRAIN> {
        let mut b = Builder::new(Self::default());
        b.scale_set(T::from(249.5_f64).unwrap());
        b.clip_angle_set(T::from(90_f64 + EPSILON).unwrap())
    }
}

impl<T> Orthographic<T> where T: CoordFloat + FloatConst {}

impl<T> Transform for Orthographic<T>
where
    T: CoordFloat,
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
        azimuthal_invert(p, angle)
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

        let mut b = Orthographic::<f64>::builder::<DrainStub<f64>>();
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
