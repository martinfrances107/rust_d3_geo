use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::rc::Rc;

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::clip::Line;
use crate::clip::PointVisible;
use crate::data_object::DataObject;
use crate::path::area_stream::AreaStream;
use crate::path::bounds_stream::BoundsStream;
use crate::path::context_stream::ContextStream;
use crate::path::Result;
use crate::projection::projection::Projection;
use crate::projection::Raw as ProjectionRaw;
use crate::stream::Streamable;

use super::PointRadiusEnum;
use super::ResultEnum;

/// Projection and context stream applied to a DataObject.
#[derive(Debug)]
pub struct Path<L, PR, PV, T>
where
	PR: ProjectionRaw<T>,
	L: Line,
	T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
	PV: PointVisible<T = T>,
{
	context_stream: Rc<RefCell<ContextStream<T>>>,
	point_radius: PointRadiusEnum<T>,
	/// don't store projection stream.
	projection: Projection<ContextStream<T>, L, PR, PV, T>,
}

impl<L, PR, PV, T> Path<L, PR, PV, T>
where
	L: Line,
	PR: ProjectionRaw<T>,
	T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
	PV: PointVisible<T = T>,
{
	/// Constructor.
	pub fn new(
		context_stream: Rc<RefCell<ContextStream<T>>>,
		projection: Projection<ContextStream<T>, L, PR, PV, T>,
	) -> Self {
		Self {
			context_stream,
			point_radius: PointRadiusEnum::Val(T::from(4.5_f64).unwrap()),
			projection,
		}
	}

	/// Combines projection, context stream and object.
	pub fn object(&mut self, object: DataObject<T>) -> Option<ResultEnum<T>> {
		let mut stream_in = self.projection.stream(self.context_stream.clone());
		object.to_stream(&mut stream_in);
		self.context_stream.borrow_mut().result()
	}

	#[inline]
	/// Returns the area of the Path
	/// This operation consumes the  Path.
	pub fn area(self, object: &DataObject<T>) -> Option<ResultEnum<T>>
	where
		T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
	{
		let stream_dst = Rc::new(RefCell::new(ContextStream::A(AreaStream::default())));
		let mut stream_in = self.projection.stream(stream_dst.clone());
		object.to_stream(&mut stream_in);

		let x = stream_dst.borrow_mut().result();
		x
	}

	/// Returns the area of the Path
	/// This operation consumes the  Path.
	pub fn bounds(self, object: &DataObject<T>) -> Option<ResultEnum<T>>
	where
		T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
	{
		let stream_dst = Rc::new(RefCell::new(ContextStream::B(BoundsStream::default())));
		let mut stream_in = self.projection.stream(stream_dst.clone());
		object.to_stream(&mut stream_in);

		let x = stream_dst.borrow_mut().result();
		x
	}

	/// Sets the context stream.
	pub fn context(mut self, context_stream: Rc<RefCell<ContextStream<T>>>) -> Self
	where T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst{
		self.context_stream = context_stream;
		self
	}

	// 	fn projection_fn(
	// 		mut self,
	// 		projection: Option<Projection<ContextStream<T>, Line<T>, PR, PV<T>, T>>,
	// 	) -> Self
	// // where
	// 	    //     <P as ProjectionTrait<'a>>::SD: Stream<T=T>,
	// 	{
	// 		match projection {
	// 			None => {
	// 				self.projection = None;
	// 				// self.projection_stream = Identity; // A stream that is pass through?
	// 				// Self::default()
	// 				todo!();
	// 			}
	// 			Some(projection) => {
	// 				self.projection = Some(projection);
	// 				// Warm the cache before storage.
	// 				// self.projection_stream = Some(projection.stream());
	// 				self
	// 			}
	// 		}
	// 	}

	//     // pub fn projection(p_in: Option<ProjectionMutator<PR, T>>) -> Path<T>
	//     // where
	//     //     T: CoordFloat + FloatConst,
	//     // {
	//     //     let projection: Option<ProjectionMutator<PR, T>>;
	//     //     let projection_stream: Box<
	//     //         dyn Fn(Box<dyn Stream>) -> StreamTransformRadians<T>,
	//     //     >;

	//     //     Path {
	//     //         ..Default::default()
	//     //     }
	//     // }

	//     // #[inline]
	//     // fn get_context(&self) -> Option<Box<dyn PointRadiusTrait<PrtT=T>>> {
	//     //     self.context_stream.as_ref().unwrap()
	//     // }

	// fn context_fn(mut self, c_in: Option<Rc<CanvasRenderingContext2d>>) -> Self {
	// 	match c_in {
	// 		None => {
	// 			self.context = None;
	// 			self.context_stream = Rc::new(RefCell::new(ContextStream::default()));
	// 		}
	// 		Some(c) => {
	// 			self.context = Some(c.clone());
	// 			self.context_stream = Rc::new(RefCell::new(ContextStream::C(PathContext::new(c))));
	// 		}
	// 	}
	// 	match &self.point_radius {
	// 		PointRadiusEnum::F(_pr) => {
	// 			// do nothing.
	// 		}
	// 		PointRadiusEnum::Val(pr) => {
	// 			self.context_stream.borrow_mut().point_radius(Some(*pr));
	// 		}
	// 	}
	// 	self
	// }

	// #[inline]
	// fn get_point_radius(&self) -> PointRadiusEnum<T> {
	//     self.point_radius
	// }

	#[inline]
	fn point_radius(mut self, input: PointRadiusEnum<T>) -> Self {
		self.point_radius = match input {
			PointRadiusEnum::F(ref _input_fn) => input,
			PointRadiusEnum::Val(_input_value) => {
				// match &mut self.context_stream {
				// 	PathContextStream::S(s) => {
				// 		s.point_radius(Some(input_value));
				// 	}
				// 	PathContextStream::C(c) => {
				// 		c.point_radius(Some(input_value));
				// 	}
				// }
				// self.context_stream.point_radius(Some(input_value));
				input
			}
		};
		self
	}
}
