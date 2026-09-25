# Course Schedule

[LeetCode #207](https://leetcode.com/problems/course-schedule/description/)

---

### Statement

There are `numCourses` courses (labeled `0` to `numCourses - 1`). `prerequisites[i] = [ai, bi]` means you must take course `bi` before course `ai`. Return `true` if you can finish all courses, `false` if a cycle makes it impossible.

**Constraints:**
- \\( 1 \le \text{numCourses} \le 2000 \\)
- \\( 0 \le \text{prerequisites.length} \le 5000 \\)
- `prerequisites[i].length == 2`, all pairs are unique.
- \\( 0 \le a_i, b_i < \text{numCourses} \\)

### Examples

**Example 1:**
```text
Input: numCourses = 2, prerequisites = [[1,0]]
Output: true
Explanation: Take course 0 first, then course 1.
```

**Example 2:**
```text
Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
Output: false
Explanation: Cycle: 0 requires 1, 1 requires 0.
```

---

### Solution

Kahn's algorithm (topological sort via BFS). Build the graph and compute in-degrees. Seed a queue with all zero-in-degree nodes (no prerequisites). Process the queue: for each node, decrement its neighbours' in-degrees and enqueue any that reach zero. If all nodes are processed, no cycle exists.

```rust,ignore
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;

        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut indegree = vec![0; n];

        for p in &prerequisites {
        	let (course, pre) = (p[0] as usize, p[1] as usize);
        	graph[pre].push(course);
        	indegree[course] += 1;
        }

        let mut queue = std::collections::VecDeque<usize> = (0..n)
        	.filter(|&i| indegree[i] == 0)
        	.collect();

        let mut taken = 0;

        while let Some(u) = queue.pop_front() {
        	taken += 1;

        	for v in &graph[u] {
        		indegree[v] -= 1;
        		if indegree[v] == 0 {
        			queue.push_back(v);
        		}
        	}
        }

        taken == n
    }
}
```
