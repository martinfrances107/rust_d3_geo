use crate::Transform;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use web_sys::CanvasRenderingContext2d;

// use crate::clip::buffer::Buffer;
// use crate::clip::Bufferable;
// use crate::clip::Interpolator;
// use crate::clip::LineConnected;
// use crate::clip::LineUnconnected;
// use crate::clip::PointVisible;
use crate::path::context::Context;
// use crate::projection::builder::PostClipNode;
use crate::projection::projector::Projector;
// use crate::projection::resampler::Resampler;
use crate::projection::ProjectionRawBase;
// use crate::stream::Connectable;
// use crate::stream::Stream;

use super::context::Context as PathContext;
use super::string::String;
use super::Path;
// use super::Path;
use super::PointRadiusTrait;
// use super::Result;

/// Path builder.
#[derive(Debug, Clone)]
pub struct Builder<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    // CS: Stream<EP = CS, T = T> + Default,
    // RC: Resampler + Stream<EP = CS, T = T>,
    // LC: LineConnected<SC = RC> + Stream<EP = CS, T = T>,
    // LU: LineUnconnected<SU = RU>,
    // I: Interpolator<EP = CS, Stream = RC, T = T>,
    // PCNC: PostClipNode + Stream<EP = CS, T = T>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = CS>,
    // RU: Resampler + Connectable<Output = RC, SC = PCNC>,
    I: Clone,
    LC: Clone,
    LU: Clone,
    CS: Clone,
    LB: Clone,
    RC: Clone,
    PR: Clone + Transform<T = T>,
    RU: Clone,
    PCNU: Clone,
    PV: Clone,
    T: CoordFloat + FloatConst,
{
    pr: T,
    p_i: PhantomData<I>,

    p_lb: PhantomData<LB>,
    p_lc: PhantomData<LC>,
    p_lu: PhantomData<LU>,
    p_pcnc: PhantomData<PCNC>,
    p_pcnu: PhantomData<PCNU>,
    p_pv: PhantomData<PV>,
    p_pr: PhantomData<PR>,
    p_rc: PhantomData<RC>,
    p_ru: PhantomData<RU>,
    // p_t: PhantomData<T>,
    context_stream: CS,
    projection: Option<Projector<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>>,
}

impl<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Builder<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    // CS: Stream<EP = CS, T = T> + Default,

    // I: Interpolator<EP = CS, Stream = RC, T = T>,
    // LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    // LC: LineConnected<SC = RC> + Stream<EP = CS, T = T>,
    // LU: LineUnconnected<SU = RU>
    //     + Bufferable<Output = LB, T = T>
    //     + Connectable<Output = LC, SC = RC>,
    // PCNC: PostClipNode + Stream<EP = CS, T = T>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = CS>,
    // RC: Resampler + Stream<EP = CS, T = T>,
    // RU: Resampler + Connectable<Output = RC, SC = PCNC>,
    I: Clone,
    LC: Clone,
    LU: Clone,
    CS: Clone,
    LB: Clone,
    RC: Clone,
    PR: Clone + Transform<T = T>,
    RU: Clone,
    PCNU: Clone,
    PV: Clone,
    T: AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
{
    /// Constructor.
    pub fn new(context_stream: CS) -> Builder<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> {
        Self {
            context_stream,
            p_i: PhantomData::<I>,
            p_lb: PhantomData::<LB>,
            p_lc: PhantomData::<LC>,
            p_lu: PhantomData::<LU>,
            p_pcnc: PhantomData::<PCNC>,
            p_pcnu: PhantomData::<PCNU>,
            p_pv: PhantomData::<PV>,
            p_pr: PhantomData::<PR>,
            p_rc: PhantomData::<RC>,
            p_ru: PhantomData::<RU>,
            pr: T::from(4.5_f64).unwrap(),
            projection: None,
        }
    }
}

/// Context related methods.
impl<'a, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Builder<Context<'a, T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    // I: Interpolator<EP = Context<'a, T>, Stream = RC, T = T>,
    // LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    // LC: LineConnected<SC = RC> + Stream<EP = Context<'a, T>, T = T>,
    // LU: LineUnconnected<SU = RU>
    //     + Bufferable<Output = LB, T = T>
    //     + Connectable<Output = LC, SC = RC>,
    // PCNC: PostClipNode + Stream<EP = Context<'a, T>, T = T>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = Context<'a, T>>,
    // RC: Resampler + Stream<EP = Context<'a, T>, T = T>,
    // RU: Resampler + Connectable<Output = RC, SC = PCNC>,
    // PR: ProjectionRawBase<T>,
    I: Clone,
    LC: Clone,
    LU: Clone,
    LB: Clone,
    RC: Clone,
    PR: Clone + Transform<T = T>,
    RU: Clone,
    PCNU: Clone,
    PV: Clone,
    LB: Clone,
    PR: Clone + Transform<T = T>,
    // PV: PointVisible<T = T>,
    T: AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
{
    /// Returns the state within the builder.
    // pub fn get_context(&self) {
    //     todo!("must implement");
    // }

    /// Programe the builder with the context.
    pub fn context(
        self,
        context: &'a CanvasRenderingContext2d,
    ) -> Builder<Context<'a, T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> {
        Builder {
            pr: self.pr,
            p_i: PhantomData::<I>,
            p_lb: PhantomData::<LB>,
            p_lc: PhantomData::<LC>,
            p_lu: PhantomData::<LU>,
            p_pcnc: PhantomData::<PCNC>,
            p_pcnu: PhantomData::<PCNU>,
            p_pv: PhantomData::<PV>,
            p_pr: PhantomData::<PR>,
            p_rc: PhantomData::<RC>,
            p_ru: PhantomData::<RU>,
            context_stream: PathContext::<T>::new(context),
            projection: self.projection,
        }
    }
}

/// Context related methods.
impl<'a, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Builder<String<T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    // CS: Stream<EP = CS, T = T> + PointRadiusTrait<T = T> + Default,
    // I: Interpolator<EP = String<T>, Stream = RC, T = T>,
    // LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    // LC: LineConnected<SC = RC> + Stream<EP = String<T>, T = T>,
    // LU: LineUnconnected<SU = RU>
    //     + Bufferable<Output = LB, T = T>
    //     + Connectable<Output = LC, SC = RC>,
    // PCNC: PostClipNode + Stream<EP = String<T>, T = T>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = String<T>>,
    // RC: Resampler + Stream<EP = String<T>, T = T>,
    // RU: Resampler + Connectable<Output = RC, SC = PCNC>,
    I: Clone,
    LC: Clone,
    LU: Clone,
    LB: Clone,
    RC: Clone,
    PR: Clone + Transform<T = T>,
    RU: Clone,
    PCNU: Clone,
    PV: Clone,
    LB: Clone,
    PR: Clone + Transform<T = T>,
    T: AddAssign<T> + AbsDiffEq<Epsilon = T> + CoordFloat + Display + FloatConst,
{
    /// Returns a Builder from default values.
    pub fn context_pathstring() -> Builder<String<T>, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    {
        let context_stream = String::default();

        Builder::new(context_stream)
    }
}

impl<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> PointRadiusTrait
    for Builder<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    CS: PointRadiusTrait<T = T>,
    // CS: Stream<EP = CS, T = T> + PointRadiusTrait<T = T> + Result + PartialEq + Default,
    // I: Interpolator<EP = CS, Stream = RC, T = T>,
    // LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    // LC: LineConnected<SC = RC> + Stream<EP = CS, T = T>,
    // LU: LineUnconnected<SU = RU>
    //     + Bufferable<Output = LB, T = T>
    //     + Connectable<Output = LC, SC = RC>,
    // PCNC: PostClipNode + Stream<EP = CS, T = T>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = CS>,
    // PR: ProjectionRawBase<T>,
    // PV: PointVisible<T = T>,
    // RC: Resampler + Stream<EP = CS, T = T>,
    // RU: Resampler + Connectable<Output = RC, SC = PCNC>,
    I: Clone,
    LC: Clone,
    LU: Clone,
    CS: Clone,
    LB: Clone,
    RC: Clone,
    PR: Clone + Transform<T = T>,
    RU: Clone,
    PCNU: Clone,
    PV: Clone,
    LB: Clone,
    PR: Clone + Transform<T = T>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// f64 or f32.
    type T = T;

    /// From the progammed state generate a new projection.
    #[inline]
    fn point_radius(&mut self, radius: T) {
        self.pr = radius;
        self.context_stream.point_radius(self.pr);
    }
}

/// Projection related methods.
impl<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Builder<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    // CS: Stream<EP = CS, T = T> + Result + PartialEq + Default,

    // I: Interpolator<EP = CS, Stream = RC, T = T>,
    // LB: LineConnected<SC = Buffer<T>> + Stream<EP = Buffer<T>, T = T>,
    // LC: LineConnected<SC = RC> + Stream<EP = CS, T = T>,
    // LU: LineUnconnected<SU = RU>
    //     + Bufferable<Output = LB, T = T>
    //     + Connectable<Output = LC, SC = RC>,

    // PCNC: PostClipNode + Stream<EP = CS, T = T>,
    // PCNU: PostClipNode + Connectable<Output = PCNC, SC = CS>,
    // PR: ProjectionRawBase<T>,
    // PV: PointVisible<T = T>,
    I: Clone,
    LC: Clone,
    LU: Clone,
    CS: Clone,
    LB: Clone,
    RC: Clone,
    PR: Clone + Transform<T = T>,
    RU: Clone,
    PCNU: Clone,
    PV: Clone,
    LB: Clone,
    PR: Clone + Transform<T = T>,
    // RC: Resampler + Stream<EP = CS, T = T>,
    // RU: Resampler + Connectable<Output = RC, SC = PCNC>,
    T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// From the progammed state generate a new projection.
    #[inline]
    pub fn build(
        self,
        projection: Projector<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>,
    ) -> Path<CS, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    where
        PR: ProjectionRawBase<T>,
    {
        Path::new(self.context_stream, projection)
    }
}
