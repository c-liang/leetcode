//! # 389. 找不同
//! https://leetcode-cn.com/problems/find-the-difference/
//! 给定两个字符串 s 和 t，它们只包含小写字母。
//! 字符串 t 由字符串 s 随机重排，然后在随机位置添加一个字母。
//! 请找出在 t 中被添加的字母。

//! # 解题思路
//! 1.利用计算计算字符串中每一个字符出现的次数，比较找出次数不同的字符
//! 2.利用异或s,t中的所有字符进行异或，最后结果就是单独多出的一个字符

pub struct Solution;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let (mut v1, mut v2) = ([0; 26], [0; 26]);
        for i in s.as_bytes() {
            v1[(i - b'a') as usize] += 1;
        }
        for i in t.as_bytes() {
            v2[(i - b'a') as usize] += 1;
        }
        for i in 0..26 {
            if v1[i] != v2[i] {
                return (i as u8 + b'a') as char;
            }
        }
        unimplemented!("should never goes here");
    }
    pub fn find_the_difference_xor(s: String, t: String) -> char {
        let mut sum = 0;
        for i in t.as_bytes() {
            sum ^= *i;
        }
        for i in s.as_bytes() {
            sum ^= *i;
        }
        sum as char
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::Solution::find_the_difference("abcd".into(), "abcde".into()),
            'e'
        );
        assert_eq!(
            super::Solution::find_the_difference("".into(), "y".into()),
            'y'
        );
        assert_eq!(
            super::Solution::find_the_difference("a".into(), "aa".into()),
            'a'
        );
        assert_eq!(
            super::Solution::find_the_difference("ae".into(), "aea".into()),
            'a'
        );
        assert_eq!(
            super::Solution::find_the_difference_xor("abcd".into(), "abcde".into()),
            'e'
        );
        assert_eq!(
            super::Solution::find_the_difference_xor("".into(), "y".into()),
            'y'
        );
        assert_eq!(
            super::Solution::find_the_difference_xor("a".into(), "aa".into()),
            'a'
        );
        assert_eq!(
            super::Solution::find_the_difference_xor("ae".into(), "aea".into()),
            'a'
        );
    }
}
