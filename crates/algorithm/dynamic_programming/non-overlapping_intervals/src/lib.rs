//! # 435. 无重叠区间
//! 给定一个区间的集合，找到需要移除区间的最小数量，使剩余区间互不重叠。

//! # 解题思路
//! 动态规划 根据起始值对区间进行排序，f(i)= max(f(i),f(j)) (0<=j<=j), f(0..n) = 1; 找出最大的不重合区间数
//! 贪心 根据结束值对区间进行排序，找到下一个不跟前一个区间有重复值的区间，算出最大的不重合区间数

pub struct Solution;
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        //动态规划
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut dp = vec![1; intervals.len()];
        for i in 1..intervals.len() {
            for j in 0..=i {
                if intervals[j][1] <= intervals[i][0] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        let mut max = 0;
        for v in dp {
            if v > max {
                max = v;
            }
        }
        (intervals.len() - max) as i32
    }
    pub fn erase_overlap_intervals_greedy(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        //贪心算法
        intervals.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut ans = 1;
        let mut right = intervals[0][1];
        for i in 1..intervals.len() {
            if intervals[i][0] >= right {
                ans += 1;
                right = intervals[i][1];
            }
        }
        intervals.len() as i32 - ans
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::Solution::erase_overlap_intervals(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 3]
            ]),
            1
        );
        assert_eq!(
            super::Solution::erase_overlap_intervals_greedy(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 3]
            ]),
            1
        );
    }
}
