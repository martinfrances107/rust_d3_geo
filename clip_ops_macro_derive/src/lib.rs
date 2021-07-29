extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ClipOps)]
pub fn clip_ops_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_clip_ops_macro(&ast)
}

fn impl_clip_ops_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl<SINK, T> ClipOps for #name<SINK, T>
            where
            SINK: Stream<SC = Coordinate<T>> + Default,
            T: ::core::ops::AddAssign + ::num_traits::AsPrimitive<T> + ::geo::CoordFloat + ::num_traits::FloatConst + ::core::fmt::Display,{
            type COT = Coordinate<T>;
            fn hello_macro(&self) -> u32{
                println!("Hello, Macro! My name is {}!", stringify!(#name));
                42
            }
            #[inline]
            fn point_default(&mut self, p: &Coordinate<T>, m: ::std::option::Option<u8>) {
                println!("clip point_default");
                if self.point_visible(p, None) {
                    // self.get_base().sink.borrow_mut().point(p, m);
                }
            }

            #[inline]
            fn point_line(&mut self, p: &Coordinate<T>, m: ::std::option::Option<u8>) {
                println!("clip point_line");
                // self.get_base().line.point(p, m);
            }

            #[inline]
            fn line_start_default(&mut self) {
                println!("clip line_start_default");
                // let base = self.get_base();
                // self.point_fn = Self::point_line;
                // self.base.use_point_line = true;
                // self.set_use_point_line(true);
                self.line_start();
            }

            #[inline]
            fn line_end_default(&mut self) {
                println!("clip line_end_default");
                // self.point_fn = Self::point_default;
                // self.set_use_point_line(false);
                // self.base.use_point_line = false;
                // self.get_base().line.line_end();
            }

            #[inline]
            fn point_ring(&mut self, p: &Coordinate<T>, m: Option<u8>) {
                println!("clip point_ring {:?} {:?}", p, m);
                // println!("about to ring/push - ring_sink ");
                // println!("self.base {:#?} ", self.base.ring_sink);
                // let mut base = self.get_base();
                // base.ring.push(LineElem { p: *p, m });
                // base.ring_sink.point(p, m);
                println!("clip point_ring -- end");
            }

            #[inline]
            fn ring_start(&mut self) {
                println!("clip ring_start");
                // self.get_base().ring_sink.line_start();
                self.base.ring.clear();
                // self.ring_clear();
                println!("end clip ring_start");
            }

            fn ring_end(&mut self) {
                // let mut base = self.get_base();
                // println!("clip ring_end  entry {:#?}", base.ring);
                // let le = base.ring[0];
                // javascript version drops m here.
                // self.point_ring(&le.p, None);
                // let mut base = self.get_base();
                // base.ring_sink.line_end();

                // let clean = self.base.ring_sink.clean();
                // let clean = self.ring_sink_clean();

                // let mut ring_segments = match self.get_base().ring_buffer.borrow_mut().result() {
                //     Some(PathResultEnum::ClipBufferOutput(result)) => {
                //         // Can I find a way of doing this with the expense of dynamic conversion.
                //         result
                //     }
                //     Some(_) => {
                //         panic!("None buffer ");
                //     }
                //     None => panic!("was expecting something."),
                // };

                // println!("clip ring_end() - ring segments {:#?}", ring_segments);
                // panic!("ring_end buffer result");
                // let n = ring_segments.len();
                // let m;
                let mut point: Coordinate<T>;

                self.base.ring.pop();
                // self.base.polygon.push(self.base.ring.clone());
                // self.polygon_push(self.get_base().ring.clone());
                // in this javascript version this value is set to NULL
                // is my assumption that this is valid true?
                // self.ring = None;
                self.base.ring = Vec::new();
                // self.ring_reset();

                // if n == 0 {
                //     return;
                // }
                // println!("no intersections n, c {:?} {:?}", n, clean);
                // No intersections.
                // match clean {
                //     CleanEnum::NoIntersections => {
                //         println!("about to clean good path");
                //         // panic!("on the good path");
                //         // let segment = ring_segments
                //         //     .pop_front()
                //         //     .expect("We have previously checked that the .len() is >0 ( n ) ");
                //         // m = segment.len() - 1;
                //         if m > 0 {
                //             let base = self.get_base();
                //             if !base.polygon_started {
                //                 base.sink.borrow_mut().polygon_start();
                //                 // self.base.polygon_started = true;
                //                 self.set_polygon_started(true);
                //             }
                //             self.get_base().sink.borrow_mut().line_start();
                //             for i in 0..m {
                //                 // point = segment[i].p;
                //                 // self.get_base().sink.borrow_mut().point(&point, None);
                //             }
                //             // self.get_base().sink.borrow_mut().line_end();
                //         }
                //         return;
                //     }
                //     // CleanEnum::IntersectionsRejoin => {
                //     //     // Rejoin connected segments.
                //     //     // TODO reuse ringBuffer.rejoin()?
                //     //     if n > 1 {
                //     //         println!("funny buisness");
                //     //         println!("ring_segemtns before fb {:#?}", ring_segments);
                //     //         let pb = [
                //     //             ring_segments.pop_back().unwrap(),
                //     //             ring_segments.pop_front().unwrap(),
                //     //         ]
                //     //         .concat();
                //     //         ring_segments.push_back(pb);
                //     //     }
                //     // }
                //     CleanEnum::IntersectionsOrEmpty => {
                //         // No-op
                //     }
                //     CleanEnum::Undefined => {
                //         panic!("must be defined by now.")
                //     }
                // }
                // println!("final segments before filter {:#?}", ring_segments);
                // panic!("final segments");
                // let filtered: Vec<Vec<LineElem<T>>> = ring_segments
                //     .into_iter()
                //     .filter(|segment| segment.len() > 1)
                //     .collect();
                // self.get_base().segments.push_back(filtered);
            }
        }
    };
    gen.into()
}
