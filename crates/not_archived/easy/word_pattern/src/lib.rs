//! #290. 单词规律
//! 给定一种规律 pattern 和一个字符串 str ，判断 str 是否遵循相同的规律。

//! 这里的 遵循 指完全匹配，例如， pattern 里的每个字母和字符串 str 中的每个非空单词之间存在着双向连接的对应规律。

//! #解题思路
//! 分别进行hash，匹配出现的位置是否相同

use std::collections::HashMap;
pub struct Solution;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if pattern.is_empty() || s.is_empty() {
            return false;
        }
        let patterns: Vec<char> = pattern.chars().collect();
        let mut patterns_hash = HashMap::new();
        patterns_hash.insert(patterns[0], 0 as usize);

        let words: Vec<&str> = s.split(' ').collect();
        let mut words_hash = HashMap::new();
        words_hash.insert(words[0], 0 as usize);

        if patterns.len() != words.len() {
            return false;
        }
        for i in 1..patterns.len() {
            match (patterns_hash.get(&patterns[i]), words_hash.get(&words[i])) {
                (Some(j), Some(k)) => {
                    if j != k {
                        return false;
                    }
                }
                (None, None) => {
                    patterns_hash.insert(patterns[i], i);
                    words_hash.insert(words[i], i);
                }
                _ => return false,
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::Solution::word_pattern("aaa".into(), "aa aa aa aa".into()),
            false
        );
        assert_eq!(
            super::Solution::word_pattern("abca".into(), "aa ab ac aa".into()),
            true
        );
    }
}
