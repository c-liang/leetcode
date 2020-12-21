//！ # 316. 去除重复字母
//! https://leetcode-cn.com/problems/remove-duplicate-letters/
/// 给你一个字符串 s ，请你去除字符串中重复的字母，使得每个字母只出现一次。需保证 返回结果的字典序最小（要求不能打乱其他字符的相对位置）。

///! # 解题思路
///! 计算每个字符出现的次数，利用递增栈

pub struct Solution;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut letters = [(0, false); 26];
        for c in s.as_bytes() {
            letters[(*c - b'a') as usize].0 += 1;
        }
        let mut result: Vec<u8> = Vec::new();
        for c in s.as_bytes() {
            let idx = (*c - b'a') as usize;
            if letters[idx].0 > 0 {
                //already exist
                if letters[idx].1 {
                    letters[idx].0 -= 1;
                    continue;
                }
                while !result.is_empty() {
                    let last = result.last().unwrap();
                    // 确保当前字符是否小于栈内字符并去出栈的字符后面还会有出现
                    if last < c || letters[(last - b'a') as usize].0 < 1 {
                        break;
                    }
                    // 满足条件，最后大的字符出栈
                    letters[(last - b'a') as usize].1 = false;
                    result.pop();
                }
                result.push(*c);
                letters[idx].0 -= 1;
                letters[idx].1 = true;
            }
        }
        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::Solution::remove_duplicate_letters("bcabc".into()),
            String::from("abc")
        );
        assert_eq!(
            super::Solution::remove_duplicate_letters("cbacdcbc".into()),
            String::from("acdb")
        );
        assert_eq!(
            super::Solution::remove_duplicate_letters("".into()),
            String::from("")
        );
    }
}
