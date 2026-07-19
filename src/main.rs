#![allow(dead_code)]
#![allow(unused)]

use atcoder::geometry::{segments_intersect, Point};
use atcoder::io::Writer;

fn main() {
    let mut writer = Writer::new();

    atcoder::input! {
        t: usize,
    }

    let mut answers = Vec::with_capacity(t);
    for _ in 0..t {
        atcoder::input! {
            p: (isize, isize),
            q: (isize, isize),
            r: (isize, isize),
            s: (isize, isize),
        }
        answers.push(usize::from(segments_intersect(
            Point::new(p.0, p.1),
            Point::new(q.0, q.1),
            Point::new(r.0, r.1),
            Point::new(s.0, s.1),
        )));
    }

    writer.join_line(answers);
}
