# Weird Algorithm

[CSES #1068](https://cses.fi/problemset/task/1068)

---

### Statement

Consider an algorithm that takes as input a positive integer \\( n \\):
- If \\( n \\) is **even**, divide it by two: \\( n \leftarrow n / 2 \\)
- If \\( n \\) is **odd**, multiply it by three and add one: \\( n \leftarrow 3n + 1 \\)

Repeat the process until \\( n = 1 \\). For example, for \\( n = 3 \\):

\\[ 3 \to 10 \to 5 \to 16 \to 8 \to 4 \to 2 \to 1 \\]

Simulate the execution of the algorithm and print all values of \\( n \\) during the process.

- **Constraints:** \\( 1 \le n \le 10^6 \\)
### Example

| Input | Output |
| :--- | :--- |
| `3` | `3 10 5 16 8 4 2 1` |

---

### Approach & Notes

This is a direct simulation of the Collatz sequence until the termination condition \\( n = 1 \\).

> **Trap (Integer Overflow):** While the input constraint is \\( n \le 10^6 \\), intermediate multiplications (\\( 3n + 1 \\)) can easily exceed the range of a standard 32-bit signed integer. Always use 64-bit integers (`u64` or `i64`) in Rust.

#### Core Logic

```rust,ignore
while n != 1 {
    print!("{n} ");
    if n % 2 == 0 {
        n /= 2;
    } else {
        n = 3 * n + 1;
    }
}
println!("1");
```

<details>
<summary><b>Complete solution</b></summary>

```rust,editable
{{#include ../../../solutions/src/cses/01_introductory_problems/01_wierd_algorithm.rs}}
```
</details>