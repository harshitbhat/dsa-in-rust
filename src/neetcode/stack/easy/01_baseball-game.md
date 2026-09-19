```rust,ignore
impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut result: Vec<i32> = Vec::new();

        for op in operations {
        	match op.as_str() {
        		"+" => {
        			result.push(Self::get_top_two_sum(&result));
        		},
        		"D" => {
        			result.push(2 * Self::get_top(&result));
        		},
        		"C" => {
        			result.pop();
        		},
        		_ => {
        			result.push(op.parse().unwrap());
        		}
        	}
        }

        result.iter().sum()
    }

    pub fn get_top(array: &Vec<i32>) -> i32 {
    	let n = array.len();
    	array[n - 1]
    }

    pub fn get_top_two_sum(array: &Vec<i32>) -> i32 {
    	let n = array.len();
    	array[n - 1] + array[n - 2]
    }
}
```

