//! # 330. 按要求补齐数组
//! https://leetcode-cn.com/problems/patching-array/
//! 给定一个已排序的正整数数组 nums，和一个正整数 n 。从 [1, n] 区间内选取任意个数字补充到 nums 中，
//!     使得 [1, n] 区间内的任何数字都可以用 nums 中某几个数字的和来表示。请输出满足上述要求的最少需要补充的数字个数。

//! # 思路
//!  贪心法则，核心算法 如果 [1,x-1]∈f(1..x-1) 则 [1,x+n-1]∈f(1..x-1,n) f(1..x)表示nums数组
//! 区间[1..n]最小表示f(1,2,4..n/2)

pub struct Solution;
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut index = 0;
        let mut x = 1 as i64;
        let mut ans = 0;
        while x <= (n as i64) {
            if index < nums.len() && (nums[index] as i64) <= x {
                x += nums[index] as i64;
                index += 1;
            } else {
                x *= 2;
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::min_patches(vec![1, 5, 10], 20), 2);
        assert_eq!(super::Solution::min_patches(vec![1, 2, 2], 5), 0);
    }
}
