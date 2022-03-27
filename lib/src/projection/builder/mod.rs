use std::fmt::Debug;
use std::marker::PhantomData;

use approx::AbsDiffEq;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

use crate::clip::antimeridian::interpolate::Interpolate;
use crate::clip::antimeridian::line::Line;
use crate::clip::buffer::Buffer;
use crate::clip::clip::Clip;
use crate::compose::Compose;
use crate::identity::Identity;
use crate::rot::rotate_radians;
use crate::rot::rotate_radians::RotateRadians;
use crate::rot::rotator_radians::RotatorRadians;
use crate::stream::Connected;
use crate::stream::Unconnected;
use crate::Transform;

use super::builder::template::NoClipC;
use super::builder::template::NoClipU;
use super::builder::template::ResampleNoClipC;
use super::builder::template::ResampleNoClipU;
use super::projector::Projector;
use super::resampler::none::None as ResampleNone;
use super::resampler::resample::Connected as ConnectedResample;
use super::resampler::resample::Resample;
use super::stream_transform_radians::StreamTransformRadians;
use super::transform::generate as generate_str;
use super::transform::scale_translate_rotate::ScaleTranslateRotate;
use super::Angle;

mod angle;
mod angle_mercator;
mod center;
mod clip_angle_adjust;
mod clip_angle_get;
mod clip_angle_reset;
mod clip_angle_set;
mod clip_bounded;
mod clip_extent_set;
mod fit;
mod fit_adjust;
mod precision_adjust;
mod precision_bypass;
mod precision_get;
mod precision_set;
mod reflect;
mod reflect_mercator;
mod rotate;
mod scale;
pub mod template;
mod translate;

/// Marker trait for structs Identity or Rectangle
pub trait PostClipNode {}

/// Projection builder.
///
/// Holds State related to the construction of the a projection.
#[derive(Clone, Debug)]
pub struct Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    T: CoordFloat + FloatConst,
{
    p_pcnc: PhantomData<PCNC>,
    p_lb: PhantomData<LB>,
    pub projection_raw: PR,
    pub clip: Clip<DRAIN, I, LB, LC, LU, PR, PV, RC, RU, Unconnected, T>,
    pub lambda: T,
    pub phi: T,
    pub alpha: T, // post-rotate angle
    pub k: T,     // scale
    pub sx: T,    // reflectX
    pub sy: T,    // reflectY

    pub x: T,
    pub y: T, // translate

    pub delta_lambda: T,
    pub delta_phi: T,
    pub delta_gamma: T,

    pub delta2: T, // precision

    pub theta: Option<T>,

    x0: Option<T>,
    y0: Option<T>,
    x1: Option<T>,
    y1: Option<T>, // post-clip extent

    /// Used by recenter() to build the factories.
    pub rotate: RotateRadians<T>,
    pub rotator: RotatorRadians<Unconnected, T>, //rotate, pre-rotate
    pub project_transform: Compose<T, PR, ScaleTranslateRotate<T>>,
    /// Used by rotate_transform_factory and projections transform.
    pub project_rotate_transform:
        Compose<T, RotateRadians<T>, Compose<T, PR, ScaleTranslateRotate<T>>>,

    /// Projection pipeline stage.
    pub postclip: PCNU,

    /// Projection pipeline stage
    pub resample: RU,
}

impl<DRAIN, PR, PV, T>
    Builder<
        DRAIN,
        Interpolate<DRAIN, ResampleNoClipC<DRAIN, PR, T>, T>,
        Line<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
        Line<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Connected<ResampleNoClipC<DRAIN, PR, T>>, T>,
        Line<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
        NoClipC<DRAIN, T>,
        NoClipU<DRAIN, T>,
        PR,
        PV,
        ResampleNoClipC<DRAIN, PR, T>,
        ResampleNoClipU<DRAIN, PR, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// Given a Raw Projection and a clipping defintion create the associated
    /// Projection builder.
    pub fn new(
        clip: Clip<
            DRAIN,
            Interpolate<DRAIN, ResampleNoClipC<DRAIN, PR, T>, T>,
            Line<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
            Line<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Connected<ResampleNoClipC<DRAIN, PR, T>>, T>,
            Line<DRAIN, ResampleNoClipC<DRAIN, PR, T>, Unconnected, T>,
            PR,
            PV,
            ResampleNoClipC<DRAIN, PR, T>,
            ResampleNoClipU<DRAIN, PR, T>,
            Unconnected,
            T,
        >,
        projection_raw: PR,
    ) -> Self {
        let x = T::from(480_f64).unwrap();
        let y = T::from(250_f64).unwrap();
        let lambda = T::zero();
        let phi = T::zero();
        let alpha = T::zero();
        let k = T::from(150_f64).unwrap();
        let sx = T::one();
        let sy = T::one();
        let delta_lambda = T::zero();
        let delta_phi = T::zero();
        let delta_gamma = T::zero();
        let delta2 = T::from(0.5_f64).unwrap();
        let center = generate_str(&k, &T::zero(), &T::zero(), &sx, &sy, &alpha)
            .transform(&projection_raw.transform(&Coordinate { x: lambda, y: phi }));
        let str = generate_str(&k, &(x - center.x), &(y - center.y), &sx, &sy, &alpha);

        let rotate = rotate_radians([delta_lambda, delta_phi, delta_gamma]); // pre-rotate
        let rotator = RotatorRadians::new(rotate.clone());
        let project_transform = Compose::new(projection_raw.clone(), str);
        let project_rotate_transform = Compose::new(rotate.clone(), project_transform.clone());
        let postclip = Identity::default();
        let resample = Resample::new(project_transform.clone(), delta2);
        let p_pcnc = PhantomData::<Identity<DRAIN, DRAIN, DRAIN, Connected<DRAIN>, T>>;
        let out_a: Self = Self {
            clip,
            p_pcnc,
            p_lb: PhantomData::<Line<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>>,
            /// Input passing onto Projection.
            projection_raw,

            /// Internal state.
            delta_lambda,
            delta_phi,
            delta_gamma,
            x,
            y,

            x0: None,
            y0: None,
            x1: None,
            y1: None, //postclip = identity, // post-clip extent

            delta2: T::from(0.5_f64).unwrap(),
            lambda,
            phi,

            alpha,
            k,
            theta: None,
            sx,
            sy,

            rotate,
            rotator,
            project_transform,
            project_rotate_transform,
            postclip,
            resample,
        };

        let out_b: Self = out_a.recenter_with_resampling();
        out_b
    }
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
    Builder<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T>
where
    DRAIN: Clone,
    I: Clone,
    LB: Clone,
    LC: Clone,
    LU: Clone,
    PCNU: Clone,
    PR: Clone,
    PV: Clone,
    RC: Clone,
    RU: Clone,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    /// Using the currently programmed state output a new projection.
    #[inline]
    pub fn build(&self) -> Projector<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, RC, RU, T> {
        Projector {
            p_lb: PhantomData::<LB>,
            p_lc: PhantomData::<LC>,
            p_pcnc: PhantomData::<PCNC>,
            cache: None,
            postclip: self.postclip.clone(),
            clip: self.clip.clone(),
            resample: self.resample.clone(),
            rotator: self.rotator.clone(),
            project_rotate_transform: self.project_rotate_transform.clone(),
            transform_radians: StreamTransformRadians(Unconnected),
        }
    }

    // fn reset(self) -> Self {
    //     // self.cache_stream = None;
    //     // self.cache = None;
    //     self
    // }
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T>
    Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        PCNC,
        PCNU,
        PR,
        PV,
        Resample<DRAIN, PR, PCNC, PCNU, ConnectedResample<PCNC, T>, T>,
        Resample<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    fn reset(self) -> Self {
        // self.cache_stream = None;
        // self.cache = None;
        self
    }

    fn recenter_with_resampling(self) -> Self {
        assert!(!self.delta2.is_zero());
        let center = generate_str(
            &self.k,
            &T::zero(),
            &T::zero(),
            &self.sx,
            &self.sy,
            &self.alpha,
        )
        .transform(&self.projection_raw.transform(&Coordinate {
            x: self.lambda,
            y: self.phi,
        }));
        let transform = generate_str(
            &self.k,
            &(self.x - center.x),
            &(self.y - center.y),
            &self.sx,
            &self.sy,
            &self.alpha,
        );

        let rotate = rotate_radians([self.delta_lambda, self.delta_phi, self.delta_gamma]);
        let project_transform = Compose::new(self.projection_raw.clone(), transform);
        let project_rotate_transform = Compose::new(rotate.clone(), project_transform.clone());
        let rotator = RotatorRadians::new(rotate.clone());

        let resample = Resample::new(project_transform.clone(), self.delta2);

        let out: Self = Builder {
            p_pcnc: self.p_pcnc,
            p_lb: PhantomData::<LB>,
            projection_raw: self.projection_raw,
            clip: self.clip,
            phi: self.phi,
            lambda: self.lambda,
            alpha: self.alpha,
            k: self.k,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,
            delta2: self.delta2,
            theta: self.theta,
            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1,
            rotate,
            rotator,
            postclip: self.postclip,
            resample,
            project_transform,
            project_rotate_transform,
        };
        out.reset()
    }
}

impl<DRAIN, I, LB, LC, LU, PCNC, PCNU, PR, PV, T>
    Builder<
        DRAIN,
        I,
        LB,
        LC,
        LU,
        PCNC,
        PCNU,
        PR,
        PV,
        ResampleNone<DRAIN, PR, PCNC, PCNU, Connected<PCNC>, T>,
        ResampleNone<DRAIN, PR, PCNC, PCNU, Unconnected, T>,
        T,
    >
where
    PR: Clone + Transform<T = T>,
    T: 'static + AbsDiffEq<Epsilon = T> + CoordFloat + FloatConst,
{
    fn reset(self) -> Self {
        // self.cache_stream = None;
        // self.cache = None;
        self
    }

    fn recenter_no_resampling(self) -> Self {
        let center = generate_str(
            &self.k,
            &T::zero(),
            &T::zero(),
            &self.sx,
            &self.sy,
            &self.alpha,
        )
        .transform(&self.projection_raw.transform(&Coordinate {
            x: self.lambda,
            y: self.phi,
        }));
        let transform = generate_str(
            &self.k,
            &(self.x - center.x),
            &(self.y - center.y),
            &self.sx,
            &self.sy,
            &self.alpha,
        );

        let rotate = rotate_radians([self.delta_lambda, self.delta_phi, self.delta_gamma]);
        let project_transform = Compose::new(self.projection_raw.clone(), transform);
        let project_rotate_transform = Compose::new(rotate.clone(), project_transform.clone());
        let rotator = RotatorRadians::new(rotate.clone());

        let resample = ResampleNone::new(project_transform.clone());
        let out: Self = Builder {
            p_pcnc: PhantomData::<PCNC>,
            p_lb: PhantomData::<LB>,
            projection_raw: self.projection_raw,
            clip: self.clip,
            phi: self.phi,
            lambda: self.lambda,
            alpha: self.alpha,
            k: self.k,
            sx: self.sx,
            sy: self.sy,
            x: self.x,
            y: self.y,
            delta_lambda: self.delta_lambda,
            delta_phi: self.delta_phi,
            delta_gamma: self.delta_gamma,
            delta2: self.delta2,
            theta: self.theta,
            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1,
            rotate,
            rotator,
            postclip: self.postclip,
            resample,
            project_transform,
            project_rotate_transform,
        };

        out.reset()
    }
}
