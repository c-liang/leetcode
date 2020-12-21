//! # 714. 买卖股票的最佳时机含手续费
//! 给定一个整数数组 prices，其中第 i 个元素代表了第 i 天的股票价格 ；非负整数 fee 代表了交易股票的手续费用。
//! 你可以无限次地完成交易，但是你每笔交易都需要付手续费。如果你已经购买了一个股票，在卖出它之前你就不能再继续购买股票了。
//! 返回获得利润的最大值。
//! 注意：这里的一笔交易指买入持有并卖出股票的整个过程，每笔交易你只需要为支付一次手续费。

//! # 解题思路
//! 动态规划思想，核心算法为
//! dp[i][0] = max(dp[i-1][0],dp[i-1][1]+v[i]-fee)
//! dp[i][1] = max(dp[i-1][1],dp[i-1][0]-dp[i])
//! dp[0][0] = 0; dp[0][1] = -v[0]
//! dp[i][1] 表示第i天手中持有股票 dp[i][0]表示手中不持有股票

pub struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let len = prices.len();
        // let mut dp = vec![vec![0; 2]; len];
        let mut hand_stock = -prices[0];
        let mut no_stock = 0;
        for i in 1..len {
            let (pre_hand_stock, pre_no_stock) = (hand_stock, no_stock);
            no_stock = std::cmp::max(pre_no_stock, pre_hand_stock + prices[i] - fee);
            hand_stock = std::cmp::max(pre_hand_stock, pre_no_stock - prices[i]);
        }
        no_stock
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::max_profit([1, 3, 2, 8, 4, 9].into(), 2), 8);
        assert_eq!(super::Solution::max_profit([1, 3, 2, 8, 4, 9].into(), 4), 4);
    }
}
