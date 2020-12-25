//! # 455. 分发饼干
//! https://leetcode-cn.com/problems/assign-cookies/
//! 假设你是一位很棒的家长，想要给你的孩子们一些小饼干。但是，每个孩子最多只能给一块饼干。
//! 对每个孩子 i，都有一个胃口值 g[i]，这是能让孩子们满足胃口的饼干的最小尺寸；并且每块饼干 j，都有一个尺寸 s[j] 。
//! 如果 s[j] >= g[i]，我们可以将这个饼干 j 分配给孩子 i ，这个孩子会得到满足。你的目标是尽可能满足越多数量的孩子，并输出这个最大数值。

//! # 解题思路
//! 排序后采用贪心规则，最小的饼干满足胃口最小的孩子

pub struct Solution;
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut cur_cookie = 0;
        let mut ret = 0;
        for i in 0..g.len() {
            for j in cur_cookie..s.len() {
                cur_cookie = j + 1;
                if s[j] >= g[i] {
                    ret += 1;
                    break;
                }
            }
            if cur_cookie == s.len() {
                break;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
