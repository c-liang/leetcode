//! # 376. 摆动序列
//!https://leetcode-cn.com/problems/wiggle-subsequence/
//!如果连续数字之间的差严格地在正数和负数之间交替，则数字序列称为摆动序列。第一个差（如果存在的话）可能是正数或负数。少于两个元素的序列也是摆动序列。
//!例如， [1,7,4,9,2,5] 是一个摆动序列，因为差值 (6,-3,5,-7,3) 是正负交替出现的。相反, [1,4,7,2,5] 和 [1,7,4,5,5] 不是摆动序列，第一个序列是因为它的前两个差值都是正数，第二个序列是因为它的最后一个差值为零。
//!给定一个整数序列，返回作为摆动序列的最长子序列的长度。 通过从原始序列中删除一些（也可以不删除）元素来获得子序列，剩下的元素保持其原始顺序。

//! #解题思路
//!根据a[n]-a[n-1]符号划分down up
//!a[n]>a[n-1] up[i]=max(up[i-1],down[i-1]+1) down[i] = down[i-1]
//!a[n]<a[n-1] down[i]=max(down[i-1],up[i-1]+1) up[i] = up[i-1]
//!a[n]=a[n-1] down[i]=down[i-1] up[i]=up[i-1]
//!

pub struct Solution;
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let (mut down, mut up) = (vec![1; len], vec![1; len]);
        for i in 1..len {
            if nums[i] > nums[i - 1] {
                down[i] = down[i - 1];
                up[i] = std::cmp::max(up[i - 1], down[i - 1] + 1);
            } else if nums[i] < nums[i - 1] {
                up[i] = up[i - 1];
                down[i] = std::cmp::max(down[i - 1], up[i - 1] + 1);
            } else {
                down[i] = down[i - 1];
                up[i] = up[i - 1];
            }
        }
        std::cmp::max(down[len - 1], up[len - 1])
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::Solution::wiggle_max_length([1, 17, 5, 10, 13, 15, 10, 5, 16, 8].into()),
            7
        );
        assert_eq!(
            super::Solution::wiggle_max_length([1, 1, 1, 1, 1, 1, 1].into()),
            1
        );
    }
}
