use num_traits::Float;
use std::cell::RefCell;
use std::rc::Rc;

use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use num_traits::Zero;

use crate::path::Result;
use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;

use crate::clip::buffer::Buffer as ClipBuffer;
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
	T: CoordFloat + FloatConst,
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

	fn corner(&self, p: &Coordinate<T>, direction: &i8) -> i8 {
		let epsilon = T::from(1e-6).unwrap();
		if (p.x - self.x0).abs() < epsilon {
			if direction > &0 {
				0
			} else {
				3
			}
		} else if (p.x - self.x1).abs() < epsilon {
			if direction > &0 {
				2
			} else {
				1
			}
		} else if (p.y - self.y0).abs() < epsilon {
			if direction > &0 {
				1
			} else {
				0
			}
		} else if direction > &0 {
			3
		} else {
			2
		}
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
	fn compare_intersection(self, a: Intersection<T>, b: Intersection<T>) -> T {
		self.compare_point(a.x.p, b.x.p)
	}

	/// Warning from JS a, b are LineElem.
	fn compare_point(&self, a: Coordinate<T>, b: Coordinate<T>) -> T {
		let ca = self.corner(&a, &1);
		let cb = self.corner(&b, &1);
		if ca != cb {
			T::from(ca - cb).unwrap()
		} else {
			match ca {
				0 => b.y - a.y,
				1 => a.x - b.x,
				2 => a.y - b.y,
				_ => b.x - a.x,
			}
		}
	}
}

impl<SINK, T> StreamNode<Rectangle<T>, SINK, T>
where
	SINK: Stream<T = T>,
	T: CoordFloat + FloatConst,
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
		} else {
			if v && self.raw.v_ {
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
		}
		self.raw.x_ = p.x;
		self.raw.y_ = p.y;
		self.raw.v_ = v;
	}

	fn interpolate(
		self,
		from: Option<Coordinate<T>>,
		to: Option<Coordinate<T>>,
		direction: i8,
		stream: Rc<RefCell<SINK>>,
	) {
		let mut a;
		let a1;

		match (to, from) {
			(Some(to), Some(from)) => {
				a = self.raw.corner(&from, &direction);
				a1 = self.raw.corner(&to, &direction);
				let mut s_mut = stream.borrow_mut();
				let cp = self.raw.compare_point(from, to) < T::zero();
				let is_direction = direction > 0;
				// logical exor: cp ^^ is_direction
				if a != a1 || (cp && !is_direction) || (!cp && is_direction) {
					loop {
						let p = Coordinate {
							x: if a == 0 || a == 3 {
								self.raw.x0
							} else {
								self.raw.x1
							},
							y: if a > 1 { self.raw.y1 } else { self.raw.y0 },
						};
						s_mut.point(&p, None);

						a = (a + direction + 4) % 4;
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
	}
}

impl<SINK, T> Stream for StreamNode<Rectangle<T>, SINK, T>
where
	SINK: Stream<T = T>,
	T: CoordFloat + FloatConst,
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
		// let segments = merge(self.segments).length();
		// let visible = self
		// 	.raw
		// 	.segments
		// 	.unwrap()
		// 	.into_iter()
		// 	.flatten()
		// 	.collect::<LineElem<T>>()
		// 	.len();
		// resolve this bypass.
		let visible = true;

		//TODO resolve clip_rejoin fn types.
		// if clean_inside || visible {
		// 	let sb = self.sink.borrow_mut();
		// 	sb.line_start();
		// 	self.interpolate(None, None, 1, self.sink.clone());
		// 	sb.line_end();

		// 	if visible {
		// 		clip_rejoin(
		// 			&self.raw.segments.unwrap(),
		// 			self.raw.compare_intersection,
		// 			start_inside,
		// 			self.interpolate,
		// 			self.sink.clone(),
		// 		);
		// 	}
		// 	sb.polygon_end();
		// }
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
