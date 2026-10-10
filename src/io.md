# Fast IO in Rust

Rust has no `cin`, and calling `read_line` for every number is slow and verbose. This template gives you a small `IO` struct that reads all of stdin once, parses tokens on demand, and buffers output. It is enough for almost every CSES and Codeforces problem.

## The template

```rust,ignore
#![allow(dead_code, unused_variables, unused_macros)]

use std::fmt::Display;
use std::io::{self, BufWriter, Read, Stdout, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

struct IO {
    it: SplitAsciiWhitespace<'static>,
    out: BufWriter<Stdout>,
}

impl IO {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        // Leak once: the input lives for the whole program anyway.
        let s: &'static str = Box::leak(s.into_boxed_str());
        Self {
            it: s.split_ascii_whitespace(),
            out: BufWriter::with_capacity(1 << 16, io::stdout()),
        }
    }

    // ---- input ----
    fn read<T: FromStr>(&mut self) -> T {
        self.it.next().unwrap().parse().ok().unwrap()
    }
    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    fn mat<T: FromStr>(&mut self, n: usize, m: usize) -> Vec<Vec<T>> {
        (0..n).map(|_| self.vec(m)).collect()
    }
    fn bytes(&mut self) -> Vec<u8> {
        self.it.next().unwrap().as_bytes().to_vec()
    }
    fn chars(&mut self) -> Vec<char> {
        self.it.next().unwrap().chars().collect()
    }
    fn grid(&mut self, n: usize) -> Vec<Vec<u8>> {
        (0..n).map(|_| self.bytes()).collect()
    }
    fn idx(&mut self) -> usize {
        self.read::<usize>() - 1
    }

    // ---- output ----
    fn print<T: Display>(&mut self, x: T) {
        write!(self.out, "{}", x).unwrap();
    }
    fn println<T: Display>(&mut self, x: T) {
        writeln!(self.out, "{}", x).unwrap();
    }
    fn line<T: Display>(&mut self, v: &[T]) {
        for (i, x) in v.iter().enumerate() {
            if i > 0 {
                write!(self.out, " ").unwrap();
            }
            write!(self.out, "{}", x).unwrap();
        }
        writeln!(self.out).unwrap();
    }
    fn lines<T: Display>(&mut self, v: &[T]) {
        for x in v {
            writeln!(self.out, "{}", x).unwrap();
        }
    }
    fn yes_no(&mut self, b: bool) {
        self.println(if b { "YES" } else { "NO" });
    }
}

// let (n, k) = rd!(io, usize, i64);
macro_rules! rd {
    ($io:expr, $($t:ty),+) => { ($($io.read::<$t>()),+) };
}

// w!(io, "{} {}", a, b);   (adds a newline)
macro_rules! w {
    ($io:expr, $($arg:tt)*) => { writeln!($io.out, $($arg)*).unwrap() };
}
```

## How it works

- `IO::new()` reads the **entire** stdin into one string, then hands out whitespace-separated tokens one at a time. Line breaks do not matter, just like `cin`.
- Output goes through a `BufWriter`, so thousands of `println` calls cost almost nothing. It flushes automatically when `io` is dropped at the end of `main`.
- `read` is generic. Whatever type you annotate is what the token gets parsed into.

## Cheat sheet

| C++ | Rust |
|---|---|
| `cin >> n;` | `let n: usize = io.read();` |
| `cin >> n >> k;` (mixed types) | `let (n, k) = rd!(io, usize, i64);` |
| `vector<ll> a(n); for... cin >> a[i];` | `let a: Vec<i64> = io.vec(n);` |
| `n x m` matrix | `let g: Vec<Vec<i32>> = io.mat(n, m);` |
| `string s; cin >> s;` | `let s: Vec<u8> = io.bytes();` |
| string as chars | `let s: Vec<char> = io.chars();` |
| string as an owned `String` | `let s: String = io.read();` |
| `n` rows of characters (a grid) | `let g = io.grid(n);` |
| 1-indexed node to 0-indexed | `let u = io.idx();` |
| `cout << x << '\n';` | `io.println(x);` |
| `cout << x << ' ';` | `io.print(x);` |
| print a vector on one line | `io.line(&a);` |
| print a vector, one per line | `io.lines(&a);` |
| formatted output | `w!(io, "{} {}", a, b);` |
| `YES` / `NO` | `io.yes_no(cond);` |

## Example: CSES Weird Algorithm

```rust,ignore
fn main() {
    let mut io = IO::new();
    let mut n: i64 = io.read();
    loop {
        io.print(format!("{} ", n));
        if n == 1 {
            break;
        }
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
    }
    io.println("");
}
```

## Example: multiple test cases

```rust,ignore
fn main() {
    let mut io = IO::new();
    let t: usize = io.read();
    for _ in 0..t {
        let n: usize = io.read();
        let a: Vec<i64> = io.vec(n);
        io.println(a.iter().sum::<i64>());
    }
}
```

## Example: graph input

```rust,ignore
fn main() {
    let mut io = IO::new();
    let (n, m) = rd!(io, usize, usize);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let (u, v) = (io.idx(), io.idx());
        adj[u].push(v);
        adj[v].push(u);
    }
    let degrees: Vec<usize> = adj.iter().map(|x| x.len()).collect();
    io.line(&degrees);
}
```

## Tips

- **Integer types:** use `i64` or `u64` whenever values can pass about 2 * 10^9, the same rule as `long long` in C++. Use `usize` for anything that indexes a `Vec`.
- **Parse failures panic.** If you annotate the wrong type (say `u32` for a negative number), you get an `unwrap` panic. That is the same as undefined input in C++, just louder.
- **Not for interactive problems.** `IO::new()` waits for all of stdin before doing anything, so it will hang on problems that need you to flush and read replies. Those need a line-by-line reader instead.
- **Lines with spaces:** `read::<String>()` only returns one token. If a problem gives you a full sentence per line, read it with `io::stdin().read_line` instead.
