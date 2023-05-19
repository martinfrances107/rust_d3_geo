use geo::CoordFloat;
use geo_types::Coord;

use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

/// A Stream node, that applies a complex transform to each point.
#[derive(Clone, Debug)]
pub struct Transformer<STATE, T>
where
    T: CoordFloat,
{
    alpha: T,
    kx: T,
    ky: T,
    ca: T,
    sa: T,
    tx: T,
    state: STATE,
    ty: T,
}

impl<T> Transformer<Unconnected, T>
where
    T: CoordFloat,
{
    #[inline]
    pub(crate) const fn new(alpha: T, kx: T, ky: T, ca: T, sa: T, tx: T, ty: T) -> Self {
        Self {
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

impl<T> Connectable for Transformer<Unconnected, T>
where
    T: CoordFloat,
{
    type Output<SC> = Transformer<Connected<SC>, T>;
    fn connect<SC>(&self, sink: SC) -> Transformer<Connected<SC>, T> {
        Self::Output {
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

impl<STATE, T> Transform for Transformer<STATE, T>
where
    T: CoordFloat,
{
    type T = T;
    fn transform(&self, p: &Coord<Self::T>) -> Coord<Self::T> {
        let mut x = p.x * self.kx;
        let mut y = p.y * self.ky;
        if !self.alpha.is_zero() {
            let t = y * self.ca - x * self.sa;
            x = x * self.ca + y * self.sa;
            y = t;
        }
        Coord {
            x: x + self.tx,
            y: y + self.ty,
        }
    }

    fn invert(&self, p: &Coord<Self::T>) -> Coord<Self::T> {
        let mut x = p.x - self.tx;
        let mut y = p.y - self.ty;

        if !self.alpha.is_zero() {
            let t = y * self.ca + x * self.sa;
            x = x * self.ca - y * self.sa;
            y = t;
        }
        Coord {
            x: x / self.kx,
            y: y / self.ky,
        }
    }
}

impl<DRAIN, SINK, T> Stream for Transformer<Connected<SINK>, T>
where
    SINK: Stream<EP = DRAIN, T = T>,
    T: CoordFloat,
{
    type EP = SINK::EP;
    type T = T;
    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.sink.endpoint()
    }

    fn line_end(&mut self) {
        self.state.sink.line_end();
    }

    fn line_start(&mut self) {
        self.state.sink.line_start();
    }

    #[inline]
    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        self.state.sink.point(&self.transform(p), m);
    }

    fn polygon_end(&mut self) {
        self.state.sink.polygon_end();
    }

    fn polygon_start(&mut self) {
        self.state.sink.polygon_start();
    }

    fn sphere(&mut self) {
        self.state.sink.sphere();
    }
}
