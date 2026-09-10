#![allow(dead_code, unused_variables)]

use std::io::{self, BufWriter, Read, StdoutLock};
use std::str::SplitAsciiWhitespace;

struct IO<'a> {
    iter: SplitAsciiWhitespace<'a>,
    out: BufWriter<StdoutLock<'static>>,
}

impl<'a> IO<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            iter: input.split_ascii_whitespace(),
            out: BufWriter::new(io::stdout().lock()),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }

    fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
}

// Default input used for mdBook playground when stdin is not provided.
// Readers can edit this value directly in the book and click Play.
const DEMO_INPUT: &str = "3";

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let input = if buffer.trim().is_empty() {
        DEMO_INPUT
    } else {
        &buffer
    };

    let mut io = IO::new(input);
    let mut n: i64 = io.read();

    loop {
        if n == 1 {
            break;
        }

        print!("{n} ");

        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
    }
    print!("{n}");
}
