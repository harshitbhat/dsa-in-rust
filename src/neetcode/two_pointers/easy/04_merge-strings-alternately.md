```rust,ignore
use std::cmp;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let n1 = word1.len();
        let n2 = word2.len();

        let mut merged = vec![' '; n1 + n2];

        let word1_ch: Vec<char> = word1.chars().collect();
        let word2_ch: Vec<char> = word2.chars().collect();

        let mut i = 0;
        let mut k = 0;

        while k < cmp::min(n1, n2) {
        	merged[i] = word1_ch[k];
        	merged[i + 1] = word2_ch[k];

        	i += 2;
            k += 1;
        }

        if k == n1 {
        	for j in n1..n2 {
        		merged[i] = word2_ch[j];
        		i += 1;
        	}
        } else {
        	for j in n2..n1 {
        		merged[i] = word1_ch[j];
        		i += 1;
        	}
        }

        merged.into_iter().collect()

    }
}
```


LLM Suggested:

```rust,ignore
impl Solution {
	pub fn merge_alternatively(word1: String, word2:String) -> String {
		let b1 = word1.as_bytes();
		let b2 = word2.as_bytes();

		let mut result = Vec::with_capicity(b1.len() + b2.len());

		let mut i = 0;
		while i < b1.len() || i < b2.len() {
			if i < b1.len() { result.push(b1[i]); }
			if i < b2.len() { result.push(b2[i]); }

			i += 1; 
		}

		unsafe { String::from_utf_unchecked(result) }
	}
}

```


#### What each part does

**`word1.as_bytes()`**
Returns a `&[u8]` — a slice of the string's raw bytes — without any allocation. No copy, just a view into the existing data.

**`Vec::with_capacity(n)`**
Pre-allocates exactly the space needed upfront. Without this, `push` would trigger multiple reallocations as the vec grows. This is just a performance hint.

**The while loop**
Interleaves bytes from both slices, one index at a time. Simpler than your two-phase approach (min loop + remainder loop) because the `if i < b1.len()` guards handle the unequal length case naturally.

**`String::from_utf8_unchecked(result)`**
This is the interesting part.

---

#### Why `unsafe` here?

Normally you'd use `String::from_utf8(result)` which returns a `Result` and validates that the bytes are valid UTF-8. The `unchecked` version **skips that validation**.

It's marked `unsafe` because if the bytes happen to *not* be valid UTF-8, you get **undefined behavior** — memory corruption, wrong output, crashes, anything.

The reason it's safe *here*: LeetCode guarantees the inputs contain only lowercase English letters (`a-z`), which are all single-byte ASCII characters (values 0–127). ASCII is always valid UTF-8, so the check would always pass anyway — `unchecked` just skips the redundant scan.

```ignore
'a' = 0x61  ✓ valid UTF-8
'z' = 0x7A  ✓ valid UTF-8
All ASCII (0x00–0x7F) = always valid UTF-8
```

---

#### Is it safe to use in an interview?

**The caveat:** In a real production codebase, most senior Rust developers would prefer the safe version:

```rust,ignore
String::from_utf8(result).unwrap()
// or
String::from_utf8(result).expect("inputs are ASCII")
```

The performance difference is negligible (one O(n) scan), and you avoid the `unsafe` block entirely. The `unsafe` version is only worth it in extremely hot paths.

**What to say if asked about it:** *"I used `from_utf8_unchecked` because the problem guarantees ASCII-only input, making the UTF-8 check redundant. In production I'd use `from_utf8().unwrap()` unless profiling showed it mattered."*
