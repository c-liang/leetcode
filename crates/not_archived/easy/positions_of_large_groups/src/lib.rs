//! # 830. 较大分组的位置
//! https://leetcode-cn.com/problems/positions-of-large-groups/

//! # 思路
//! 一次遍历，记录上一次的字符
pub struct Solution;
impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        if s.len() < 3 {
            return vec![];
        }
        let bytes = s.as_bytes();
        let mut ans = vec![];
        let mut pre_c = bytes[0];
        let mut start = 0;
        let mut end = 0;
        for i in 1..bytes.len() {
            if bytes[i] == pre_c {
                end = i;
            } else {
                if (end - start + 1) >= 3 {
                    ans.push(vec![start as i32, end as i32]);
                }
                start = i;
                end = i;
                pre_c = bytes[i];
            }
        }
        if (end - start + 1) >= 3 {
            ans.push(vec![start as i32, end as i32]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::Solution::large_group_positions("abbxxxxzzzyycccccgfesfdsaeee".into()),
            vec![vec![3, 6], vec![7, 9], vec![12, 16], vec![25, 27]]
        );
    }
}
