use std::marker::PhantomData;

use geo::Coord;

use crate::multidrain::Multidrain;
use crate::projection::albers::albers;
use crate::projection::albers_usa::AlbersUsa;
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

/// Hardcode type for now until things are generic
impl Multiplex<Unconnected> {
    /// Connects the next stage in the stream pipline.

    #[inline]
    pub fn connect<SD>(&self, sink: Multidrain<SD, f64>) -> AlbersUsaTransformer<SD>
    where
        SD: Clone + Default,
    {
        let mut alaska = EqualArea::<SD, f64>::builder();
        let alaska = alaska.rotate2_set(&[154_f64, 0_f64]);
        let alaska = alaska.center_set(&Coord {
            x: -2_f64,
            y: 58.5_f64,
        });

        let mut hawaii = EqualArea::<SD, f64>::builder();
        let hawaii = hawaii.rotate2_set(&[157_f64, 0_f64]);
        let hawaii = hawaii.center_set(&Coord {
            x: -3_f64,
            y: 19.9_f64,
        });

        let lower_48 = albers::<SD, f64>();

        MultiTransformer::new(
            sink,
            vec![
                alaska.build().stream(&SD::default()),
                lower_48.build().stream(&SD::default()),
                hawaii.build().stream(&SD::default()),
            ],
        )
    }
}

impl<DRAIN> Transform for Multiplex<Connected<DRAIN, AlbersUsaTransformer<DRAIN>>>
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
