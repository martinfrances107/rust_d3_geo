use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use web_sys::CanvasRenderingContext2d;

use crate::clip::Line;
use crate::clip::PointVisible;
use crate::data_object::DataObject;
use crate::path::area_stream::AreaStream as PathArea;
use crate::path::Result;
use crate::path::ResultEnum;
use crate::projection::projection::Projection;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Streamable;

use super::context::Context as PathContext;
use super::context_stream::ContextStream;
use super::string::String as PathString;

// #[derive(Debug)]
// enum StreamObjectType {
//     Feature,
//     FeatureCollection,
// }

// struct Path<DRAIN, L, PR, PV, T>
// where
//     DRAIN: Stream<T = T> + Default,
//     L: Line,
//     PR: ProjectionRaw<T>,
//     PV: PointVisible<T = T>,
//     T: CoordFloat + Display + FloatConst,
// {
//     projection: Projection<DRAIN, L, PR, PV, T>,
// }

impl<L, PR, PV, T> Builder<L, PR, PV, T>
where
    // CS: Stream<T = T>,
    // DRAIN: Stream<T = T> + Default,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    /// Returns the area of object input.
    pub fn area(&self, object: DataObject<T>) -> Option<ResultEnum<T>> {
        let path_area = Rc::new(RefCell::new(ContextStream::A(PathArea::default())));
        let mut projected = self.p.as_ref().unwrap().stream(path_area.clone());
        object.to_stream(&mut projected);
        let out = path_area.borrow_mut().result();

        out
    }
}

/// Path builder.
#[derive(Debug)]
pub struct Builder<L, PR, PV, T>
where
    // CS: Stream<T = T>,
    // DRAIN: Stream<T = T> + Default,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: CoordFloat + Display + FloatConst,
{
    point_radius: T,
    c: Option<Rc<CanvasRenderingContext2d>>,
    c_stream: Option<Rc<RefCell<ContextStream<T>>>>,
    p: Option<Projection<ContextStream<T>, L, PR, PV, T>>,
    p_stream: Option<ContextStream<T>>,
}

impl<L, PR, PV, T> Default for Builder<L, PR, PV, T>
where
    // CS: Stream<T = T>,
    // DRAIN: Stream<T = T> + Default,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: CoordFloat + Display + FloatConst,
{
    fn default() -> Builder<L, PR, PV, T> {
        Self {
            c: None,
            c_stream: None,
            point_radius: T::from(4.5_f64).unwrap(),
            p_stream: None,
            p: None,
        }
    }
}

impl<L, PR, PV, T> Builder<L, PR, PV, T>
where
    // CS: Stream<T = T>,
    // DRAIN: Stream<T = T> + Default,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: CoordFloat + Display + FloatConst,
{
    // fn path(object: DataObject<T>) {
    //     object.
    // }

    // fn area() {
    //     object.to_stream(PathArea::)
    // }
}
/// Context related methods.
impl<L, PR, PV, T> Builder<L, PR, PV, T>
where
    // CS: Stream<T = T>,
    // DRAIN: Stream<T = T> + Default,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: CoordFloat + Display + FloatConst,
{
    /// Returns the state within the builder.
    pub fn get_context(&self) {
        todo!("must implement");
    }

    /// Programe the builder with the context.
    pub fn context(self, context: CanvasRenderingContext2d) -> Builder<L, PR, PV, T> {
        // let context_stream = Some(Rc::new(RefCell::new(PathString::default())));

        let context = Rc::new(context);
        Builder {
            point_radius: self.point_radius,
            c: Some(context.clone()),
            c_stream: Some(Rc::new(RefCell::new(ContextStream::C(
                PathContext::<T>::new(context),
            )))),
            p: self.p,
            p_stream: self.p_stream,
        }
    }

    /// Returns a Builder from default values.
    pub fn context_pathstring(self) -> Builder<L, PR, PV, T> {
        let c_stream = Some(Rc::new(RefCell::new(ContextStream::S(
            PathString::default(),
        ))));

        Builder {
            point_radius: self.point_radius,
            c: None,
            c_stream,
            p: self.p,
            p_stream: self.p_stream,
        }
    }
}

/// Projection related methods.
impl<L, PR, PV, T> Builder<L, PR, PV, T>
where
    // DRAIN: Stream<T = T> + Default,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: CoordFloat + Display + FloatConst,
{
    /// From the progammed state generate a new projection.
    #[inline]
    pub fn build(
        self,
        projection: Projection<ContextStream<T>, L, PR, PV, T>,
    ) -> Builder<L, PR, PV, T> {
        Builder {
            point_radius: self.point_radius,
            c: None,
            c_stream: self.c_stream,
            p: Some(projection),
            p_stream: None,
        }
    }

    /// Reset only the projection section of the bulder.
    #[inline]
    pub fn projection_reset(self) -> Builder<L, PR, PV, T> {
        // let _out = Builder {
        //     point_radius: self.point_radius,
        //     context: self.context,
        //     context_stream: self.context_stream,
        //     projection: None,
        //     projection_stream: None, //TODO ( Identitty in JS |x| x)
        // };
        todo!("must sort out identity output");
    }
}

impl<L, PR, PV, T> Builder<L, PR, PV, T>
where
    // CS: Stream<T = T>,
    // DRAIN: Stream<T = T> + Default,
    L: Line,
    PR: ProjectionRaw<T>,
    PV: PointVisible<T = T>,
    T: CoordFloat + Display + FloatConst,
{
    /// Supply raw projection and conext.
    pub fn init(
        projection: Option<Projection<ContextStream<T>, L, PR, PV, T>>,
        context: Option<CanvasRenderingContext2d>,
    ) -> Builder<L, PR, PV, T> {
        let b = Builder::<L, PR, PV, T>::default();
        let b = match projection {
            Some(projection) => b.build(projection),
            None => b.projection_reset(),
        };
        match context {
            Some(context) => b.context(context),
            None => {
                b.context_pathstring()
                // todo!("must resolve changing output type");
                // b.context_pathstring()
            }
        }
    }
}
