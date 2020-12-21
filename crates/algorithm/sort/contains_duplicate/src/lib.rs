//! # 217. 存在重复元素
//!https://leetcode-cn.com/problems/contains-duplicate/
//!给定一个整数数组，判断是否存在重复元素。
//!如果任意一值在数组中出现至少两次，函数返回 true 。如果数组中每个元素都不相同，则返回 false 。

//! # 解题思路
//!排序完，判断是否存在相邻元素相等情况

pub struct Solution;
impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort();
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::Solution::contains_duplicate([1, 1, 1, 2, 3, 3, 2, 3, 4].into()),
            true
        );
        assert_eq!(
            super::Solution::contains_duplicate([1, 2, 3, 4].into()),
            false
        );
    }
}
