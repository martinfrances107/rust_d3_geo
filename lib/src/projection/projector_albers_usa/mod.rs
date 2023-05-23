use std::fmt::Debug;

use geo::Coord;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::clip::buffer::Buffer;
use crate::clip::clipper::Clipper;
use crate::clip::clipper::Connected as ConnectedClipper;
use crate::clip::rectangle::Rectangle;
use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::Build;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connected as ConnectedStream;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use self::multidrain::Multidrain;
use self::multidrain::Populated;
use self::multidrain::Unpopulated;
use super::albers_usa::AlbersUsa;
use super::equal_area::EqualArea;
use super::resampler::resample::Resample;
use super::stream_transform_radians::StreamTransformRadians;
use super::Projector as ProjectorTrait;

/// End point for projections like `AlbersUsa` which end in mulitple points.
pub mod multidrain;

type AlbersTransformer<SD, T> = StreamTransformRadians<
    ConnectedStream<
        RotatorRadians<
            ConnectedStream<
                Clipper<
                    Interpolate<T>,
                    Line<Unconnected, T>,
                    PV<T>,
                    Resample<
                        EqualArea<T>,
                        ConnectedResample<Rectangle<ConnectedStream<SD>, T>, T>,
                        T,
                    >,
                    ConnectedClipper<
                        Line<ConnectedStream<Buffer<T>>, T>,
                        Line<
                            ConnectedStream<
                                Resample<
                                    EqualArea<T>,
                                    ConnectedResample<Rectangle<ConnectedStream<SD>, T>, T>,
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

/// Projection output of projection/Builder.
///
/// Commnon functionality for all raw projection structs.
#[derive(Clone, Debug)]
pub struct Projector<SD, T>
where
    SD: Clone,
    T: CoordFloat + Default + FloatConst,
{
    /// The internal single stage of the path.
    pub pr: AlbersUsa<SD, T>,
}

impl<SD, T> Default for Projector<SD, T>
where
    SD: Clone + PartialEq + Stream<EP = SD, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    fn default() -> Self {
        Self {
            pr: AlbersUsa::<SD, T>::default(),
        }
    }
}

impl<SD, T> ProjectorTrait for Projector<SD, T>
where
    SD: Clone + PartialEq + Stream<EP = SD, T = T>,
    T: CoordFloat + Default + FloatConst,
{
    type EP = Multidrain<3, SD, Unpopulated>;

    type Transformer = Multidrain<3, SD, Populated<3, AlbersTransformer<SD, T>>>;

    /// Connects a DRAIN to the `AlbersUSA` projector.
    ///
    /// The Projection Stream Path :-
    ///
    /// Multiplex -> DRAIN
    ///
    fn stream(&mut self, drain: &Self::EP) -> Self::Transformer {
        let pr = AlbersUsa::default();
        let sd = &drain.sd;

        // The order of objects in the store is important for performance.
        // The earlier a point is found the better,
        // so the lower_48 is searched first, and the smallest land area last.
        let store = [
            pr.lower_48_stream.build().stream(sd),
            pr.alaska_stream.build().stream(sd),
            pr.hawaii_stream.build().stream(sd),
        ];

        let md = Multidrain::new(drain.sd.clone());
        md.populate(store)
    }
}

impl<SD, T> Transform for Projector<SD, T>
where
    SD: Clone,
    T: 'static + CoordFloat + Default + FloatConst,
{
    /// f32 or f64
    type T = T;

    fn transform(&self, p: &Coord<T>) -> Coord<T> {
        self.pr.transform(p)
    }
    fn invert(&self, p: &Coord<T>) -> Coord<T> {
        self.pr.invert(p)
    }
}
