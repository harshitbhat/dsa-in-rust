```rust,ignore
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
    	let n = prices.len();
        let mut max_till_now = prices[n - 1];

        let mut max_till_i = vec![0; n];

        let mut i = n - 1;

        while(i >= 0) {
        	if prices[i] > max_till_now {
        		max_till_now = prices[i];
        	}
        	max_till_i[i] = max_till_now;
        	
        	if i == 0 {
        		break;
        	}
        	i -= 1;
        }

        let mut ans = 0;

        for i in 0..n {
        	if (max_till_i[i] - prices[i]) > ans {
        		ans = max_till_i[i] - prices[i];
        	}
        }

        ans
    }
}
```


LLM Review:

Rather than looking from right, and just compare the price with "min price so far" approach. Elegant approach

```rust,ignore
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for price in prices {
        	if price < min_price {
        		min_price = price;
        	} else if price - min_price > max_profit {
        		max_profit = price - min_price;
        	}
        }

        max_profit
    }
}
```