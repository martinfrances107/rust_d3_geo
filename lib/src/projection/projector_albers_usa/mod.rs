use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coord;

use crate::clip::antimeridian::interpolate::Interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::clip::buffer::Buffer;
use crate::clip::clipper::Clipper;
use crate::clip::clipper::Connected as ConnectedClipper;
use crate::identity::Identity;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connected as ConnectedStream;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use self::multiplex::Multiplex;
use self::multitransformer::MultiTransformer;
use super::equal_area::EqualArea;
use super::resampler::resample::Resample;
use super::stream_transform_radians::StreamTransformRadians;
use super::Projector as ProjectorTrait;

/// The multiplex is a collection of sub-projections.
pub mod multiplex;
/// Transformer defined as more than more proejction.
pub mod multitransformer;

/// Used in the formation of a `AlbersUsa` pipeline.
pub type AlbersUsaTransformer<const N: usize, SD, T> = MultiTransformer<
    3,
    SD,
    StreamTransformRadians<
        ConnectedStream<
            RotatorRadians<
                ConnectedStream<
                    Clipper<
                        Interpolate<T>,
                        Line<
                            ConnectedStream<
                                Resample<
                                    EqualArea<SD, T>,
                                    ConnectedResample<Identity<ConnectedStream<SD>>, T>,
                                    T,
                                >,
                            >,
                            T,
                        >,
                        Line<Unconnected, T>,
                        PV<T>,
                        Resample<
                            EqualArea<SD, T>,
                            ConnectedResample<Identity<ConnectedStream<SD>>, T>,
                            T,
                        >,
                        ConnectedClipper<
                            Line<ConnectedStream<Buffer<T>>, T>,
                            Line<
                                ConnectedStream<
                                    Resample<
                                        EqualArea<SD, T>,
                                        ConnectedResample<Identity<ConnectedStream<SD>>, T>,
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
    >,
>;

/// Projection output of projection/Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<DRAIN, MULTIPLEX> {
    phantom_drain: PhantomData<DRAIN>,
    /// The internal single stage of the pipeline.
    pub multiplex: MULTIPLEX,
}

impl<DRAIN, MULTIPLEX> Default for Projector<DRAIN, MULTIPLEX>
where
    DRAIN: Clone,
    MULTIPLEX: Default,
{
    fn default() -> Self {
        Self {
            phantom_drain: PhantomData::<DRAIN>,
            multiplex: MULTIPLEX::default(),
        }
    }
}

impl<DRAIN, PR> ProjectorTrait for Projector<DRAIN, Multiplex<PR, Unconnected>>
where
    PR: Default,
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    type EP = DRAIN;

    type Transformer = AlbersUsaTransformer<3, DRAIN, f64>;

    /// Connects a DRAIN to the `AlbersUSA` projector.
    ///
    /// The Projection Stream Pipeline :-
    ///
    /// Multiplex -> DRAIN
    ///
    fn stream(&mut self, drain: &Self::EP) -> Self::Transformer {
        self.multiplex.connect(drain)
    }
}

impl<DRAIN, MULTIPLEX> Transform for Projector<DRAIN, MULTIPLEX>
where
    DRAIN: Clone,
    MULTIPLEX: Transform<T = f64>,
{
    /// f32 or f64
    type T = f64;

    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
        self.multiplex.transform(p)
    }
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        self.multiplex.invert(p)
    }
}
