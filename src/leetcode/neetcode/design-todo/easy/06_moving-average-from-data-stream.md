Question: https://leetcode.com/problems/moving-average-from-data-stream/description/

```rust,ignore
use std::collections::VecDeque;

struct MovingAverage {
	queue: VecDeque<i32>,
	size: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {

    fn new(size: i32) -> Self {
        Self {
        	queue: VecDeque::new(),
        	size: size,
        }
    }
    
    fn next(&mut self, val: i32) -> f64 {
        self.queue.push_back(val);

        if self.queue.len() > self.queue.size() {
        	self.queue.pop_front();
        }

        let mean: f64 = self.queue.iter().sum() / self.queue.len();

        mean
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */
```