use std::marker::PhantomData;

use crate::clip::clipper;
use crate::stream::Connectable;
use crate::stream::Stream;
use crate::stream::Unconnected;
use geo::Coord;

/// When connected the state changes to hold the connected Projectors.
#[derive(Debug)]
pub struct Connected<P> {
    store: Vec<P>,
}
/// A projection stream pipeline stage which holds a collection of
/// Projectors, in the case of AlbersUSA one for every region.
/// lower_48, alaaska, hawaii.
#[derive(Clone, Debug)]
pub struct Multiplex<DRAIN, PCON, PUNCON, STATE> {
    phantom_drain: PhantomData<DRAIN>,
    phantom_pcon: PhantomData<PCON>,
    state: STATE,
    unconnected_store: Vec<PUNCON>,
}

impl<DRAIN, PCON, PUNCON> Multiplex<DRAIN, PCON, PUNCON, Unconnected> {
    const fn new(unconnected_store: Vec<PUNCON>) -> Self {
        Self {
            phantom_drain: PhantomData::<DRAIN>,
            phantom_pcon: PhantomData::<PCON>,

            state: Unconnected,
            unconnected_store,
        }
    }
}

impl<DRAIN, PCON, PUNCON: clipper::Connectable> Connectable
    for Multiplex<DRAIN, PCON, PUNCON, Unconnected>
where
    DRAIN: Clone,
    PCON: Clone,
{
    type Output<SC: Clone> = Multiplex<DRAIN, PCON, PUNCON, Connected<PCON>>;

    /// Connects the next stage in the stream pipline.
    #[inline]
    fn connect<SC: Clone>(&self, sink: SC) -> Self::Output<SC>
    where
        SC: Clone,
    {
        todo!();
        // let store: Vec<PCON> = self
        //     .unconnected_store
        //     .iter()
        //     .map(|elem| elem.connect(sink))
        //     .collect();

        // Multiplex {
        //     phantom_drain: self.phantom_drain,
        //     phantom_pcon: self.phantom_pcon,
        //     state: Connected { store },
        //     unconnected_store: self.unconnected_store,
        // }
    }
}

impl<DRAIN, PCON, PUNCON> Stream for Multiplex<DRAIN, PCON, PUNCON, Connected<PCON>>
where
    DRAIN: Clone + PartialEq,
    PCON: Stream<EP = DRAIN, T = f64>,
    // P: Stream<EP = DRAIN, T = T>,
    // T: CoordFloat,
{
    type EP = DRAIN;
    type T = f64;
    /// Returns the end point of the stream.
    fn endpoint(&mut self) -> &mut Self::EP {
        todo!();
        // self.store
        //     .first()
        //     .expect("Cannot supply an empty list of Projectors.")
        //     .endpoint()
    }

    /// Declare the end of a line segment.
    fn line_end(&mut self) {
        for item in self.state.store.iter_mut() {
            item.line_end();
        }
    }

    /// Declare the start of a line segment.
    fn line_start(&mut self) {
        for item in self.state.store.iter_mut() {
            item.line_start();
        }
    }

    /// Declare a point.
    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        for item in self.state.store.iter_mut() {
            item.point(p, m);
        }
    }

    /// Declare the end of a polygon.
    fn polygon_end(&mut self) {
        for item in self.state.store.iter_mut() {
            item.polygon_end();
        }
    }
    /// Declare the start of a polygon.
    fn polygon_start(&mut self) {
        for item in self.state.store.iter_mut() {
            item.polygon_start();
        }
    }
    /// Declare a sphere object.
    fn sphere(&mut self) {
        for item in self.state.store.iter_mut() {
            item.sphere();
        }
    }
}
