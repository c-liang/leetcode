//! # 205. 同构字符串
//! https://leetcode-cn.com/problems/isomorphic-strings/
//! 给定两个字符串 s 和 t，判断它们是否是同构的。
//! 如果 s 中的字符可以被替换得到 t ，那么这两个字符串是同构的。
//! 所有出现的字符都必须用另一个字符替换，同时保留字符的顺序。两个字符不能映射到同一个字符上，但字符可以映射自己本身。

//! # 解题思路
//! y=f(x) && x=f-1(y)

pub struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut chars_map = [None; 256];
        let mut mapped = [false; 256];
        for i in 0..s.len() {
            match chars_map[s.as_bytes()[i] as usize] {
                Some(v) => {
                    if v != t.as_bytes()[i] {
                        return false;
                    }
                }
                None => {
                    //不能存在相同的映射关系，比如"ab""aa" a->a  b不能再映射a
                    if mapped[t.as_bytes()[i] as usize] {
                        return false;
                    }
                    chars_map[s.as_bytes()[i] as usize] = Some(t.as_bytes()[i]);
                    mapped[t.as_bytes()[i] as usize] = true;
                }
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
            super::Solution::is_isomorphic("aa".into(), "bb".into()),
            true
        );
        assert_eq!(
            super::Solution::is_isomorphic("ab".into(), "ba".into()),
            true
        );
    }
}
