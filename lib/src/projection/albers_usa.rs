use std::marker::PhantomData;
use std::ops::Range;

use geo_types::Coord;

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
pub struct AlbersUsa<DRAIN> {
    p_drain: PhantomData<DRAIN>,
    k: f64,
    t: Coord<f64>,

    // The builders with clip_extent() applied.
    pub(super) lower_48_point:
        BuilderConicAntimeridianResampleClip<LastPoint<f64>, EqualArea<LastPoint<f64>, f64>, f64>,
    pub(super) alaska_point:
        BuilderConicAntimeridianResampleClip<LastPoint<f64>, EqualArea<LastPoint<f64>, f64>, f64>,
    pub(super) hawaii_point:
        BuilderConicAntimeridianResampleClip<LastPoint<f64>, EqualArea<LastPoint<f64>, f64>, f64>,

    // The builder with base setting used as a starting point everytime translate is adjusted.
    pub(super) lower_48: BuilderConicAntimeridianResampleNoClip<DRAIN, EqualArea<DRAIN, f64>, f64>,
    pub(super) alaska: BuilderConicAntimeridianResampleNoClip<DRAIN, EqualArea<DRAIN, f64>, f64>,
    pub(super) hawaii: BuilderConicAntimeridianResampleNoClip<DRAIN, EqualArea<DRAIN, f64>, f64>,
}

impl<DRAIN> Default for AlbersUsa<DRAIN>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    fn default() -> Self {
        let mut alaska = EqualArea::builder();
        alaska.rotate2_set(&[154_f64, 0_f64]).center_set(&Coord {
            x: -2_f64,
            y: 58.5_f64,
        });

        let mut hawaii = EqualArea::builder();
        hawaii.rotate2_set(&[157_f64, 0_f64]).center_set(&Coord {
            x: -3_f64,
            y: 19.9_f64,
        });

        let lower_48 = albers();

        let k: f64 = lower_48.scale();
        let t = lower_48.translate();

        let mut lower_48_point = albers();
        let lower_48_point = lower_48_point.translate_set(&t).clip_extent_set(&[
            Coord {
                x: 0.455_f64.mul_add(-k, t.x),
                y: 0.234f64.mul_add(-k, t.y),
            },
            Coord {
                x: 0.455_f64.mul_add(k, t.x),
                y: 0.234f64.mul_add(k, t.y),
            },
        ]);

        let mut alaska_point = EqualArea::builder();
        alaska_point
            .rotate2_set(&[154_f64, 0_f64])
            .center_set(&Coord {
                x: -2_f64,
                y: 58.5_f64,
            })
            .translate_set(&Coord {
                x: 0.307_f64.mul_add(-k, t.x),
                y: 0.201f64.mul_add(-k, t.y),
            });
        let alaska_point = alaska_point.clip_extent_set(&[
            Coord {
                x: 0.425_f64.mul_add(-k, t.x) + EPSILON,
                y: 0.120_f64.mul_add(-k, t.y) + EPSILON,
            },
            Coord {
                x: 0.214_f64.mul_add(-k, t.x) - EPSILON,
                y: 0.234f64.mul_add(-k, t.y) - EPSILON,
            },
        ]);

        let mut hawaii_point = EqualArea::builder();
        hawaii_point
            .rotate2_set(&[157_f64, 0_f64])
            .center_set(&Coord {
                x: -3_f64,
                y: 19.9_f64,
            })
            .translate_set(&Coord {
                x: 0.205_f64.mul_add(-k, t.x),
                y: 0.212_f64.mul_add(-k, t.y),
            });
        let hawaii_point = hawaii_point.clip_extent_set(&[
            Coord {
                x: 0.214_f64.mul_add(-k, t.x) + EPSILON,
                y: 0.166f64.mul_add(-k, t.y) + EPSILON,
            },
            Coord {
                x: 0.214_f64.mul_add(-k, t.x) + EPSILON,
                y: 0.234_f64.mul_add(k, t.y) - EPSILON,
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
        }
    }
}

impl<DRAIN> RawBase for AlbersUsa<DRAIN>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    type Builder = Builder<DRAIN>;

    #[inline]
    fn builder() -> Builder<DRAIN> {
        let mut b = Builder::new(Self::default());
        b.scale_set(1070_f64);
        b
    }
}

const ALASKA_Y: Range<f64> = 0.120..0.234;
const ALASKA_X: Range<f64> = -0.425..-0.214;

const HAWAII_X: Range<f64> = -0.214..-0.115;
const HAWAII_Y: Range<f64> = 0.166..0.234;

use crate::path::Result;
impl<DRAIN> Transform for AlbersUsa<DRAIN> {
    type T = f64;

    #[inline]
    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
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
                                x: f64::NAN,
                                y: f64::NAN,
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
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        let x = (p.x - self.t.x) / self.k;
        let y = (p.y - self.t.y) / self.k;
        if ALASKA_Y.contains(&y) && ALASKA_X.contains(&x) {
            self.alaska_point.build().invert(p)
        } else if HAWAII_Y.contains(&y) && HAWAII_X.contains(&x) {
            self.hawaii_point.build().invert(p)
        } else {
            self.lower_48_point.build().invert(p)
        }
    }
}
