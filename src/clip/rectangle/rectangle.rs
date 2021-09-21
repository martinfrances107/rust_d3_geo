use crate::clip::rejoin::CompareIntersectionsFn;
use crate::clip::InterpolateFn;
use num_traits::Float;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use num_traits::Zero;

use crate::path::Result;
use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;

use crate::clip::buffer::Buffer as ClipBuffer;
use crate::clip::compare_intersection::gen_compare_intersection;
use crate::clip::intersection::Intersection;
use crate::clip::line::line as clip_line;
use crate::clip::line_elem::LineElem;
use crate::clip::rejoin::rejoin as clip_rejoin;
use crate::path::ResultEnum;

#[derive(Clone, Debug)]
pub(crate) struct Rectangle<T>
where
	T: CoordFloat,
{
	buffer_stream: Rc<RefCell<ClipBuffer<T>>>,
	clean: bool,
	clip_min: T,
	clip_max: T,
	epsilon: T,
	first: bool,

	x0: T,
	y0: T,
	x1: T,
	y1: T,

	polygon: Vec<Vec<Coordinate<T>>>,
	segments: Option<Vec<Vec<LineElem<T>>>>,
	ring: Vec<Coordinate<T>>,

	// first point.
	x__: T,
	y__: T,
	v__: bool,
	// previous point.
	x_: T,
	y_: T,
	v_: bool,

	use_line_point: bool,
	use_buffer_stream: bool,
}

impl<T> Rectangle<T>
where
	T: 'static + CoordFloat + FloatConst,
{
	#[inline]
	pub(crate) fn new(x0: T, y0: T, x1: T, y1: T) -> Rectangle<T> {
		Self {
			buffer_stream: Rc::new(RefCell::new(ClipBuffer::<T>::default())),
			first: false,
			clean: false,
			clip_max: T::from(1e9).unwrap(),
			clip_min: T::from(-1e9).unwrap(),
			epsilon: T::from(1e-6).unwrap(),

			x0,
			y0,
			x1,
			y1,

			polygon: Vec::new(),
			segments: None,
			ring: Vec::new(),

			// first point.
			x__: T::zero(),
			y__: T::zero(),
			v__: false,

			// previous point.
			x_: T::zero(),
			y_: T::zero(),
			v_: false,
			use_line_point: false,
			use_buffer_stream: false,
		}
	}

	#[inline]
	fn visible(&self, p: &Coordinate<T>) -> bool {
		self.x0 <= p.x && p.x <= self.x1 && self.y0 <= p.y && p.y <= self.y1
	}

	fn gen_corner(&self) -> Box<dyn Fn(&Coordinate<T>, &T) -> i8> {
		let x0 = self.x0;
		let y0 = self.y0;
		let x1 = self.x1;
		Box::new(move |p: &Coordinate<T>, direction: &T| -> i8 {
			let epsilon = T::from(1e-6).unwrap();
			if (p.x - x0).abs() < epsilon {
				if direction > &T::zero() {
					0
				} else {
					3
				}
			} else if (p.x - x1).abs() < epsilon {
				if direction > &T::zero() {
					2
				} else {
					1
				}
			} else if (p.y - y0).abs() < epsilon {
				if direction > &T::zero() {
					1
				} else {
					0
				}
			} else if direction > &T::zero() {
				3
			} else {
				2
			}
		})
	}

	fn polygon_inside(&self) -> bool {
		let mut winding = 0;

		// TODO I have looked at the JS version .. it seems polygons is only every push an empty array
		// but yet when I console.log is carries data... I can't see where the JS version sets polygons!!!
		for p in self.polygon.iter().take(1) {
			let ring = p;
			let j = 1;
			let mut point = ring[j];
			let mut a0;
			let mut a1;
			let mut b0 = point.x;
			let mut b1 = point.y;
			for r in ring {
				a0 = b0;
				a1 = b1;
				point = *r;
				b0 = point.x;
				b1 = point.y;
				// a0 = b0, a1 = b1, point = ring[j], b0 = point[0], b1 = point[1];

				// if (a1 <= y1) { if (b1 > y1 && (b0 - a0) * (y1 - a1) > (b1 - a1) * (x0 - a0)) ++winding; }
				// else { if (b1 <= y1 && (b0 - a0) * (y1 - a1) < (b1 - a1) * (x0 - a0)) --winding; }

				if a1 <= self.y1 {
					if b1 > self.y1 && (b0 - a0) * (self.y1 - a1) > (b1 - a1) * (self.x0 - a0) {
						winding += 1;
					}
				} else if b1 <= self.y1 && (b0 - a0) * (self.y1 - a1) < (b1 - a1) * (self.x0 - a0) {
					winding -= 1
				}
			}
		}

		!winding.is_zero()
	}

	#[inline]
	fn gen_compare_intersection(&self) -> CompareIntersectionsFn<T> {
		let compare_point = self.gen_compare_point();
		Box::new(
			move |a: &Rc<RefCell<Intersection<T>>>, b: &Rc<RefCell<Intersection<T>>>| {
				compare_point(a.borrow().x.p, b.borrow().x.p)
			},
		)
	}

	/// Warning from JS a, b are LineElem.
	fn gen_compare_point(&self) -> Box<dyn Fn(Coordinate<T>, Coordinate<T>) -> Ordering> {
		let corner = self.gen_corner();
		Box::new(move |a: Coordinate<T>, b: Coordinate<T>| -> Ordering {
			let ca = corner(&a, &T::one());
			let cb = corner(&b, &T::one());
			if ca != cb {
				if (ca - cb) > 0 {
					Ordering::Greater
				} else {
					Ordering::Less
				}
			} else {
				let diff = match ca {
					0 => b.y - a.y,
					1 => a.x - b.x,
					2 => a.y - b.y,
					_ => b.x - a.x,
				};
				if diff > T::zero() {
					Ordering::Greater
				} else if diff < T::zero() {
					Ordering::Less
				} else {
					Ordering::Equal
				}
			}
		})
	}
}

impl<SINK, T> StreamNode<Rectangle<T>, SINK, T>
where
	SINK: Stream<T = T>,
	T: 'static + CoordFloat + FloatConst,
{
	#[inline]
	fn default_point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
		if self.raw.visible(p) {
			if self.raw.use_buffer_stream {
				self.raw.buffer_stream.borrow_mut().point(p, m);
			} else {
				self.sink.borrow_mut().point(p, m);
			}
		}
	}

	fn line_point(&mut self, p_in: &Coordinate<T>, m: Option<u8>) {
		let mut p = *p_in;
		let v = self.raw.visible(&p);

		// JS has if (polygon) {..} here which is never false;
		self.raw.ring.push(p);

		if self.raw.first {
			self.raw.x__ = p.x;
			self.raw.y__ = p.y;
			self.raw.v__ = v;
			self.raw.first = false;
			if v {
				// let as_b = self.raw.active_stream.borrow_mut();
				if self.raw.use_buffer_stream {
					let mut as_b = self.raw.buffer_stream.borrow_mut();
					as_b.line_start();
					as_b.point(&p, None);
				} else {
					let mut sink_b = self.sink.borrow_mut();
					sink_b.line_start();
					sink_b.point(&p, None);
				};
			}
		} else if v && self.raw.v_ {
			if self.raw.use_buffer_stream {
				self.raw.buffer_stream.borrow_mut().point(&p, m);
			} else {
				self.sink.borrow_mut().point(&p, m);
			}
		} else {
			self.raw.x_ = Float::max(
				self.raw.clip_min,
				Float::min(self.raw.clip_max, self.raw.x_),
			);
			self.raw.y_ = Float::max(
				self.raw.clip_min,
				Float::min(self.raw.clip_max, self.raw.y_),
			);
			let a = [self.raw.x_, self.raw.y_];
			p.x = Float::max(self.raw.clip_min, Float::min(self.raw.clip_max, p.x));
			p.y = Float::max(self.raw.clip_min, Float::min(self.raw.clip_max, p.y));
			let b = [p.x, p.y];
			if clip_line(a, b, self.raw.x0, self.raw.y0, self.raw.x1, self.raw.y1) {
				if !self.raw.v_ {
					if self.raw.use_buffer_stream {
						let mut bs_b = self.raw.buffer_stream.borrow_mut();
						bs_b.line_start();
						bs_b.point(&Coordinate { x: a[0], y: a[1] }, None);
					} else {
						let mut sink_b = self.sink.borrow_mut();
						sink_b.line_start();
						sink_b.point(&Coordinate { x: a[0], y: a[1] }, None);
					}
				}
				if self.raw.use_buffer_stream {
					self.raw
						.buffer_stream
						.borrow_mut()
						.point(&Coordinate { x: b[0], y: b[1] }, None);
				} else {
					self.sink
						.borrow_mut()
						.point(&Coordinate { x: b[0], y: b[1] }, None);
				}
				if !v {
					if self.raw.use_buffer_stream {
						self.raw.buffer_stream.borrow_mut().line_end();
					} else {
						self.sink.borrow_mut().line_end();
					}
					self.raw.clean = false;
				}
			} else if v {
				if self.raw.use_buffer_stream {
					let mut as_b = self.raw.buffer_stream.borrow_mut();
					as_b.line_start();
					as_b.point(&p, None);
				} else {
					let mut sink_b = self.sink.borrow_mut();
					sink_b.line_start();
					sink_b.point(&p, None);
				}
				self.raw.clean = false;
			}
		}

		self.raw.x_ = p.x;
		self.raw.y_ = p.y;
		self.raw.v_ = v;
	}

	pub fn gen_interpolate(&self) -> InterpolateFn<SINK, T> {
		// Is capturing here a good thing.
		let x0 = self.raw.x0;
		let y0 = self.raw.y0;
		let x1 = self.raw.x1;
		let y1 = self.raw.y1;

		let compare_point = self.raw.gen_compare_point();
		let corner = self.raw.gen_corner();
		Rc::new(
			move |from: Option<Coordinate<T>>,
			      to: Option<Coordinate<T>>,
			      direction: T,
			      stream: Rc<RefCell<SINK>>| {
				let mut a;
				let a1;
				let direction_i8: i8 = T::to_i8(&direction).unwrap();
				match (to, from) {
					(Some(to), Some(from)) => {
						a = corner(&from, &direction);
						a1 = corner(&to, &direction);
						let mut s_mut = stream.borrow_mut();
						let cp = compare_point(from, to) < Ordering::Less;
						let is_direction = direction > T::zero();
						// logical exor: cp ^^ is_direction
						if a != a1 || (cp && !is_direction) || (!cp && is_direction) {
							loop {
								let p = Coordinate {
									x: if a == 0 || a == 3 { x0 } else { x1 },
									y: if a > 1 { y1 } else { y0 },
								};
								s_mut.point(&p, None);

								a = (a + direction_i8 + 4) % 4;
								if a == a1 {
									break;
								}
							}
						}
					}
					(Some(to), None) => {
						stream.borrow_mut().point(&to, None);
					}
					_ => {
						panic!("did not expect only from and no to .. or Nothing at all Does the JS version get here?");
					}
				}
			},
		)
	}
}

impl<SINK, T> Stream for StreamNode<Rectangle<T>, SINK, T>
where
	SINK: Stream<T = T>,
	T: 'static + CoordFloat + FloatConst,
{
	type T = T;

	#[inline]
	fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
		if self.raw.use_line_point {
			self.line_point(p, m);
		} else {
			self.default_point(p, m);
		}
	}

	// Buffer geometry within a polygon and then clip it en masse.
	fn polygon_start(&mut self) {
		// activeStream = bufferStream,
		self.raw.use_buffer_stream = true;

		self.raw.segments = Some(Vec::new());
		self.raw.polygon = Vec::new();
		self.raw.clean = true;
	}

	fn polygon_end(&mut self) {
		let start_inside = self.raw.polygon_inside();
		let clean_inside = self.raw.clean && start_inside;
		let num_visible_elements = self
			.raw
			.segments
			.clone()
			.unwrap()
			.into_iter()
			.flatten()
			.count();
		let visible = !num_visible_elements.is_zero();

		if clean_inside || visible {
			{
				let mut sb = self.sink.borrow_mut();
				sb.line_start();
			}
			let interpolate: InterpolateFn<SINK, T> = self.gen_interpolate();

			interpolate(None, None, T::one(), self.sink.clone());

			let mut sb = self.sink.borrow_mut();
			sb.line_end();

			let compare_point = self.raw.gen_compare_point();
			let compare_intersection: CompareIntersectionsFn<T> = Box::new(
				move |a: &Rc<RefCell<Intersection<T>>>,
				      b: &Rc<RefCell<Intersection<T>>>|
				      -> Ordering { compare_point(a.borrow().x.p, b.borrow().x.p) },
			);

			if visible {
				clip_rejoin(
					&(self.raw.segments.as_ref().unwrap().clone()),
					compare_intersection,
					start_inside,
					interpolate,
					self.sink.clone(),
				);
			}

			let mut sb = self.sink.borrow_mut();
			sb.polygon_end();
		}
	}

	fn line_start(&mut self) {
		self.raw.use_line_point = true;
		// if self.raw.polygon {
		self.raw.ring = Vec::new();
		self.raw.polygon.push(Vec::new());
		// }
		self.raw.first = true;
		self.raw.v_ = false;
		self.raw.x_ = T::nan();
		self.raw.y_ = T::nan();
	}

	fn line_end(&mut self) {
		if let Some(segments) = &self.raw.segments {
			self.line_point(
				&Coordinate {
					x: self.raw.x__,
					y: self.raw.y__,
				},
				None,
			);
			if self.raw.v__ && self.raw.v_ {
				self.raw.buffer_stream.borrow_mut().rejoin();
			}

			// TODO must uncomments.
			// if let Some(ResultEnum::Path(result)) = self.raw.buffer_stream.borrow_mut().result() {
			// 	segments.push(result);
			// }
		}
		self.raw.use_line_point = false;
		if self.raw.v_ {
			// self.raw.active_stream.borrow_mut().line_end();
			if self.raw.use_buffer_stream {
				self.raw.buffer_stream.borrow_mut().line_end();
			} else {
				self.sink.borrow_mut().line_end();
			}
		}
	}
}
