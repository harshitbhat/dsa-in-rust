# Best Time to Buy and Sell Stock

[LeetCode #121](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/)

---

### Statement

You are given an array `prices` where `prices[i]` is the price of a given stock on the `i`th day.

You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock. Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return `0`.

**Constraints:**
- \\( 1 \le \text{prices.length} \le 10^5 \\)
- \\( 0 \le \text{prices}[i] \le 10^4 \\)

### Examples

**Example 1:**
```text
Input: prices = [7,1,5,3,6,4]
Output: 5
Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6 - 1 = 5.
```

**Example 2:**
```text
Input: prices = [7,6,4,3,1]
Output: 0
Explanation: Prices are always declining — no profitable transaction is possible.
```

---

### Solution

#### Initial approach (right-to-left max prefix):

Precompute a `max_till_i` array where each position holds the maximum price from that index to the end. Then iterate left to right and compute `max_till_i[i] - prices[i]` for the best sell-after-buy profit.

```rust,ignore
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut max_till_now = prices[n - 1];

        let mut max_till_i = vec![0; n];

        let mut i = n - 1;

        while i >= 0 {
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

#### Approach 2 (single pass — minimum price so far):

Instead of looking right, track the minimum price seen so far. At each step, the best profit achievable by selling today is `price - min_price`. No extra allocation needed.

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
