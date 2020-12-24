//! # 135. 分发糖果
//! https://leetcode-cn.com/problems/candy/
//! 老师想给孩子们分发糖果，有 N 个孩子站成了一条直线，老师会根据每个孩子的表现，预先给他们评分。
//! 你需要按照以下要求，帮助老师给这些孩子分发糖果：
//! 每个孩子至少分配到 1 个糖果。
//! 相邻的孩子中，评分高的孩子必须获得更多的糖果。
//! 那么这样下来，老师至少需要准备多少颗糖果呢？

//! # 解题思路
//! 采用最少分配原则，每个人至少一个，左右遍历一次进行条件判断

pub struct Solution;
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let (mut left, mut right, mut ret) = (vec![1; ratings.len()], vec![1; ratings.len()], 0);
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                left[i] = left[i - 1] + 1;
            }
        }
        for i in (0..ratings.len()).rev() {
            if i < ratings.len() - 1 && ratings[i] > ratings[i + 1] {
                right[i] = right[i + 1] + 1;
            }
            ret += left[i].max(right[i]);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::candy([1, 2, 2].into()), 4);
    }
}
