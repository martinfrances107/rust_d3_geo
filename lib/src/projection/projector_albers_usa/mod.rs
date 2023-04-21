use std::fmt::Debug;
use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::clip::buffer::Buffer;
use crate::clip::clipper::Clipper;
use crate::clip::clipper::Connected as ConnectedClipper;
use crate::identity::Identity;
use crate::projection::projector_albers_usa::multiplex::Connected as ConnectedMultiplex;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connected as ConnectedStream;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use self::multiplex::Multiplex;
use self::multitransformer::MultiTransformer;
use super::albers_usa::AlbersUsa;
use super::equal_area::EqualArea;
use super::resampler::resample::Resample;
use super::stream_transform_radians::StreamTransformRadians;
use super::Projector as ProjectorTrait;

/// The multiplex is a collection of sub-projections.
pub mod multiplex;

pub mod multidrain;
/// Transformer defined as more than more proejction.
pub mod multitransformer;

type AlbersTransformer<SD, T> = StreamTransformRadians<
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
>;

/// Used in the formation of a `AlbersUsa` pipeline.
pub type AlbersUsaMultiTransformer<SD, T> = MultiTransformer<3, SD, T, AlbersTransformer<SD, T>>;
pub type AlbersUsaMultiplex<SD, T> =
    Multiplex<AlbersUsa<SD, T>, ConnectedMultiplex<3, AlbersTransformer<SD, T>>, T>;

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

impl<DRAIN, PR, T> ProjectorTrait for Projector<DRAIN, Multiplex<PR, Unconnected, T>>
where
    PR: Default,
    T: CoordFloat + Default + FloatConst,
    DRAIN: Clone + Default + PartialEq + Stream<EP = DRAIN, T = T>,
{
    type EP = DRAIN;

    type Transformer = AlbersUsaMultiTransformer<DRAIN, T>;

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

impl<DRAIN, MULTIPLEX, T> Transform for Projector<DRAIN, MULTIPLEX>
where
    T: CoordFloat + Debug,
    MULTIPLEX: Transform<T = T>,
{
    /// f32 or f64
    type T = T;

    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        self.multiplex.transform(p)
    }
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        self.multiplex.invert(p)
    }
}
