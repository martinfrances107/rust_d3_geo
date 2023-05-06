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

use super::AlbersTransformer;

/// Only when the Multidrain is connected the sub drain becomes known
/// as so the SUBTRANS type can be defined.
#[derive(Clone, Debug)]
pub struct Populated<const N: usize, SUBTRANS> {
    store: [SUBTRANS; N],
}

/// The state before connection and the drain is populated
#[derive(Debug)]
pub struct Unpopulated;

/// Wrapper for a Drain type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Multidrain<const N: usize, SD, STATE> {
    /// After initialisation, this value is used in a .connect()
    /// call to construct the drains.
    pub sd: SD,
    state: STATE,
}

impl<const N: usize, SD> Multidrain<N, SD, Unpopulated> {
    /// Define initial multidrain
    /// population() which be called when connectied into a pipeline
    /// this will change the state.
    pub const fn new(sd: SD) -> Self {
        Self {
            sd,
            state: Unpopulated,
        }
    }
}

impl<const N: usize, SD> Multidrain<N, SD, Unpopulated>
where
    SD: Clone + Default,
{
    /// Constructor.
    #[must_use]
    pub fn populate<SUBTRANS>(
        &self,
        store: [SUBTRANS; N],
    ) -> Multidrain<N, SD, Populated<N, SUBTRANS>> {
        Multidrain {
            sd: self.sd.clone(),
            state: Populated { store },
        }
    }
}

type A<const N: usize, T> =
    Multidrain<N, PathString<T>, Populated<N, AlbersTransformer<PathString<T>, T>>>;

impl Result for A<3, f64> {
    type Out = Vec<String>;

    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        let mut out = vec![];
        // let mut drain = self.state.store[0].clone();
        // out.push(drain.endpoint().result());

        for c in &mut self.state.store {
            let results = c.endpoint().result();

            out.push(results);
        }
        out
    }
}

impl<const N: usize, SUBTRANS, T> Result for Multidrain<N, LastPoint<f64>, Populated<N, SUBTRANS>>
where
    SUBTRANS: Stream<EP = LastPoint<T>, T = T>,
    T: CoordFloat,
{
    type Out = Option<Coord<T>>;
    /// Merges the results of all the sub-drains.
    fn result(&mut self) -> Self::Out {
        for d in &mut self.state.store {
            if let Some(p) = d.endpoint().result() {
                return Some(p);
            }
        }
        None
    }
}

impl<const N: usize, SD, SUBTRANS, T> Stream for Multidrain<N, SD, Populated<N, SUBTRANS>>
where
    SUBTRANS: Stream<EP = SD, T = T>,
    T: CoordFloat,
{
    type EP = Self;
    type T = T;

    fn endpoint(&mut self) -> &mut Self::EP {
        self
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
