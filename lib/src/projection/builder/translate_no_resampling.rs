use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::projection::builder::ResampleNoneClipC;
use crate::projection::builder::ResampleNoneClipU;
use crate::projection::builder::ResampleNoneNoClipC;
use crate::projection::builder::ResampleNoneNoClipU;
use crate::projection::RecenterNoResampling;
use crate::projection::Translate;
use crate::stream::Stream;
use crate::Transform;

use super::template::ClipU;
use super::Builder;
use super::NoClipU;

impl<DRAIN, I, LC, LB, LU, PR, PV, T> Translate
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		NoClipU<DRAIN>,
		PR,
		PV,
		ResampleNoneNoClipC<DRAIN, PR, T>,
		ResampleNoneNoClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T>,
	Self: RecenterNoResampling,
	T: CoordFloat + FloatConst,
{
	type T = T;

	fn translate(mut self, t: &Coordinate<T>) -> Self {
		self.x = t.x;
		self.y = t.y;
		self.recenter_no_resampling()
	}
}

impl<DRAIN, I, LC, LB, LU, PR, PV, T> Translate
	for Builder<
		DRAIN,
		I,
		LB,
		LC,
		LU,
		ClipU<DRAIN, T>,
		PR,
		PV,
		ResampleNoneClipC<DRAIN, PR, T>,
		ResampleNoneClipU<DRAIN, PR, T>,
		T,
	> where
	DRAIN: Stream<EP = DRAIN, T = T>,
	PR: Clone + Transform<T = T>,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type T = T;

	fn translate(mut self, t: &Coordinate<T>) -> Self {
		self.x = t.x;
		self.y = t.y;
		self.recenter_no_resampling()
	}
}
