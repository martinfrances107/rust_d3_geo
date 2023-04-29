use std::marker::PhantomData;

use geo::Coord;
use geo::CoordFloat;

use crate::clip::antimeridian::interpolate::Interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::antimeridian::pv::PV;
use crate::clip::buffer::Buffer;
use crate::clip::clipper::Clipper;
use crate::clip::clipper::Connected as ConnectedClipper;
use crate::identity::Identity;
use crate::last_point::LastPoint;
use crate::path::string::String as PathString;
use crate::path::Result;
use crate::projection::equal_area::EqualArea;

use crate::projection::resampler::resample::Connected as ConnectedResample;
use crate::projection::resampler::resample::Resample;
use crate::projection::stream_transform_radians::StreamTransformRadians;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connected as ConnectedStream;
use crate::stream::Stream;
use crate::stream::Unconnected;

/// Only when the Multidrain is connected the sub drain becomes known
/// as so the SUBTRANS type can be defined.
#[derive(Clone, Debug)]
pub struct Populated<SUBTRANS> {
    drains: Vec<SUBTRANS>,
}

/// The state before connection and the drain is populated
#[derive(Debug)]
pub struct Unpopulated;

/// Wrapper for a Drain type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Multidrain<const N: usize, SD, STATE, T> {
    p_t: PhantomData<T>,
    /// After initialisation, this value is used in a .connect()
    /// call to construct the drains.
    pub sd: SD,
    // drains: Vec<SUBTRANS>,
    state: STATE,
}

impl<const N: usize, SD, T> Multidrain<N, SD, Unpopulated, T> {
    /// Define initial multidrain
    /// population() which be called when connectied into a pipeline
    /// this will change the state.
    pub const fn new(sd: SD) -> Self {
        Self {
            p_t: PhantomData::<T>,
            sd,
            state: Unpopulated,
        }
    }
}

impl<const N: usize, SD, T> Multidrain<N, SD, Unpopulated, T>
where
    SD: Clone + Default,
{
    /// Constructor.
    #[must_use]
    pub fn populate<SUBTRANS>(
        &self,
        drains: Vec<SUBTRANS>,
    ) -> Multidrain<N, SD, Populated<SUBTRANS>, T> {
        Multidrain {
            p_t: PhantomData::<T>,
            sd: self.sd.clone(),
            state: Populated { drains },
        }
    }
}

type A = Multidrain<
    3,
    PathString<f64>,
    Populated<
        StreamTransformRadians<
            ConnectedStream<
                RotatorRadians<
                    ConnectedStream<
                        Clipper<
                            Interpolate<f64>,
                            Line<
                                ConnectedStream<
                                    Resample<
                                        EqualArea<PathString<f64>, f64>,
                                        ConnectedResample<
                                            Identity<ConnectedStream<PathString<f64>>>,
                                            f64,
                                        >,
                                        f64,
                                    >,
                                >,
                                f64,
                            >,
                            Line<Unconnected, f64>,
                            PV<f64>,
                            Resample<
                                EqualArea<PathString<f64>, f64>,
                                ConnectedResample<Identity<ConnectedStream<PathString<f64>>>, f64>,
                                f64,
                            >,
                            ConnectedClipper<
                                Line<ConnectedStream<Buffer<f64>>, f64>,
                                Line<
                                    ConnectedStream<
                                        Resample<
                                            EqualArea<PathString<f64>, f64>,
                                            ConnectedResample<
                                                Identity<ConnectedStream<PathString<f64>>>,
                                                f64,
                                            >,
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
    >,
    f64,
>;

impl Result for A {
    type Out = Vec<String>;

    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        let mut out = vec![];
        for c in &mut self.state.drains {
            let results = c.endpoint().result();

            out.push(results);
        }
        out
    }
}

impl<const N: usize, SUBTRANS, T> Result for Multidrain<N, LastPoint<f64>, Populated<SUBTRANS>, T>
where
    T: CoordFloat,
    SUBTRANS: Stream<EP = LastPoint<T>, T = T>,
{
    type Out = Option<Coord<T>>;
    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        for d in &mut self.state.drains {
            if let Some(p) = d.endpoint().result() {
                return Some(p);
            }
        }
        None
    }
}

impl<const N: usize, SD, SUBTRANS, T> Stream for Multidrain<N, SD, Populated<SUBTRANS>, T>
where
    SUBTRANS: Stream<EP = SD, T = T>,
    T: CoordFloat,
{
    type T = T;
    type EP = Self;

    fn endpoint(&mut self) -> &mut Self::EP {
        self
    }

    fn line_end(&mut self) {
        for item in &mut self.state.drains {
            item.line_end();
        }
    }

    fn line_start(&mut self) {
        for item in &mut self.state.drains {
            item.line_start();
        }
    }

    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        for item in &mut self.state.drains {
            item.point(p, m);
        }
    }

    fn polygon_end(&mut self) {
        for item in &mut self.state.drains {
            item.polygon_end();
        }
    }

    fn polygon_start(&mut self) {
        for item in &mut self.state.drains {
            item.polygon_start();
        }
    }

    fn sphere(&mut self) {
        for item in &mut self.state.drains {
            item.sphere();
        }
    }
}
