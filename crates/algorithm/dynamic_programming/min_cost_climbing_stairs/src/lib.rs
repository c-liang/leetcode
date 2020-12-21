//! # 746. 使用最小花费爬楼梯
//! https://leetcode-cn.com/problems/min-cost-climbing-stairs/

//! # 解题思路
//! 采用动态规划思想 dp[i] = min(dp[i-2]+v[i-2], dp[i-1]+v[i-1]) dp[0]=dp[1]=0
//! dp[i]表示爬上第i层所需的最小花费

pub struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut cos0, mut cos1) = (0, 0);
        for i in 2..(cost.len() + 1) {
            let cos = std::cmp::min(cos0 + cost[i - 2], cos1 + cost[i - 1]);
            cos0 = cos1;
            cos1 = cos;
        }
        cos1
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::Solution::min_cost_climbing_stairs([1, 100, 1, 1, 1, 100, 1, 1, 100, 1].into()),
            6
        );
        assert_eq!(
            super::Solution::min_cost_climbing_stairs([10, 15, 20].into()),
            15
        );
    }
}
