use std::marker::PhantomData;

use crate::clip::antimeridian::interpolate::Interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::clip::buffer::Buffer;
use crate::clip::clipper::Clipper;
use crate::clip::clipper::Connected as ConnectedClipper;
use crate::identity::Identity;
use crate::multidrain::Multidrain;
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
// use crate::stream::MultiStream;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;
use geo::Coord;

use super::multitransformer::MultiTransformer;
/// When connected the state changes to hold the connected Projectors.
#[derive(Debug)]
pub struct Connected<DRAIN, TRANSFORM> {
    pd_drain: PhantomData<DRAIN>,
    pr: AlbersUsa<DRAIN>,
    /// A collections of sub transforms.
    /// TODO can this be simplified once workings.
    pub store: Vec<TRANSFORM>,
}
/// A projection stream pipeline stage which holds a collection of
/// Projectors, in the case of `AlbersUSA` one for every region.
/// `lower_48`, `alaaska`, `hawaii`.
#[derive(Clone, Debug)]
pub struct Multiplex<STATE> {
    /// The State is Connected or Unconnected.
    /// TODO Once things are working consider simplifying here
    /// by removing this wrapper.
    pub state: STATE,
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

// type Output<SC: Clone> = MultiTransformer<SC, ConnectedStream<SC>, f64, AlbersTransformer<SC>>;

/// Hardcode type for now until things are generic
impl Connectable for Multiplex<Unconnected> {
    /// Connects the next stage in the stream pipline.

    type Output<SC: Clone> = MultiTransformer<
        Multidrain<AlbersTransformer<SC>, f64>,
        ConnectedStream<Multidrain<SC, f64>>,
        f64,
        AlbersTransformer<Multidrain<SC, f64>>,
    >;

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

        todo!();

        // MultiTransformer::new(
        //     sink,
        //     vec![
        //         alaska.build().stream(&sink),
        //         lower_48.build().stream(&sink),
        //         hawaii.build().stream(&sink),
        //     ],
        // )
    }
}

impl<DRAIN, TRANSFORM> Stream for Multiplex<Connected<DRAIN, TRANSFORM>>
where
    TRANSFORM: Stream<EP = DRAIN, T = f64>,
{
    type EP = DRAIN;
    type T = f64;

    fn endpoint(&mut self) -> &mut Self::EP {
        todo!();
    }

    fn line_end(&mut self) {
        for item in &mut self.state.store {
            item.line_end();
        }
    }

    fn line_start(&mut self) {
        for item in &mut self.state.store {
            item.line_start();
        }
    }

    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        for item in &mut self.state.store {
            item.point(p, m);
        }
    }

    fn polygon_end(&mut self) {
        for item in &mut self.state.store {
            item.polygon_end();
        }
    }

    fn polygon_start(&mut self) {
        for item in &mut self.state.store {
            item.polygon_start();
        }
    }

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
