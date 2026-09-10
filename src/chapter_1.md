# Chapter 1

Sample Code for Binary Search:

```rust
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let (mut low, mut high) = (0, arr.len());
    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target { return Some(mid); }
        if arr[mid] < target { low = mid + 1; } else { high = mid; }
    }
    None
}

fn main() {
    let nums = [1, 3, 5, 7, 9];
    assert_eq!(binary_search(&nums, 7), Some(3));
}
```