# Meeting Rooms

[LeetCode #252](https://leetcode.com/problems/meeting-rooms/description/)

---

### Statement

Given an array of meeting time intervals `intervals` where `intervals[i] = [start_i, end_i]`, determine if a person could attend all meetings (no two intervals overlap).

**Constraints:**
- \\( 0 \le \text{intervals.length} \le 10^4 \\)
- `intervals[i].length == 2`
- \\( 0 \le \text{start}_i < \text{end}_i \le 10^6 \\)

### Examples

**Example 1:**
```text
Input: intervals = [[0,30],[5,10],[15,20]]
Output: false
```

**Example 2:**
```text
Input: intervals = [[7,10],[2,4]]
Output: true
```

---

### Solution

#### Initial approach (mark-and-sweep):

Use a window array: for each interval, increment every index in `[start, end)`. Any index with a count above 1 signals an overlap.

```rust,ignore
impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        const MAX: usize = 100000 + 2;
        let mut window = [0; MAX];

        for interval in intervals {
            let start = interval[0];
            let end = interval[1];

            for i in start..end {
                window[i as usize] += 1;
            }
        }

        for i in 0..MAX {
            if window[i] > 1 {
                return false;
            }
        }

        true
    }
}
```

#### Better approach (sort + sweep):

Sort by start time, then check each adjacent pair. If the next meeting starts before the current one ends, there's an overlap. \\(\mathcal{O}(n \log n)\\) instead of \\(\mathcal{O}(\text{MAX})\\).

```rust,ignore
impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|i| i[0]);

        for w in intervals.windows(2) {
            if w[1][0] < w[0][1] {
                return false;
            }
        }

        true
    }
}
```
