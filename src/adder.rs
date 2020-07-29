// Adds floating point numbers with twice the normal precision.
// Reference: J. R. Shewchuk, Adaptive Precision Floating-Point Arithmetic and
// Fast Robust Geometric Predicates, Discrete & Computational Geometry 18(3)
// 305–363 (1997).
// Code adapted from GeographicLib by Charles F. F. Karney,
// http://geographiclib.sourceforge.net/

use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

static temp64: Adder<f64> = Adder::new();
static temp32: Adder<f32> = Adder<f32>::new();
// static tempF: Adder<F> = Adder<F>::new();


pub struct Adder<F>{
  s: F,
  t: F,
}

fn add_local<F>(mut adder: Adder<F>, a: F, b: F)
where F: Float{
  let x = a+ b;
  adder.s = a + b;
  let bv = x - a;
  let av = x - bv;
  adder.t = (a - av) + (b - bv);
}

impl <F>Adder<F>
where F: Float + FloatConst + FromPrimitive {

  pub fn new() -> Self {
    Self{
      s: F::zero(),
      t: F::zero(),
    }
  }

  pub fn reset(&mut self) {
    self.s = F::zero(); // rounded value
    self.t = F::zero(); // exact error
  }

  pub fn add(&mut self, y: F) {
    add_local(temp, y, self.t);
    add_local(self,temp.s, self.s);
    if !self.s.is_zero() {
      self.t += temp.s;
    }
    else {
      self.s = temp.s;
    }// constructor,
  }

  pub fn value_of(&self) -> F {
    return self.s;
  }

}

// static temp: Adder<f64> = Adder::new();




// export default function() {
//   return new Adder;
// }

// function Adder() {
//   this.reset();
// }

// Adder.prototype = {
//   constructor: Adder,
//   reset: function() {
//     this.s = // rounded value
//     this.t = 0; // exact error
//   },
//   add: function(y) {
//     add(temp, y, this.t);
//     add(this, temp.s, this.s);
//     if (this.s) this.t += temp.t;
//     else this.s = temp.t;
//   },
//   valueOf: function() {
//     return this.s;
//   }
// };

// var temp = new Adder;

// function add(adder, a, b) {
//   var x = adder.s = a + b,
//       bv = x - a,
//       av = x - bv;
//   adder.t = (a - av) + (b - bv);
// }
