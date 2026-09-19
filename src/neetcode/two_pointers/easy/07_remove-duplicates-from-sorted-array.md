```rust,ignore
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 1;
        let mut i = 1;
        let mut current_elem = nums[0];

        while i < nums.len() {
        	if nums[i] != current_elem {
        		current_elem = nums[i];
        		nums.swap(i, k);
        		i += 1;
        		k += 1;
        	} else {
        		i += 1;
        	}
        }

        k as i32
    }
}
```