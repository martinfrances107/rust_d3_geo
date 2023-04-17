use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Range;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::last_point::LastPoint;
use crate::math::EPSILON;
use crate::projection::Projector;
use crate::stream::Stream;
use crate::Transform;

use super::albers::albers;
use super::BuilderTrait;
use super::CenterSet;
use super::ClipExtentSet;
use super::RotateSet;
use super::ScaleSet;

use super::builder_albers_usa::Builder;
use super::builder_conic::types::BuilderConicAntimeridianResampleClip;
use super::builder_conic::types::BuilderConicAntimeridianResampleNoClip;
use super::equal_area::EqualArea;
use super::Build;
use super::RawBase;
use super::ScaleGet;
use super::TranslateGet;
use super::TranslateSet;

/// Projection definition.
///
/// Why is the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Debug)]
pub struct AlbersUsa<DRAIN, T>
where
    DRAIN: Clone,
    T: CoordFloat + Debug + Default + FloatConst,
{
    p_drain: PhantomData<DRAIN>,
    k: T,
    t: Coord<T>,

    // The builders with clip_extent() applied.
    pub(super) lower_48_point:
        BuilderConicAntimeridianResampleClip<LastPoint<T>, EqualArea<LastPoint<T>, T>, T>,
    pub(super) alaska_point:
        BuilderConicAntimeridianResampleClip<LastPoint<T>, EqualArea<LastPoint<T>, T>, T>,
    pub(super) hawaii_point:
        BuilderConicAntimeridianResampleClip<LastPoint<T>, EqualArea<LastPoint<T>, T>, T>,

    // The builder with base setting used as a starting point everytime translate is adjusted.
    pub(super) lower_48: BuilderConicAntimeridianResampleNoClip<DRAIN, EqualArea<DRAIN, T>, T>,
    pub(super) alaska: BuilderConicAntimeridianResampleNoClip<DRAIN, EqualArea<DRAIN, T>, T>,
    pub(super) hawaii: BuilderConicAntimeridianResampleNoClip<DRAIN, EqualArea<DRAIN, T>, T>,
    alaska_x: Range<T>,
    alaska_y: Range<T>,

    hawaii_x: Range<T>,
    hawaii_y: Range<T>,
}

impl<DRAIN, T> Default for AlbersUsa<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: CoordFloat + Debug + Default + FloatConst,
{
    fn default() -> Self {
        let epsilon = T::from(EPSILON).unwrap();
        let alaska_y: Range<T> = T::from(0.120).unwrap()..T::from(0.234).unwrap();
        let alaska_x: Range<T> = T::from(-0.425).unwrap()..T::from(-0.214).unwrap();

        let hawaii_x: Range<T> = T::from(-0.214).unwrap()..T::from(-0.115).unwrap();
        let hawaii_y: Range<T> = T::from(0.166).unwrap()..T::from(0.234).unwrap();

        let mut alaska = EqualArea::builder();
        alaska
            .rotate2_set(&[T::from(154_f64).unwrap(), T::zero()])
            .center_set(&Coord {
                x: T::from(-2_f64).unwrap(),
                y: T::from(58.5_f64).unwrap(),
            });

        let mut hawaii = EqualArea::builder();
        hawaii
            .rotate2_set(&[T::from(157_f64).unwrap(), T::zero()])
            .center_set(&Coord {
                x: T::from(-3_f64).unwrap(),
                y: T::from(19.9_f64).unwrap(),
            });

        let lower_48 = albers();

        let k: T = lower_48.scale();
        let t = lower_48.translate();

        let mut lower_48_point = albers();
        let lower_48_point = lower_48_point.translate_set(&t).clip_extent_set(&[
            Coord {
                x: T::from(0.455_f64).unwrap().mul_add(-k, t.x),
                y: T::from(0.234_f64).unwrap().mul_add(-k, t.y),
            },
            Coord {
                x: T::from(0.455_f64).unwrap().mul_add(k, t.x),
                y: T::from(0.234_f64).unwrap().mul_add(k, t.y),
            },
        ]);

        let mut alaska_point = EqualArea::builder();
        alaska_point
            .rotate2_set(&[T::from(154_f64).unwrap(), T::zero()])
            .center_set(&Coord {
                x: T::from(-2_f64).unwrap(),
                y: T::from(58.5_f64).unwrap(),
            })
            .translate_set(&Coord {
                x: T::from(0.307_f64).unwrap().mul_add(-k, t.x),
                y: T::from(0.201_f64).unwrap().mul_add(-k, t.y),
            });
        let alaska_point = alaska_point.clip_extent_set(&[
            Coord {
                x: T::from(0.425_f64).unwrap().mul_add(-k, t.x) + epsilon,
                y: T::from(0.120_f64).unwrap().mul_add(-k, t.y) + epsilon,
            },
            Coord {
                x: T::from(0.214_f64).unwrap().mul_add(-k, t.x) - epsilon,
                y: T::from(0.234_f64).unwrap().mul_add(-k, t.y) - epsilon,
            },
        ]);

        let mut hawaii_point = EqualArea::builder();
        hawaii_point
            .rotate2_set(&[T::from(157_f64).unwrap(), T::zero()])
            .center_set(&Coord {
                x: T::from(-3_f64).unwrap(),
                y: T::from(19.9_f64).unwrap(),
            })
            .translate_set(&Coord {
                x: T::from(0.205_f64).unwrap().mul_add(-k, t.x),
                y: T::from(0.212_f64).unwrap().mul_add(-k, t.y),
            });
        let hawaii_point = hawaii_point.clip_extent_set(&[
            Coord {
                x: T::from(0.214_f64).unwrap().mul_add(-k, t.x) + epsilon,
                y: T::from(0.166_f64).unwrap().mul_add(-k, t.y) + epsilon,
            },
            Coord {
                x: T::from(0.214_f64).unwrap().mul_add(-k, t.x) + epsilon,
                y: T::from(0.234_f64).unwrap().mul_add(k, t.y) - epsilon,
            },
        ]);

        Self {
            p_drain: PhantomData::<DRAIN>,
            k,
            t,
            // Initially there is not difference between builder with base settings and
            // Builder with translation applied.
            alaska_point,
            lower_48_point,
            hawaii_point,

            alaska,
            lower_48,
            hawaii,
            alaska_y,
            alaska_x,

            hawaii_x,
            hawaii_y,
        }
    }
}

impl<DRAIN, T> RawBase for AlbersUsa<DRAIN, T>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = T>,
    T: 'static + CoordFloat + Debug + Default + FloatConst,
{
    type Builder = Builder<DRAIN, T>;

    #[inline]
    fn builder() -> Builder<DRAIN, T> {
        let mut b = Builder::new(Self::default());
        b.scale_set(T::from(1070_f64).unwrap());
        b
    }
}

use crate::path::Result;
impl<DRAIN, T> Transform for AlbersUsa<DRAIN, T>
where
    DRAIN: Clone + Stream<EP = DRAIN, T = T>,
    T: 'static + CoordFloat + Debug + Default + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        let mut lower_48_end_point = LastPoint::default();
        let mut pipeline = self.lower_48_point.build().stream(&lower_48_end_point);

        pipeline.point(p, None);
        lower_48_end_point.result().map_or_else(
            || {
                let mut alaska_end_point = LastPoint::default();
                let mut pipeline = self.alaska_point.build().stream(&alaska_end_point);
                pipeline.point(p, None);
                alaska_end_point.result().map_or_else(
                    || {
                        let mut hawaii_end_point = LastPoint::default();
                        let mut pipeline = self.hawaii_point.build().stream(&hawaii_end_point);
                        pipeline.point(p, None);
                        hawaii_end_point.result().map_or(
                            Coord {
                                x: T::nan(),
                                y: T::nan(),
                            },
                            |t| t,
                        )
                    },
                    |t| t,
                )
            },
            |t| t,
        )
    }

    #[inline]
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        let x = (p.x - self.t.x) / self.k;
        let y = (p.y - self.t.y) / self.k;
        if self.alaska_y.contains(&y) && self.alaska_x.contains(&x) {
            self.alaska_point.build().invert(p)
        } else if self.hawaii_y.contains(&y) && self.hawaii_x.contains(&x) {
            self.hawaii_point.build().invert(p)
        } else {
            self.lower_48_point.build().invert(p)
        }
    }
}
