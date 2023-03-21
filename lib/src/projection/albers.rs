use geo::{Coord, CoordFloat};
use num_traits::FloatConst;

use crate::stream::Stream;

use super::builder::types::BuilderAntimeridianResampleNoClip;
use super::builder_conic::Builder;
use super::builder_conic::ParallelsSet;
use super::equal_area::EqualArea;
use super::CenterSet;
use super::RawBase;
use super::RotateSet;
use super::ScaleSet;
use super::TranslateSet;

/// Output type of the function albers().
pub type BuilderAlbers<DRAIN, T> =
    Builder<BuilderAntimeridianResampleNoClip<DRAIN, EqualArea<DRAIN, T>, T>, T>;

/// Albers - `ConicEqualArea` centered on the U.S.
///
/// # Panics
/// unwrap() is used here but a panic will never happen as constants  will
/// always be converted into T.
#[must_use]
pub fn albers<DRAIN, T>() -> BuilderAlbers<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    let mut b = EqualArea::<DRAIN, T>::builder();
    b.parallels_set(T::from(29.5_f64).unwrap(), T::from(45.5_f64).unwrap())
        .scale_set(T::from(1070_f64).unwrap())
        .translate_set(&Coord {
            x: T::from(480_f64).unwrap(),
            y: T::from(250_f64).unwrap(),
        })
        .rotate2_set(&[T::from(96_f64).unwrap(), T::zero()])
        .center_set(&Coord {
            x: T::from(-0.6).unwrap(),
            y: T::from(38.7).unwrap(),
        });
    b
}
