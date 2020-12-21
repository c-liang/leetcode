//! # 738. 单调递增的数字
//! https://leetcode-cn.com/problems/monotone-increasing-digits/
//!给定一个非负整数 N，找出小于或等于 N 的最大的整数，同时这个整数需要满足其各个位数上的数字是单调递增。
//!（当且仅当每个相邻位数上的数字 x 和 y 满足 x <= y 时，我们称这个整数是单调递增的。）

//! #解题思路
///! 贪心，分治思想，数字从高位到地位，一旦出现逆序，把前一高位减一，后面所有位置9，把高位数字重新计算
pub struct Solution;
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let chars: Vec<u8> = n.to_string().bytes().map(|b| b - b'0').collect();
        let mut high = chars[0] as i32;
        for i in 1..chars.len() {
            //出现逆序跳出
            if chars[i] < chars[i - 1] {
                let next_factor = 10u32.pow((chars.len() - i) as u32);
                //重新计算高位数字
                high = Self::monotone_increasing_digits(high - 1);
                high *= next_factor as i32;
                high += (next_factor - 1) as i32;
                break;
            } else {
                high = high * 10 + (chars[i] as i32);
            }
        }
        high
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::monotone_increasing_digits(10), 9);
        assert_eq!(
            super::Solution::monotone_increasing_digits(15324868),
            14999999
        );
        assert_eq!(super::Solution::monotone_increasing_digits(33332), 29999);
        assert_eq!(super::Solution::monotone_increasing_digits(0), 0);
        assert_eq!(super::Solution::monotone_increasing_digits(9), 9);
    }
}
