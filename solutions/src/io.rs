#![allow(dead_code, unused_variables, unused_macros)]

use std::fmt::Display;
use std::io::{self, BufWriter, Read, Stdout, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

// ---------- Fast IO (cin / cout style) ----------
struct IO {
    it: SplitAsciiWhitespace<'static>,
    out: BufWriter<Stdout>,
}

impl IO {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        // Leak once: input lives for the whole program anyway.
        let s: &'static str = Box::leak(s.into_boxed_str());
        Self {
            it: s.split_ascii_whitespace(),
            out: BufWriter::with_capacity(1 << 16, io::stdout()),
        }
    }

    // ---- input ----
    /// cin >> x      ->  let x: i64 = io.read();
    fn read<T: FromStr>(&mut self) -> T {
        self.it.next().unwrap().parse().ok().unwrap()
    }

    /// n values       ->  let a: Vec<i64> = io.vec(n);
    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }

    /// n x m matrix   ->  let g: Vec<Vec<i32>> = io.mat(n, m);
    fn mat<T: FromStr>(&mut self, n: usize, m: usize) -> Vec<Vec<T>> {
        (0..n).map(|_| self.vec(m)).collect()
    }

    /// a word as bytes ->  let s: Vec<u8> = io.bytes();
    fn bytes(&mut self) -> Vec<u8> {
        self.it.next().unwrap().as_bytes().to_vec()
    }

    /// a word as chars ->  let s: Vec<char> = io.chars();
    fn chars(&mut self) -> Vec<char> {
        self.it.next().unwrap().chars().collect()
    }

    /// n words (e.g. a grid of '.' and '#') -> let g: Vec<Vec<u8>> = io.grid(n);
    fn grid(&mut self, n: usize) -> Vec<Vec<u8>> {
        (0..n).map(|_| self.bytes()).collect()
    }

    /// 1-indexed -> 0-indexed shortcut: let u = io.idx();
    fn idx(&mut self) -> usize {
        self.read::<usize>() - 1
    }

    // ---- output ----
    /// cout << x << ' ';
    fn print<T: Display>(&mut self, x: T) {
        write!(self.out, "{}", x).unwrap();
    }

    /// cout << x << '\n';
    fn println<T: Display>(&mut self, x: T) {
        writeln!(self.out, "{}", x).unwrap();
    }

    /// Prints a slice space-separated on one line.
    fn line<T: Display>(&mut self, v: &[T]) {
        for (i, x) in v.iter().enumerate() {
            if i > 0 {
                write!(self.out, " ").unwrap();
            }
            write!(self.out, "{}", x).unwrap();
        }
        writeln!(self.out).unwrap();
    }

    /// Prints a slice one element per line.
    fn lines<T: Display>(&mut self, v: &[T]) {
        for x in v {
            writeln!(self.out, "{}", x).unwrap();
        }
    }

    fn yes_no(&mut self, b: bool) {
        self.println(if b { "YES" } else { "NO" });
    }
}

// Read several values of different types at once:
//   let (n, m) = rd!(io, usize, usize);
//   let (a, b, s) = rd!(io, i64, i64, String);
macro_rules! rd {
    ($io:expr, $($t:ty),+) => { ($($io.read::<$t>()),+) };
}

// Formatted output: w!(io, "{} {}", a, b);   (adds newline)
macro_rules! w {
    ($io:expr, $($arg:tt)*) => { writeln!($io.out, $($arg)*).unwrap() };
}
