//! # 387. 字符串中的第一个唯一字符
//! https://leetcode-cn.com/problems/first-unique-character-in-a-string/
//! 给定一个字符串，找到它的第一个不重复的字符，并返回它的索引。如果不存在，则返回 -1。

//! # 解题思路
//! 对字符进行计算排序，然后遍历找到只出现一次的最小下标值

pub struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut letters: [Option<(i32, usize)>; 26] = [None; 26];
        for i in 0..s.as_bytes().len() {
            if let Some(v) = &mut letters[(s.as_bytes()[i] - b'a') as usize] {
                v.0 += 1;
            } else {
                letters[(s.as_bytes()[i] - b'a') as usize] = Some((1, i))
            }
        }
        let mut ans = std::usize::MAX;
        for i in 0..letters.len() {
            if let Some(ref v) = letters[i] {
                if v.0 == 1 && v.1 < ans {
                    ans = v.1;
                }
            }
        }
        match ans {
            std::usize::MAX => -1,
            _ => ans as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::first_uniq_char("leetcode".into()), 0);
        assert_eq!(super::Solution::first_uniq_char("loveleetcode".into()), 2);
        assert_eq!(super::Solution::first_uniq_char("a".into()), 0);
    }
}
