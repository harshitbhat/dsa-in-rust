```rust,ignore
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
        	let mid: usize = (left + right) / 2;

        	if nums[mid] == target {
        		return mid as i32;
        	} else if (left == right) {
                break;
        	} else if nums[mid] < target {
        		left = mid + 1;
        	} else {
                if(mid == 0) {
                    break
                }
        		right = mid - 1;
        	}
        }

        if target < nums[left] {
        	left as i32
        } else {
			(left + 1) as i32
        }
    }
}
```

LLM suggested, let the loop run its course, and the left will always be at the right place, with early break that was necessary. 

```rust,ignore
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
        	let mid: usize = (left + right) / 2;

        	if nums[mid] == target {
        		return mid as i32;
        	} else if nums[mid] < target {
        		left = mid + 1;
        	} else {
                if(mid == 0) {
                    break
                }
        		right = mid - 1;
        	}
        }

        left as i32
    }
}
```


