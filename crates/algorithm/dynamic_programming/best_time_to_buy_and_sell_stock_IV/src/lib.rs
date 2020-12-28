//! # 188. 买卖股票的最佳时机 IV
//! https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-iv/

pub struct Solution;
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let k = k.min((prices.len() / 2) as i32);
        let (mut buys, mut sells) = (
            vec![vec![0; k as usize + 1]; prices.len()],
            vec![vec![0; k as usize + 1]; prices.len()],
        );
        buys[0][0] = -prices[0];
        sells[0][0] = 0;
        for i in 1..=k as usize {
            buys[0][i] = std::i32::MIN / 2;
            sells[0][i] = std::i32::MIN / 2;
        }

        for i in 1..prices.len() {
            buys[i][0] = std::cmp::max(buys[i - 1][0], sells[i - 1][0] - prices[i]);
            for j in 1..=k as usize {
                buys[i][j] = std::cmp::max(buys[i - 1][j], sells[i - 1][j] - prices[i]);
                sells[i][j] = std::cmp::max(sells[i - 1][j], buys[i - 1][j - 1] + prices[i]);
            }
        }
        let mut ans = std::i32::MIN;
        for v in sells[prices.len() - 1].iter() {
            if v > &ans {
                ans = *v;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::max_profit(2, vec![4, 2, 1, 7]), 6);
    }
}
