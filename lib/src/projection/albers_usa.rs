use core::fmt::Debug;
use std::ops::Range;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::buffer::Buffer;
use crate::clip::clipper::Clipper;
use crate::clip::clipper::Connected as ConnectedClipper;
use crate::clip::rectangle::Rectangle;
use crate::last_point::LastPoint;
use crate::math::EPSILON;
use crate::path::Result;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::Projector;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connected as ConnectedStream;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::albers::albers;
use super::builder_albers_usa::Builder;
use super::builder_conic::types::BuilderConicAntimeridianResampleClip;
use super::builder_conic::ParallelsSet;
use super::equal_area::EqualArea;
use super::resampler::resample::Resample;
use super::stream_transform_radians::StreamTransformRadians;
use super::Build;
use super::BuilderTrait;
use super::CenterSet;
use super::ClipExtentSet;
use super::RawBase;
use super::RotateSet;
use super::ScaleGet;
use super::ScaleSet;
use super::TranslateGet;
use super::TranslateSet;

type StreamPoint<DRAIN, T> = StreamTransformRadians<
    ConnectedStream<
        RotatorRadians<
            ConnectedStream<
                Clipper<
                    Interpolate<T>,
                    Line<Unconnected, T>,
                    Resample<
                        EqualArea<T>,
                        ConnectedResample<Rectangle<ConnectedStream<DRAIN>, T>, T>,
                        T,
                    >,
                    ConnectedClipper<
                        Line<ConnectedStream<Buffer<T>>, T>,
                        Line<
                            ConnectedStream<
                                Resample<
                                    EqualArea<T>,
                                    ConnectedResample<Rectangle<ConnectedStream<DRAIN>, T>, T>,
                                    T,
                                >,
                            >,
                            T,
                        >,
                        T,
                    >,
                    T,
                >,
            >,
            T,
        >,
    >,
>;

/// Projection definition.
#[derive(Clone, Debug)]
pub struct AlbersUsa<SD, T>
where
    T: CoordFloat + Debug + FloatConst,
{
    k: T,
    t: Coord<T>,

    alaska_x: Range<T>,
    alaska_y: Range<T>,

    hawaii_x: Range<T>,
    hawaii_y: Range<T>,

    // The builder with base setting used as a starting point everytime translate is adjusted.
    pub(super) alaska: BuilderConicAntimeridianResampleClip<LastPoint<T>, EqualArea<T>, T>,
    pub(super) lower_48: BuilderConicAntimeridianResampleClip<LastPoint<T>, EqualArea<T>, T>,
    pub(super) hawaii: BuilderConicAntimeridianResampleClip<LastPoint<T>, EqualArea<T>, T>,

    pub(super) alaska_point: StreamPoint<LastPoint<T>, T>,
    pub(super) lower_48_point: StreamPoint<LastPoint<T>, T>,
    pub(super) hawaii_point: StreamPoint<LastPoint<T>, T>,

    pub(super) alaska_stream: BuilderConicAntimeridianResampleClip<SD, EqualArea<T>, T>,
    pub(super) lower_48_stream: BuilderConicAntimeridianResampleClip<SD, EqualArea<T>, T>,
    pub(super) hawaii_stream: BuilderConicAntimeridianResampleClip<SD, EqualArea<T>, T>,
}

/// Construct a Projection builder capable of rendering the view of alaksa.
fn alaska_inset<DRAIN: Clone, T>(
    scaling_factor: T,
    k: T,
    t: Coord<T>,
    epsilon: T,
) -> BuilderConicAntimeridianResampleClip<DRAIN, EqualArea<T>, T>
where
    T: CoordFloat + Default + FloatConst,
{
    let mut alaska = EqualArea::builder();
    alaska
        .rotate2_set(&[T::from(154_f64).unwrap(), T::zero()])
        .center_set(&Coord {
            x: T::from(-2_f64).unwrap(),
            y: T::from(58.5_f64).unwrap(),
        })
        .parallels_set(T::from(55_f64).unwrap(), T::from(65_f64).unwrap());

    // Emulate .scale() call.
    let alaska = alaska.scale_set(T::from(0.35).unwrap() * scaling_factor);

    // Emulate .translate() call.
    alaska
        .translate_set(&Coord {
            x: T::from(0.307_f64).unwrap().mul_add(-k, t.x),
            y: t.y + T::from(0.201).unwrap() * k,
        })
        .clip_extent_set(&[
            Coord {
                x: T::from(0.425_f64).unwrap().mul_add(-k, t.x) + epsilon,
                y: t.y + T::from(0.120_f64).unwrap() * k + epsilon,
            },
            Coord {
                x: T::from(0.214_f64).unwrap().mul_add(-k, t.x) - epsilon,
                y: t.y + T::from(0.234_f64).unwrap() * k - epsilon,
            },
        ])
}

/// Construct a Projection builder capable of rendering the view of hawaii
fn hawaii_inset<DRAIN: Clone, T>(
    scaling_factor: T,
    k: T,
    t: Coord<T>,
    epsilon: T,
) -> BuilderConicAntimeridianResampleClip<DRAIN, EqualArea<T>, T>
where
    T: CoordFloat + Default + FloatConst,
{
    let mut hawaii = EqualArea::builder();
    hawaii
        .rotate2_set(&[T::from(157_f64).unwrap(), T::zero()])
        .center_set(&Coord {
            x: T::from(-3_f64).unwrap(),
            y: T::from(19.9_f64).unwrap(),
        })
        .parallels_set(T::from(8_f64).unwrap(), T::from(18_f64).unwrap());

    let hawaii = hawaii.scale_set(scaling_factor);

    hawaii
        .translate_set(&Coord {
            x: T::from(0.205_f64).unwrap().mul_add(-k, t.x),
            y: T::from(0.212f64).unwrap().mul_add(k, t.y),
        })
        .clip_extent_set(&[
            Coord {
                x: T::from(0.214_f64).unwrap().mul_add(-k, t.x) + epsilon,
                y: t.y + T::from(0.166).unwrap() * k + epsilon,
            },
            Coord {
                x: t.x - T::from(0.115).unwrap() * k - epsilon,
                y: T::from(0.234f64).unwrap().mul_add(k, t.y) - epsilon,
            },
        ])
}

impl<SD, T> Default for AlbersUsa<SD, T>
where
    SD: Clone,
    T: CoordFloat + Default + FloatConst,
{
    fn default() -> Self {
        let epsilon = T::from(EPSILON).unwrap();
        let scaling_factor = T::from(1070).unwrap();

        let mut lower_48 = albers();
        let mut lower_48_stream = albers::<SD, T>();

        // Emulate .scale() call.
        let lower_48 = lower_48.scale_set(scaling_factor);
        let lower_48_stream = lower_48_stream.scale_set(scaling_factor);

        // Emulate .translate() call.
        let k: T = lower_48.scale();
        let t = lower_48.translate();

        let lower_48 = lower_48.translate_set(&t).clip_extent_set(&[
            Coord {
                x: T::from(0.455_f64).unwrap().mul_add(-k, t.x),
                y: T::from(0.234_f64).unwrap().mul_add(-k, t.y),
            },
            Coord {
                x: T::from(0.455_f64).unwrap().mul_add(k, t.x),
                y: T::from(0.234_f64).unwrap().mul_add(k, t.y),
            },
        ]);
        let lower_48_stream = lower_48_stream.translate_set(&t).clip_extent_set(&[
            Coord {
                x: T::from(0.455_f64).unwrap().mul_add(-k, t.x),
                y: T::from(0.234_f64).unwrap().mul_add(-k, t.y),
            },
            Coord {
                x: T::from(0.455_f64).unwrap().mul_add(k, t.x),
                y: T::from(0.234_f64).unwrap().mul_add(k, t.y),
            },
        ]);

        // Used in the invert()
        let alaska = alaska_inset(scaling_factor, k, t, epsilon);
        let hawaii = hawaii_inset(scaling_factor, k, t, epsilon);

        // Point is used in the foreward transform.
        let lower_48_point = lower_48.build().stream(&LastPoint::default());
        let alaska_point = alaska.build().stream(&LastPoint::default());
        let hawaii_point = hawaii.build().stream(&LastPoint::default());

        let alaska_stream = alaska_inset(scaling_factor, k, t, epsilon);
        let hawaii_stream = hawaii_inset(scaling_factor, k, t, epsilon);

        Self {
            k,
            t,

            alaska_y: T::from(0.120).unwrap()..T::from(0.234).unwrap(),
            alaska_x: T::from(-0.425).unwrap()..T::from(-0.214).unwrap(),

            hawaii_x: T::from(-0.214).unwrap()..T::from(-0.115).unwrap(),
            hawaii_y: T::from(0.166).unwrap()..T::from(0.234).unwrap(),

            alaska,
            lower_48,
            hawaii,

            alaska_point,
            hawaii_point,
            lower_48_point,

            alaska_stream,
            hawaii_stream,
            lower_48_stream,
        }
    }
}

impl<SD, T> AlbersUsa<SD, T>
where
    SD: Clone,
    T: CoordFloat + Debug + Default + FloatConst,
{
    /// None standard builder pattern.
    /// No need to specify the DRAIN at this point.
    #[inline]
    #[must_use]
    pub fn builder() -> Builder<SD, T> {
        Builder::new(Self::default())
    }
}

impl<SD, T> Transform for AlbersUsa<SD, T>
where
    SD: Clone,
    T: 'static + CoordFloat + Debug + Default + FloatConst,
{
    type T = T;

    #[inline]
    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        let mut lower_48_point = self.lower_48_point.clone();

        lower_48_point.point(p, None);
        lower_48_point.endpoint().result().map_or_else(
            || {
                let mut alaska_point = self.alaska_point.clone();
                alaska_point.point(p, None);
                alaska_point.endpoint().result().map_or_else(
                    || {
                        let mut hawaii_point = self.hawaii_point.clone();
                        hawaii_point.point(p, None);
                        hawaii_point.endpoint().result().map_or(
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
            self.alaska.build().invert(p)
        } else if self.hawaii_y.contains(&y) && self.hawaii_x.contains(&x) {
            self.hawaii.build().invert(p)
        } else {
            self.lower_48.build().invert(p)
        }
    }
}
