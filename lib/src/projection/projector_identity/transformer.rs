use std::marker::PhantomData;

use geo::{CoordFloat, Coordinate};

use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::Unconnected;
use crate::Transform;

// TODO this is very similar to st.rs am I repeating myself?
#[derive(Clone, Debug)]
pub struct Transformer<DRAIN, SC, STATE, T>
where
    T: CoordFloat,
{
    state: STATE,
    /// PhantomData<SC>
    ///
    /// The hidden linkage in Connectable::connect.
    /// Changing the input paramter changes the output
    /// parameter.
    p_sc: PhantomData<SC>,
    p_drain: PhantomData<DRAIN>,
    alpha: T,
    kx: T,
    ky: T,
    ca: T,
    sa: T,
    tx: T,
    ty: T,
}

impl<DRAIN, SC, T> Transformer<DRAIN, SC, Unconnected, T>
where
    T: CoordFloat,
{
    pub(crate) fn new(alpha: T, kx: T, ky: T, ca: T, sa: T, tx: T, ty: T) -> Self {
        Self {
            p_drain: PhantomData::<DRAIN>,
            p_sc: PhantomData::<SC>,
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

impl<DRAIN, SC, T> Connectable for Transformer<DRAIN, SC, Unconnected, T>
where
    SC: Clone,
    T: CoordFloat,
{
    type Output = Transformer<DRAIN, SC, Connected<SC>, T>;
    type SC = SC;
    fn connect(self, sink: Self::SC) -> Self::Output {
        Self::Output {
            state: Connected { sink },
            p_drain: PhantomData::<DRAIN>,
            p_sc: PhantomData::<SC>,
            alpha: self.alpha,
            kx: self.kx,
            ky: self.ky,
            ca: self.ca,
            sa: self.sa,
            tx: self.tx,
            ty: self.ty,
        }
    }
}

impl<DRAIN, SC, STATE, T> Transform for Transformer<DRAIN, SC, STATE, T>
where
    T: CoordFloat,
{
    type T = T;
    fn transform(&self, p: &Coordinate<Self::T>) -> Coordinate<Self::T> {
        let mut x = p.x * self.kx;
        let mut y = p.y * self.ky;
        if !self.alpha.is_zero() {
            let t = y * self.ca + x * self.sa;
            x = x * self.ca + y + self.sa;
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

impl<DRAIN, SC, T> Stream for Transformer<DRAIN, SC, Connected<DRAIN>, T>
where
    DRAIN: Clone + Stream<EP = DRAIN, T = T>,
    T: CoordFloat,
{
    type T = T;
    type EP = DRAIN;
    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.sink.endpoint()
    }

    #[inline]
    fn point(&mut self, p: &Coordinate<Self::T>, m: Option<u8>) {
        self.state.sink.point(&self.transform(p), m)
    }
}
