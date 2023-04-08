use geo::Coord;

use crate::multidrain::Multidrain;
use crate::projection::albers::albers;
use crate::projection::albers_usa::AlbersUsa;
use crate::projection::builder_conic::ParallelsSet;
use crate::projection::equal_area::EqualArea;
use crate::projection::projector_albers_usa::AlbersUsaTransformer;
use crate::projection::Build;
use crate::projection::CenterSet;
use crate::projection::Projector as ProjectoTait;
use crate::projection::RawBase;
use crate::projection::RotateSet;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

use super::multitransformer::MultiTransformer;

/// When connected the state changes to hold the connected Projectors.
#[derive(Debug)]
pub struct Connected<DRAIN, const N: usize, TRANSFORM> {
    pr: AlbersUsa<DRAIN>,
    /// A collections of sub transforms.
    /// TODO can this be simplified once workings.
    pub store: [TRANSFORM; N],
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

/// Hardcode type for now until things are generic
impl Multiplex<Unconnected> {
    /// Connects the next stage in the stream pipline.

    #[inline]
    pub fn connect<const N: usize, SD>(
        &self,
        sink: Multidrain<N, SD, f64>,
    ) -> AlbersUsaTransformer<N, SD, f64>
    where
        SD: Clone + Default,
    {
        let lower_48 = albers::<SD, f64>();

        let mut alaska = EqualArea::<SD, f64>::builder();
        let alaska = alaska
            .rotate2_set(&[154_f64, 0_f64])
            .center_set(&Coord {
                x: -2_f64,
                y: 58.5_f64,
            })
            .parallels_set(55_f64, 65_f64);

        let mut hawaii = EqualArea::<SD, f64>::builder();
        let hawaii = hawaii
            .rotate2_set(&[157_f64, 0_f64])
            .center_set(&Coord {
                x: -3_f64,
                y: 19.9_f64,
            })
            .parallels_set(8_f64, 18_f64);

        // The order of objects in the store is important for performance.
        // The earlier a point is found the better,
        // so the lower_48 is searched first, and the smallest land area last.
        debug_assert_eq!(3usize, N);
        let store = [
            lower_48.build().stream(&sink.drains[0]),
            alaska.build().stream(&sink.drains[1]),
            hawaii.build().stream(&sink.drains[2]),
        ];
        MultiTransformer::new(sink, store)
    }
}

impl<DRAIN, const N: usize> Transform
    for Multiplex<Connected<DRAIN, N, AlbersUsaTransformer<N, DRAIN, f64>>>
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
