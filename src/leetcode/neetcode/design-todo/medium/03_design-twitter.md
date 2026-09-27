# Design Twitter

[LeetCode #355](https://leetcode.com/problems/design-twitter/description/)

---

### Statement

Design a simplified Twitter where users can post tweets, follow/unfollow others, and see the most recent tweets in their news feed.

Implement `Twitter`:
- `Twitter()` — initializes the object.
- `void postTweet(int userId, int tweetId)` — user posts a tweet.
- `List<Integer> getNewsFeed(int userId)` — returns the 10 most recent tweet IDs from the user and everyone they follow, ordered newest first.
- `void follow(int followerId, int followeeId)` — `followerId` follows `followeeId`.
- `void unfollow(int followerId, int followeeId)` — `followerId` unfollows `followeeId`.

**Constraints:**
- \\( 1 \le \text{userId}, \text{followerId}, \text{followeeId} \le 500 \\)
- \\( 0 \le \text{tweetId} \le 10^4 \\)
- All tweet IDs are unique.
- At most \\( 3 \times 10^4 \\) calls in total.

### Examples

**Example 1:**
```text
Input:  ["Twitter","postTweet","getNewsFeed","follow","postTweet","getNewsFeed","unfollow","getNewsFeed"]
        [[],[1,5],[1],[1,2],[2,6],[1],[1,2],[1]]
Output: [null,null,[5],null,null,[6,5],null,[5]]
```

---

### Solution

*Not yet implemented.*
