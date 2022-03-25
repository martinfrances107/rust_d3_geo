use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
use crate::clip::antimeridian::line::Line as LineAntimeridian;
use crate::clip::antimeridian::pv::PV as PVAntimeridian;
use crate::clip::buffer::Buffer;
use crate::clip::circle::gen_clip_circle;
use crate::clip::circle::interpolate::Interpolate as InterpolateCircle;
use crate::clip::circle::line::Line as LineCircle;
use crate::clip::circle::pv::PV as PVCircle;
use crate::projection::builder::Clip;
use crate::projection::ClipAngleSet;
use crate::stream::Connected;
use crate::Transform;

use crate::stream::Unconnected;

use super::Builder;

impl<DRAIN, PCNC, PCNU, PR, RC, RU, T> ClipAngleSet
	for Builder<
		DRAIN,
		InterpolateAntimeridian<DRAIN, RC, T>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<DRAIN, RC, Connected<RC>, T>,
		LineAntimeridian<DRAIN, RC, Unconnected, T>,
		PCNC,
		PCNU,
		PR,
		PVAntimeridian<T>,
		RC,
		RU,
		T,
	> where
	DRAIN: Clone + Debug,
	PCNC: Clone + Debug,
	PCNU: Clone + Debug,
	PR: Transform<T = T>,
	RC: Clone + Debug,
	RU: Clone + Debug,
	T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Output = Builder<
		DRAIN,
		InterpolateCircle<DRAIN, RC, T>,
		LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineCircle<DRAIN, RC, Connected<RC>, T>,
		LineCircle<DRAIN, RC, Unconnected, T>,
		PCNC,
		PCNU,
		PR,
		PVCircle<T>,
		RC,
		RU,
		T,
	>;
	type T = T;
	// Set the internal clip angle (theta) to null and return a builder
	// which uses the antimeridian clipping stratergy.
	// fn clip_angle_reset(self) -> Builder<DRAIN, LineAntimeridian<T>, PR, PVAntimeridian<T>, T> {
	//     let preclip_factory: StreamNodeClipFactory<
	//         DRAIN,
	//         LineAntimeridian<T>,
	//         PR,
	//         PVAntimeridian<T>,
	//         DRAIN,
	//         T,
	//     > = gen_clip_factory_antimeridian();
	//     // let preclip_factory = gen_clip_factory_antimeridian();

	//     let out = Builder::new(preclip_factory, self.projection_raw);
	//     // update only theta and preclip_factory.
	//     // let out: Builder<DRAIN, LineAntimeridian<T>, PR, PVAntimeridian<T>, T> = Builder {
	//     //     // projection_raw: self.projection_raw,
	//     //     /// Internal state.
	//     //     // delta_lambda: self.delta_lambda,
	//     //     // delta_phi: self.delta_phi,
	//     //     // delta_gamma: self.delta_gamma,

	//     //     // x: self.x,
	//     //     // y: self.y,

	//     //     // x0: self.x0,
	//     //     // y0: self.y0,
	//     //     // x1: self.x1,
	//     //     // y1: self.y1,
	//     //     // delta2: self.delta2,
	//     //     // lambda: self.lambda,
	//     //     // phi: self.phi,

	//     //     // alpha: self.alpha,
	//     //     // k: self.k,
	//     //     // theta: None,
	//     //     // sx: self.sx,
	//     //     // sy: self.sy,
	//     //     // rotate: self.rotate.clone(),
	//     //     // project_transform: self.project_transform,
	//     //     // project_rotate_transform: self.project_rotate_transform.clone(),
	//     //     // postclip_factory: self.postclip_factory,
	//     //     preclip_factory,
	//     //     // resample_factory: self.resample_factory,

	//     //     // rotate_transform_factory: StreamNodeFactory::new(self.project_rotate_transform),
	//     //     // rotate_factory: StreamNodeFactory::new(self.rotate),
	//     // };
	//     out
	//     // out.reset()
	// }

	// Given an angle in degrees. Sets the internal clip angle and returns a builder
	// which uses the clip circle stratergy.
	fn clip_angle(self, angle: T) -> Self::Output {
		if angle == T::zero() {
			panic!("must call clip_angle_reset() instead");
		}

		let theta = angle.to_radians();
		let clip: Clip<
			DRAIN,
			InterpolateCircle<DRAIN, RC, T>,
			LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
			LineCircle<DRAIN, RC, Connected<RC>, T>,
			LineCircle<DRAIN, RC, Unconnected, T>,
			PR,
			PVCircle<T>,
			RC,
			RU,
			Unconnected,
			T,
		> = gen_clip_circle::<DRAIN, PCNC, PCNU, PR, RC, RU, T>(theta);
		// Copy, Mutate - updating only theta and preclip_factory.
		let out = Self::Output {
			p_lb: PhantomData::<LineCircle<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>>,
			p_pcnc: PhantomData::<PCNC>,
			projection_raw: self.projection_raw,
			clip,
			delta_lambda: self.delta_lambda,
			delta_phi: self.delta_phi,
			delta_gamma: self.delta_gamma,
			x: self.x,
			y: self.y,

			x0: self.x0,
			y0: self.y0,
			x1: self.x1,
			y1: self.y1,

			delta2: self.delta2,
			lambda: self.lambda,
			phi: self.phi,

			alpha: self.alpha,
			k: self.k,

			theta: Some(theta),

			sx: self.sx,
			sy: self.sy,

			rotate: self.rotate,
			project_transform: self.project_transform,
			project_rotate_transform: self.project_rotate_transform,
			postclip: self.postclip,

			resample: self.resample,
			rotator: self.rotator,
		};

		// out.reset()
		out
	}
}
