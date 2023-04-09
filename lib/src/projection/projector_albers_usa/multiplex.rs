use geo::Coord;

use crate::projection::albers_usa::AlbersUsa;
use crate::projection::projector_albers_usa::AlbersUsaTransformer;
use crate::projection::Build;
use crate::projection::Projector as ProjectoTait;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::multitransformer::MultiTransformer;

/// When connected the state changes to hold the connected Projectors.
#[derive(Debug)]
pub struct Connected<const N: usize, TRANSFORM> {
    /// A collections of sub transforms.
    /// TODO can this be simplified once workings.
    pub store: [TRANSFORM; N],
}

/// A projection stream pipeline stage which holds a collection of
/// Projectors, in the case of `AlbersUSA` one for every region.
/// `lower_48`, `alaaska`, `hawaii`.
#[derive(Clone, Debug)]
pub struct Multiplex<PR, STATE> {
    pr: PR,
    /// The State is Connected or Unconnected.
    /// TODO Once things are working consider simplifying here
    /// by removing this wrapper.
    pub state: STATE,
}

impl<PR> Default for Multiplex<PR, Unconnected>
where
    PR: Default,
{
    fn default() -> Self {
        Self {
            pr: PR::default(),
            state: Unconnected,
        }
    }
}

/// Hardcode type for now until things are generic
impl<PR> Multiplex<PR, Unconnected> {
    /// Connects the next stage in the stream pipline.
    #[inline]
    pub fn connect<SD>(&self, sink: &SD) -> AlbersUsaTransformer<3, SD, f64>
    where
        SD: Clone + Default + Stream<EP = SD, T = f64>,
    {
        let pr = AlbersUsa::<SD>::default();
        // The order of objects in the store is important for performance.
        // The earlier a point is found the better,
        // so the lower_48 is searched first, and the smallest land area last.
        let store = [
            pr.lower_48.build().stream(&sink.clone()),
            pr.alaska.build().stream(&sink.clone()),
            pr.hawaii.build().stream(&sink.clone()),
        ];
        MultiTransformer::new(store)
    }
}

impl<DRAIN, const N: usize> Transform
    for Multiplex<AlbersUsa<DRAIN>, Connected<N, AlbersUsaTransformer<N, DRAIN, f64>>>
where
    DRAIN: Clone + Default + Stream<EP = DRAIN, T = f64>,
{
    /// f32 or f64
    type T = f64;

    fn transform(&self, p: &Coord<f64>) -> Coord<f64> {
        self.pr.transform(p)
    }
    fn invert(&self, p: &Coord<f64>) -> Coord<f64> {
        self.pr.invert(p)
    }
}
