use approx::AbsDiffEq;
use geo::CoordFloat;
use num_traits::FloatConst;

use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleClip;
use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleNoneClip;
use crate::projection::builder_mercator::types::BuilderMercatorAntimeridianResampleNoneNoClip;
use crate::projection::builder_mercator::types::BuilderMercatorCircleResampleClip;
use crate::projection::builder_mercator::types::BuilderMercatorCircleResampleNoClip;
use crate::projection::builder_mercator::types::BuilderMercatorCircleResampleNoneClip;
use crate::projection::builder_mercator::types::BuilderMercatorCircleResampleNoneNoClip;
use crate::projection::builder_mercator::BuilderMercatorAntimeridianResampleNoClip;
use crate::projection::PrecisionBypass;
use crate::stream::Stream;

impl<DRAIN, PR, T> PrecisionBypass for BuilderMercatorAntimeridianResampleNoClip<DRAIN, PR, T>
where
	DRAIN: Clone + Stream<EP = DRAIN, T = T>,
	PR: Clone,
	T: AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
	type Output = BuilderMercatorAntimeridianResampleNoneNoClip<DRAIN, PR, T>;
	type T = T;

	fn precision_bypass(self) -> Self::Output {
		let base = self.base.precision_bypass();
		Self::Output {
			extent: self.extent, // post-clip extent
			pr: self.pr,
			base,
		}
	}
}

impl<DRAIN, PR, T> PrecisionBypass for BuilderMercatorAntimeridianResampleClip<DRAIN, PR, T>
where
	DRAIN: Clone + Stream<EP = DRAIN, T = T>,
	PR: Clone,
	T: CoordFloat + FloatConst,
{
	type Output = BuilderMercatorAntimeridianResampleNoneClip<DRAIN, PR, T>;
	type T = T;

	fn precision_bypass(self) -> Self::Output {
		let base = self.base.precision_bypass();
		Self::Output {
			extent: self.extent, // post-clip extent
			pr: self.pr,
			base,
		}
	}
}

impl<DRAIN, PR, T> PrecisionBypass for BuilderMercatorCircleResampleNoClip<DRAIN, PR, T>
where
	DRAIN: Clone + Stream<EP = DRAIN, T = T>,
	PR: Clone,
	T: CoordFloat + FloatConst,
{
	type Output = BuilderMercatorCircleResampleNoneNoClip<DRAIN, PR, T>;
	type T = T;

	fn precision_bypass(self) -> Self::Output {
		let base = self.base.precision_bypass();
		Self::Output {
			extent: self.extent, // post-clip extent
			pr: self.pr,
			base,
		}
	}
}

impl<DRAIN, PR, T> PrecisionBypass for BuilderMercatorCircleResampleClip<DRAIN, PR, T>
where
	DRAIN: Clone + Stream<EP = DRAIN, T = T>,
	PR: Clone,
	T: CoordFloat + FloatConst,
{
	type Output = BuilderMercatorCircleResampleNoneClip<DRAIN, PR, T>;
	type T = T;

	fn precision_bypass(self) -> Self::Output {
		let base = self.base.precision_bypass();
		Self::Output {
			extent: self.extent, // post-clip extent
			pr: self.pr,
			base,
		}
	}
}
