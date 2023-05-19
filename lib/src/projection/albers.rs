use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::ClipAntimeridianC;
use crate::clip::circle::ClipCircleC;

use super::builder::template::ResampleNoPCNC;
use super::builder::template::ResampleNoneNoPCNC;
use super::builder::types::BuilderAntimeridianResampleNoClip;
use super::builder_conic::Builder;
use super::builder_conic::ParallelsSet;
use super::equal_area::EqualArea;
use super::CenterSet;
use super::RawBase;
use super::RotateSet;
use super::ScaleSet;
use super::TranslateSet;

type Output<DRAIN, T> = Builder<BuilderAntimeridianResampleNoClip<DRAIN, EqualArea<T>, T>, T>;

/// Albers - `ConicEqualArea` centered on the U.S.
///
/// # Panics
/// unwrap() is used here but a panic will never happen as constants  will
/// always be converted into T.
#[must_use]
pub fn albers<DRAIN, T>() -> Output<DRAIN, T>
where
    DRAIN: Clone,
    T: CoordFloat + Default + FloatConst,
{
    let mut b = EqualArea::<T>::builder();
    b.parallels_set(T::from(29.5_f64).unwrap(), T::from(45.5_f64).unwrap())
        .scale_set::<ClipAntimeridianC<ResampleNoPCNC<DRAIN, EqualArea<T>, T>, T>>(
            T::from(1070_f64).unwrap(),
        )
        .translate_set(&Coord {
            x: T::from(480_f64).unwrap(),
            y: T::from(250_f64).unwrap(),
        })
        .rotate2_set(&[T::from(96_f64).unwrap(), T::zero()])
        .center_set::<ClipCircleC<ResampleNoneNoPCNC<DRAIN, EqualArea<T>, T>, T>>(&Coord {
            x: T::from(-0.6_f64).unwrap(),
            y: T::from(38.7_f64).unwrap(),
        });
    b
}
