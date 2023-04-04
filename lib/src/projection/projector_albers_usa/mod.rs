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
use crate::multidrain::Multidrain;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connected as ConnectedStream;
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
/// Builder shorthand notations.
pub mod types;

pub type AlbersUsaTransformer<SD> = MultiTransformer<
    Multidrain<SD, f64>,
    ConnectedStream<Multidrain<SD, f64>>,
    f64,
    StreamTransformRadians<
        ConnectedStream<
            RotatorRadians<
                ConnectedStream<
                    Clipper<
                        Interpolate<f64>,
                        Line<
                            ConnectedStream<
                                Resample<
                                    EqualArea<SD, f64>,
                                    ConnectedResample<Identity<ConnectedStream<SD>>, f64>,
                                    f64,
                                >,
                            >,
                            f64,
                        >,
                        Line<Unconnected, f64>,
                        PV<f64>,
                        Resample<
                            EqualArea<SD, f64>,
                            ConnectedResample<Identity<ConnectedStream<SD>>, f64>,
                            f64,
                        >,
                        ConnectedClipper<
                            Line<ConnectedStream<Buffer<f64>>, f64>,
                            Line<
                                ConnectedStream<
                                    Resample<
                                        EqualArea<SD, f64>,
                                        ConnectedResample<Identity<ConnectedStream<SD>>, f64>,
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
    >,
>;

/// Projection output of projection/Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<DRAIN, MULTIPLEX> {
    phantom_drain: PhantomData<DRAIN>,
    /// The internals stages of the pipeline
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

impl<SD> ProjectorTrait for Projector<Multidrain<SD, f64>, Multiplex<Unconnected>>
where
    SD: Clone + Default,
{
    type EP = Multidrain<SD, f64>;

    type Transformer = AlbersUsaTransformer<SD>;

    /// Connects a DRAIN to the `AlbersUSA` projector.
    ///
    /// The Projection Stream Pipeline :-
    ///
    /// Multiplex -> DRAIN
    ///
    fn stream(&mut self, drain: &Self::EP) -> Self::Transformer {
        self.multiplex.connect::<SD>(drain.clone())
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
