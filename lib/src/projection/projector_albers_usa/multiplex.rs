use std::marker::PhantomData;

use crate::clip::antimeridian::interpolate::Interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::clip::buffer::Buffer;
use crate::clip::clipper::Clipper;
use crate::clip::clipper::Connected as ConnectedClipper;
use crate::identity::Identity;
use crate::projection::albers::albers;
use crate::projection::albers_usa::AlbersUsa;
use crate::projection::equal_area::EqualArea;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::projection::Build;
use crate::projection::CenterSet;
use crate::projection::Projector as ProjectoTait;
use crate::projection::RawBase;
use crate::projection::RotateSet;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connectable;
use crate::stream::Connected as ConnectedStream;
use crate::stream::MultiStream;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;
use geo::Coord;

/// When connected the state changes to hold the connected Projectors.
#[derive(Debug)]
pub struct Connected<DRAIN, TRANSFORM> {
    pd_drain: PhantomData<DRAIN>,
    pr: AlbersUsa<DRAIN>,
    store: Vec<TRANSFORM>,
}
/// A projection stream pipeline stage which holds a collection of
/// Projectors, in the case of `AlbersUSA` one for every region.
/// `lower_48`, `alaaska`, `hawaii`.
#[derive(Clone, Debug)]
pub struct Multiplex<STATE> {
    state: STATE,
}

impl Default for Multiplex<Unconnected> {
    fn default() -> Self {
        Self { state: Unconnected }
    }
}

/// Sub type of `AlbersMultiplexType`
pub type AlbersTransformer<SC> = StreamTransformRadians<
    ConnectedStream<
        RotatorRadians<
            ConnectedStream<
                Clipper<
                    Interpolate<f64>,
                    Line<
                        ConnectedStream<
                            Resample<
                                EqualArea<SC, f64>,
                                ConnectedResample<Identity<ConnectedStream<SC>>, f64>,
                                f64,
                            >,
                        >,
                        f64,
                    >,
                    Line<Unconnected, f64>,
                    PV<f64>,
                    Resample<
                        EqualArea<SC, f64>,
                        ConnectedResample<Identity<ConnectedStream<SC>>, f64>,
                        f64,
                    >,
                    ConnectedClipper<
                        Line<ConnectedStream<Buffer<f64>>, f64>,
                        Line<
                            ConnectedStream<
                                Resample<
                                    EqualArea<SC, f64>,
                                    ConnectedResample<Identity<ConnectedStream<SC>>, f64>,
                                    f64,
                                >,
                            >,
                            f64,
                        >,
                        f64,
                    >,
                    f64,
                >,
            >,
            f64,
        >,
    >,
>;

/// Hardcode type for now until things are generic
pub type AlbersMultiplexType<SC> = Multiplex<Connected<SC, AlbersTransformer<SC>>>;

impl Connectable for Multiplex<Unconnected> {
    /// Connects the next stage in the stream pipline.

    type Output<SC: Clone> = Vec<AlbersTransformer<SC>>;
    #[inline]
    fn connect<SC>(&self, sink: SC) -> Self::Output<SC>
    where
        SC: Clone,
    {
        let mut alaska = EqualArea::<SC, f64>::builder();
        let alaska = alaska.rotate2_set(&[154_f64, 0_f64]);
        let alaska = alaska.center_set(&Coord {
            x: -2_f64,
            y: 58.5_f64,
        });

        let mut hawaii = EqualArea::<SC, f64>::builder();
        let hawaii = hawaii.rotate2_set(&[157_f64, 0_f64]);
        let hawaii = hawaii.center_set(&Coord {
            x: -3_f64,
            y: 19.9_f64,
        });

        let lower_48 = albers::<SC, f64>();

        let out = vec![
            alaska.build().stream(&sink),
            lower_48.build().stream(&sink),
            hawaii.build().stream(&sink),
        ];

        out
    }
}

impl<DRAIN, TRANSFORM> MultiStream for Multiplex<Connected<DRAIN, TRANSFORM>>
where
    TRANSFORM: Stream<EP = DRAIN, T = f64>,
{
    type EP = Vec<DRAIN>;
    type T = f64;
    /// Returns the end point of the stream.
    fn endpoints(&mut self) -> &mut Vec<Self::EP> {
        todo!();
        // self.store
        //     .first()
        //     .expect("Cannot supply an empty list of Projectors.")
        //     .endpoint()
    }

    /// Declare the end of a line segment.
    fn line_end(&mut self) {
        for item in &mut self.state.store {
            item.line_end();
        }
    }

    /// Declare the start of a line segment.
    fn line_start(&mut self) {
        for item in &mut self.state.store {
            item.line_start();
        }
    }

    /// Declare a point.
    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        for item in &mut self.state.store {
            item.point(p, m);
        }
    }

    /// Declare the end of a polygon.
    fn polygon_end(&mut self) {
        for item in &mut self.state.store {
            item.polygon_end();
        }
    }
    /// Declare the start of a polygon.
    fn polygon_start(&mut self) {
        for item in &mut self.state.store {
            item.polygon_start();
        }
    }
    /// Declare a sphere object.
    fn sphere(&mut self) {
        for item in &mut self.state.store {
            item.sphere();
        }
    }
}

impl<DRAIN> Transform for Multiplex<Connected<DRAIN, AlbersTransformer<DRAIN>>>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    /// f32 or f64
    type T = f64;

    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
        self.state.pr.transform(p)
    }
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        self.state.pr.invert(p)
    }
}
