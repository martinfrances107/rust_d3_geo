use crate::transform_stream::TransformStream;

#[derive(Clone, Copy, Debug)]
struct LineTuple {
    x: f64,
    y: f64,
    m: Option<u8>,
}
#[derive(Debug)]
pub struct ClipBuffer {
    lines: Vec<Vec<LineTuple>>,
    line: Vec<LineTuple>,
}

impl ClipBuffer {
    pub fn new() -> Box<dyn TransformStream> {
        return Box::new(Self {
            lines: Vec::new(),
            line: Vec::new(),
        });
    }

    fn rejoin(&mut self) {
        if self.lines.len() > 1 {
            // Shift from the top end.
            let lines_shift = self.lines.remove(0);
            // Pop from the bottom end.
            let lines_pop = self.lines.pop().unwrap_or(Vec::new());
            let join = [lines_pop, lines_shift].concat();
            self.lines.push(join);
        }
    }

    fn result(&mut self) -> Vec<Vec<LineTuple>> {
        self.lines.clear();
        self.line.clear();
        let result = &self.lines;
        return result.to_vec();
    }
}

impl<'a> TransformStream for ClipBuffer {
    fn point(&mut self, x: f64, y: f64, m: Option<u8>) {
        self.line.push(LineTuple { x, y, m });
    }

    fn line_start(&mut self) {
        self.line.clear();
        // self.lines.push(self.line);
    }

    fn line_end(&mut self) {
        // no-op.
    }
}
// import noop from "../noop.js";

// export default function() {
//   var lines = [],
//       line;
//   return {
//     point: f64unction(x, y, m) {
//       line.push([x, y, m]);
//     },
//     lineStart: f64unction() {
//       lines.push(line = []);
//     },
//     lineEnd: noop,
//     rejoin: f64unction() {
//       if (lines.length > 1) lines.push(lines.pop().concat(lines.shift()));
//     },
//     result: f64unction() {
//       var result = lines;
//       lines = [];
//       line = null;
//       return result;
//     }
//   };
// }
