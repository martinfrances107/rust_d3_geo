use std::marker::PhantomData;

use geo::CoordFloat;
use geo::Coordinate;

use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

/// A Stream node, that applies a complex transform to each point.
#[derive(Clone, Debug)]
pub struct Transformer<DRAIN, STATE, T>
where
    T: CoordFloat,
{
    p_drain: PhantomData<DRAIN>,
    alpha: T,
    kx: T,
    ky: T,
    ca: T,
    sa: T,
    tx: T,
    state: STATE,
    ty: T,
}

impl<DRAIN, T> Transformer<DRAIN, Unconnected, T>
where
    T: CoordFloat,
{
    #[inline]
    pub(crate) const fn new(alpha: T, kx: T, ky: T, ca: T, sa: T, tx: T, ty: T) -> Self {
        Self {
            p_drain: PhantomData::<DRAIN>,
            alpha,
            kx,
            ky,
            ca,
            sa,
            tx,
            ty,
            state: Unconnected,
        }
    }
}

impl<DRAIN, T> Connectable for Transformer<DRAIN, Unconnected, T>
where
    T: CoordFloat,
{
    type Output<SC: Clone> = Transformer<DRAIN, Connected<SC>, T>;
    fn connect<SC>(self, sink: SC) -> Transformer<DRAIN, Connected<SC>, T>
    where
        SC: Clone,
    {
        Self::Output {
            p_drain: PhantomData::<DRAIN>,
            alpha: self.alpha,
            kx: self.kx,
            ky: self.ky,
            ca: self.ca,
            sa: self.sa,
            tx: self.tx,
            ty: self.ty,
            state: Connected { sink },
        }
    }
}

impl<DRAIN, STATE, T> Transform for Transformer<DRAIN, STATE, T>
where
    T: CoordFloat,
{
    type T = T;
    fn transform(&self, p: &Coordinate<Self::T>) -> Coordinate<Self::T> {
        let mut x = p.x * self.kx;
        let mut y = p.y * self.ky;
        if !self.alpha.is_zero() {
            let t = y * self.ca - x * self.sa;
            x = x * self.ca + y * self.sa;
            y = t;
        }
        Coordinate {
            x: x + self.tx,
            y: y + self.ty,
        }
    }

    fn invert(&self, p: &Coordinate<Self::T>) -> Coordinate<Self::T> {
        let mut x = p.x - self.tx;
        let mut y = p.y - self.ty;

        if !self.alpha.is_zero() {
            let t = y * self.ca + x * self.sa;
            x = x * self.ca - y * self.sa;
            y = t;
        }
        Coordinate {
            x: x / self.kx,
            y: y / self.ky,
        }
    }
}

impl<DRAIN, SINK, T> Stream for Transformer<DRAIN, Connected<SINK>, T>
where
    DRAIN: Clone + Stream<EP = DRAIN, T = T>,
    SINK: Clone + Stream<EP = DRAIN, T = T>,
    T: CoordFloat,
{
    type EP = DRAIN;
    type T = T;
    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.sink.endpoint()
    }

    /// Declare the end of a line segment.
    fn line_end(&mut self) {
        self.state.sink.line_end();
    }
    // Declare the start of a line segment.
    fn line_start(&mut self) {
        self.state.sink.line_start();
    }
    /// Declare a point.
    #[inline]
    fn point(&mut self, p: &Coordinate<Self::T>, m: Option<u8>) {
        self.state.sink.point(&self.transform(p), m);
    }

    /// Declare the end of a polygon.
    fn polygon_end(&mut self) {
        self.state.sink.polygon_end();
    }
    /// Declare the start of a polygon.
    fn polygon_start(&mut self) {
        self.state.sink.polygon_start();
    }
    /// Declare a sphere object.
    fn sphere(&mut self) {
        self.state.sink.sphere();
    }
}
