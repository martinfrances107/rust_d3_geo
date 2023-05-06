use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Range;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::clip::antimeridian::line::Line;
use crate::clip::buffer::Buffer;
use crate::clip::clipper::Clipper;
use crate::clip::rectangle::Rectangle;
use crate::last_point::LastPoint;
use crate::math::EPSILON;
use crate::path::Result;
use crate::projection::Projector;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connected as ConnectedStream;
use crate::stream::Stream;
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
use crate::clip::antimeridian::interpolate::Interpolate;
use crate::clip::antimeridian::pv::PV;
use crate::clip::clipper::Connected as ConnectedClipper;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::stream::Unconnected;

type StreamPoint<T> = StreamTransformRadians<
    ConnectedStream<
        RotatorRadians<
            ConnectedStream<
                Clipper<
                    Interpolate<T>,
                    Line<
                        ConnectedStream<
                            Resample<
                                EqualArea<LastPoint<T>, T>,
                                ConnectedResample<Rectangle<ConnectedStream<LastPoint<T>>, T>, T>,
                                T,
                            >,
                        >,
                        T,
                    >,
                    Line<Unconnected, T>,
                    PV<T>,
                    Resample<
                        EqualArea<LastPoint<T>, T>,
                        ConnectedResample<Rectangle<ConnectedStream<LastPoint<T>>, T>, T>,
                        T,
                    >,
                    ConnectedClipper<
                        Line<ConnectedStream<Buffer<T>>, T>,
                        Line<
                            ConnectedStream<
                                Resample<
                                    EqualArea<LastPoint<T>, T>,
                                    ConnectedResample<
                                        Rectangle<ConnectedStream<LastPoint<T>>, T>,
                                        T,
                                    >,
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
///
/// Why is the Phantom Data is required here...
///
/// The Raw trait is generic ( and the trait way of dealing with generic is to have a interior type )
/// The implementation of Transform is generic and the type MUST be stored in relation to the Struct,
#[derive(Clone, Debug)]
pub struct AlbersUsa<SD, T>
where
    SD: Clone,
    T: CoordFloat + Debug + Default + FloatConst,
{
    k: T,
    t: Coord<T>,

    alaska_x: Range<T>,
    alaska_y: Range<T>,

    hawaii_x: Range<T>,
    hawaii_y: Range<T>,

    // The builder with base setting used as a starting point everytime translate is adjusted.
    pub(super) alaska:
        BuilderConicAntimeridianResampleClip<LastPoint<T>, EqualArea<LastPoint<T>, T>, T>,
    pub(super) lower_48:
        BuilderConicAntimeridianResampleClip<LastPoint<T>, EqualArea<LastPoint<T>, T>, T>,
    pub(super) hawaii:
        BuilderConicAntimeridianResampleClip<LastPoint<T>, EqualArea<LastPoint<T>, T>, T>,

    pub(super) alaska_point: StreamPoint<T>,
    pub(super) lower_48_point: StreamPoint<T>,
    pub(super) hawaii_point: StreamPoint<T>,
    p_sd: PhantomData<SD>,
}

impl<SD, T> Default for AlbersUsa<SD, T>
where
    SD: Clone + Default + Stream<EP = SD, T = T>,
    T: CoordFloat + Debug + Default + FloatConst,
{
    fn default() -> Self {
        let epsilon = T::from(EPSILON).unwrap();

        let mut alaska = EqualArea::builder();
        alaska
            .rotate2_set(&[T::from(154_f64).unwrap(), T::zero()])
            .center_set(&Coord {
                x: T::from(-2_f64).unwrap(),
                y: T::from(58.5_f64).unwrap(),
            })
            .parallels_set(T::from(55_f64).unwrap(), T::from(65_f64).unwrap());

        let mut hawaii = EqualArea::builder();
        hawaii
            .rotate2_set(&[T::from(157_f64).unwrap(), T::zero()])
            .center_set(&Coord {
                x: T::from(-3_f64).unwrap(),
                y: T::from(19.9_f64).unwrap(),
            })
            .parallels_set(T::from(8_f64).unwrap(), T::from(18_f64).unwrap());

        let mut lower_48 = albers();

        // Emulate .scale() call.
        let scaling_factor = T::from(1070).unwrap();
        let lower_48 = lower_48.scale_set(scaling_factor);
        let alaska = alaska.scale_set(T::from(0.35).unwrap() * scaling_factor);
        let hawaii = hawaii.scale_set(scaling_factor);

        // Emulate .translate() call.
        let k: T = lower_48.scale();
        let t = lower_48.translate();

        let mut lower_48 = lower_48.clone();
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

        let alaska = alaska
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
            ]);

        let hawaii = hawaii
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
            ]);

        let lower_48_point = lower_48.build().stream(&LastPoint::default());
        let alaska_point = alaska.build().stream(&LastPoint::default());
        let hawaii_point = hawaii.build().stream(&LastPoint::default());

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
            p_sd: PhantomData::<SD>,
        }
    }
}

impl<SD, T> RawBase for AlbersUsa<SD, T>
where
    SD: Clone + Default + Stream<EP = SD, T = T>,
    T: 'static + CoordFloat + Debug + Default + FloatConst,
{
    type Builder = Builder<SD, T>;

    #[inline]
    fn builder() -> Builder<SD, T> {
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
