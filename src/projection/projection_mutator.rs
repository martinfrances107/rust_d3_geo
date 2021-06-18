impl<PR, T> ProjectionMutator<PR, T>
where
    ProjectionRawTrait
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    pub fn new(project: PR, delta2_p: Option<T>) -> ProjectionMutator<PR, T> {
        let delta2 = match delta2_p {
            None => {
                T::from(0.5).unwrap() // precision
            }
            Some(delta2) => delta2,
        };

        let pm = ProjectionMutator {
            project,
            alpha: T::zero(), // post-rotate angle
            // cache: None,
            // cache_stream: None,
            delta2, // precision
            delta_lambda: T::zero(),
            delta_phi: T::zero(),
            delta_gamma: T::zero(),
            // scale
            k: T::from(150f64).unwrap(),
            // translate
            lambda: T::zero(),
            phi: T::zero(),
            rotate: RotateRadiansEnum::I(RotationIdentity::default()), // pre-rotate
            preclip: ClipAntimeridian::gen_clip(),                     // stub value
            postclip: |x: ClipSinkEnum<Compose<PR, ScaleTranslateRotateEnum<T>>, T>| x,
            sx: T::one(), // reflectX
            sy: T::one(), // reflectX
            theta: None,  // pre-clip angle
            x: T::from(480f64).unwrap(),
            x0: None,
            y0: None,
            x1: None,
            y1: None, //postclip = identity, // post-clip extent
            y: T::from(250).unwrap(),
            project_resample: ResampleEnum::R(Resample::default()),
            project_transform: Compose::new(PR::default(), ScaleTranslateRotateEnum::default()),
            project_rotate_transform: Compose::new(
                RotateRadiansEnum::I(RotationIdentity::default()),
                Compose::new(PR::default(), ScaleTranslateRotateEnum::default()),
            ),
        };

        pm.recenter()
    }

    #[inline]
    fn reset(self) -> ProjectionMutator<PR, T> {
        // self.cache_stream = None;
        // self.cache = None;
        self
    }

    fn recenter(mut self) -> ProjectionMutator<PR, T> {
        let center = ScaleTranslateRotate::new(
            &self.k,
            &T::zero(),
            &T::zero(),
            &self.sx,
            &self.sy,
            self.alpha,
        )
        .transform(&self.project.transform(&Coordinate {
            x: self.lambda,
            y: self.phi,
        }));

        let transform = ScaleTranslateRotate::new(
            &self.k,
            &(self.x - center.x),
            &(self.y - center.y),
            &self.sx,
            &self.sy,
            self.alpha,
        );

        self.rotate = rotate_radians_transform(self.delta_lambda, self.delta_phi, self.delta_gamma);

        self.project_transform = Compose::new(self.project.clone(), transform);

        self.project_rotate_transform =
            Compose::new(self.rotate.clone(), self.project_transform.clone());

        self.project_resample = gen_resample_node(self.project_transform.clone(), self.delta2);

        self.reset()
    }

}

impl<PR, T> Transform for ProjectionMutator<PR, T>
where
    ProjectionRawTrait
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    type C = Coordinate<T>;
    fn transform(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let r = Coordinate {
            x: p.x.to_radians(),
            y: p.y.to_radians(),
        };
        self.project_rotate_transform.transform(&r)
    }
    fn invert(&self, p: &Coordinate<T>) -> Coordinate<T> {
        let d = self.project_rotate_transform.invert(p);
        Coordinate {
            x: d.x.to_degrees(),
            y: d.y.to_degrees(),
        }
    }
}

impl<PR, T> Projection<PR, T> for ProjectionMutator<PR,  T>
where
    ProjectionRawTrait
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    // #[inline]
    // fn get_preclip(&self) -> StreamPreClipNode<T> {
    //     self.preclip
    // }

    // fn preclip(&mut self, preclip: StreamPRClipNode<T>) {
    //     self.preclip = preclip;
    //     self.theta = None;
    //     return self.reset();
    // }

    // fn get_postclip(&self) -> Option<Box<dyn GeoStream>> {
    //   return self.postclip;
    // }

    // fn postclip(&mut self, postclip: StreamProcessor<T>) {
    //     // self.postclip = postclip;
    //     // self.theta = None;
    //     // return self.reset();
    // }

    // fn get_center(&self) -> Point {
    //   return [self.lambda.to_degrees(), self.phi.to_degrees()];
    // }

    /// TODO dynamic cast and unwrap - Must find a better way.
    // fn center(&mut self, p: Point) {
    //   // self.lambda = (p[0] % F::from_u16(360u16).unwrap()).to_radians();
    //   // self.phi = (p[1] % F::from_u16(360u16).unwrap()).to_radians();
    //   self.recenter();
    // }

    // projection.clipAngle = function(_) {
    //   return arguments.length ? (preclip = +_ ? clipCircle(theta = _ * radians) : (theta = null, clipAntimeridian), reset()) : theta * degrees;
    // };

    #[inline]
    fn get_precision(self) -> T {
        self.delta2.sqrt()
    }

    #[inline]
    fn get_reflect_x(&self) -> bool {
        self.sx < T::zero()
    }

    fn reflect_x(mut self, reflect: bool) -> ProjectionMutator<PR, T> {
        if reflect {
            self.sx = T::from(-1.0).unwrap();
        } else {
            self.sx = T::one();
        }
        self.recenter()
    }

    #[inline]
    fn get_reflect_y(&self) -> bool {
        self.sy < T::zero()
    }

    #[inline]
    fn reflect_y(mut self, reflect: bool) -> ProjectionMutator<PR, T> {
        if reflect {
            self.sy = T::from(-1.0).unwrap();
        } else {
            self.sy = T::one();
        }
        self.recenter()
    }

    fn precision(mut self, delta: T) -> ProjectionMutator<PR, T> {
        self.delta2 = delta * delta;
        self.project_resample = gen_resample_node(self.project_transform.clone(), self.delta2);
        self.reset()
    }

    #[inline]
    fn fit_extent(
        self,
        extent: [Coordinate<T>; 2],
        object: DataObject<T>,
    ) -> ProjectionMutator<PR, T> {
        fit_extent(self, extent, object)
    }

    // fn get_clip_angle(&self) -> T {}

    fn clip_angle(mut self, angle: StreamOrValueMaybe<T>) -> ProjectionMutator<PR, T> {
        match angle {
            StreamOrValueMaybe::Value(angle) => {
                let theta = angle.to_radians();
                self.theta = Some(theta);
                self.preclip = ClipCircle::gen_clip(theta);
                // println!("preclip {:#?}", self.preclip);
                // panic!("clip_angler stop");
            }
            StreamOrValueMaybe::SP(_preclip) => {
                todo!("must sort this out.");
                // self.theta = None;
                // self.preclip = preclip;
                // self.reset();
            }
        }
        self.reset()
    }

    fn get_clip_extent(&self) -> Option<[Coordinate<T>; 2]> {
        match (self.x0, self.y0, self.x1, self.y1) {
            (Some(x0), Some(y0), Some(x1), Some(y1)) => {
                Some([Coordinate { x: x0, y: y0 }, Coordinate { x: x1, y: y1 }])
            }
            _ => None,
        }
    }

    fn clip_extent(mut self, extent: Option<[Coordinate<T>; 2]>) -> ProjectionMutator<PR, T> {
        match extent {
            None => {
                self.x0 = None;
                self.y0 = None;
                self.x1 = None;
                self.y1 = None;
                // self.postclip = Identity;
                // Is this the identity projection Mutator???
                todo!("must implement identity");
            }
            Some(extent) => {
                // set x0 ...
                self.x0 = Some(extent[0].x);
                self.y0 = Some(extent[0].y);
                self.x1 = Some(extent[1].x);
                self.y1 = Some(extent[1].y);
                // todo!("must implement clip rectangle")
                // clipRectangle(self.x0, self.y0, self.x1, self.y1);
                self.reset()
            }
        }
    }

    #[inline]
    fn get_scale(&self) -> T {
        self.k
    }

    fn scale(mut self, scale: T) -> ProjectionMutator<PR, T> {
        self.k = scale;
        self.recenter()
    }

    #[inline]
    fn get_translate(&self) -> Coordinate<T> {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }

    fn translate(mut self, t: &Coordinate<T>) -> ProjectionMutator<PR, T> {
        self.x = t.x;
        self.y = t.y;
        self.recenter()
    }

    #[inline]
    fn get_rotate(&self) -> [T; 3] {
        [
            self.delta_lambda.to_degrees(),
            self.delta_phi.to_degrees(),
            self.delta_lambda.to_degrees(),
        ]
    }

    fn rotate(mut self, angles: [T; 3]) -> ProjectionMutator<PR, T> {
        let [delta_lambda, delta_phi, delta_gamma] = angles;
        let f360 = T::from(360f64).unwrap();
        self.delta_lambda = (delta_lambda % f360).to_radians();
        self.delta_phi = (delta_phi % f360).to_radians();
        self.delta_gamma = (delta_gamma % f360).to_radians();
        self.recenter()
    }
}
