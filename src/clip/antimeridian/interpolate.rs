use crate::clip::InterpolateRaw;
use crate::clip::InterpolateTrait;
use crate::projection::stream_node::StreamNode;
use crate::stream::Stream;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::AsPrimitive;
use num_traits::FloatConst;
use std::cell::RefMut;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::AddAssign;

/// Antimeridian Interpolate.
#[derive(Clone, Debug)]
pub struct Interpolate<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    pd: PhantomData<T>,
}

impl<T> Default for Interpolate<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    fn default() -> Interpolate<T> {
        Interpolate {
            pd: PhantomData::<T>,
        }
    }
}

impl<T> InterpolateRaw for Interpolate<T> where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst
{
}

impl<I, SINK, T> InterpolateTrait for RefMut<'_, StreamNode<I, SINK, T>>
where
    I: InterpolateRaw,
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type IC = Coordinate<Self::IT>;
    type IT = T;
    fn interpolate(&mut self, to: Option<Self::IC>, from: Option<Self::IC>, dir: Self::IT) {
        todo!("is this the right thing to do");
    }
}
impl<SINK, T> InterpolateTrait for StreamNode<Interpolate<T>, SINK, T>
where
    SINK: Stream<SC = Coordinate<T>>,
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    type IC = Coordinate<Self::IT>;
    type IT = T;

    fn interpolate(
        &mut self,
        to: Option<Coordinate<T>>,
        from: Option<Coordinate<T>>,
        direction: T,
    ) {
        // let stream = Interpolate::get_sink(self);
        let mut stream = self.sink.borrow_mut();
        let phi: T;
        match from {
            None => {
                phi = direction * T::FRAC_PI_2();
                self.sink.borrow_mut().point(
                    &Coordinate {
                        x: -T::PI(),
                        y: phi,
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: T::zero(),
                        y: phi,
                    },
                    None,
                );
                stream.point(&Coordinate { x: T::PI(), y: phi }, None);
                stream.point(
                    &Coordinate {
                        x: T::PI(),
                        y: T::zero(),
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: T::PI(),
                        y: -phi,
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: T::zero(),
                        y: -phi,
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: -phi,
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: T::zero(),
                    },
                    None,
                );
                stream.point(
                    &Coordinate {
                        x: -T::PI(),
                        y: phi,
                    },
                    None,
                );
            }
            Some(from) => {
                let to = to.unwrap();
                if (from.x - to.x).abs() > T::epsilon() {
                    let lambda = if from.x < to.x { T::PI() } else { -T::PI() };

                    phi = direction * lambda / T::from(2).unwrap();
                    stream.point(&Coordinate { x: -lambda, y: phi }, None);
                    stream.point(
                        &Coordinate {
                            x: T::zero(),
                            y: phi,
                        },
                        None,
                    );
                    stream.point(&Coordinate { x: lambda, y: phi }, None);
                } else {
                    stream.point(&to, None);
                }
            }
        }
    }
}
