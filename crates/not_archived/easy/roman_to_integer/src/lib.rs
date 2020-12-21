//! # 13. 罗马数字转整数
//! https://leetcode-cn.com/problems/roman-to-integer/
//!罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。
//! 字符          数值
//! I             1
//! V             5
//! X             10
//! L             50
//! C             100
//! D             500
//! M             1000
//! 例如， 罗马数字 2 写做 II ，即为两个并列的 1。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。
//! 通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，‘
//! 所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
//! I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
//! X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
//! C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
//! 给定一个罗马数字，将其转换成整数。输入确保在 1 到 3999 的范围内。

//! # 解题思路
//! 匹配罗马字符转为数字，小的数字如果在大数字的左边用则加上大数字减去小数字，否则一直相加

pub struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let romans: Vec<char> = s.chars().collect();
        let mut pre = Self::to_int(romans.first().unwrap());
        let mut pre_sum = pre;
        let mut ans = pre;
        for i in 1..romans.len() {
            let cur = Self::to_int(&romans[i]);
            match cur.cmp(&pre) {
                std::cmp::Ordering::Equal => {
                    pre_sum += cur;
                    ans += cur;
                }
                std::cmp::Ordering::Less => {
                    pre = cur;
                    pre_sum = pre;
                    ans += cur;
                }
                std::cmp::Ordering::Greater => {
                    pre = cur;
                    ans -= pre_sum;
                    ans += cur - pre_sum;
                    pre_sum = pre;
                }
            }
        }
        ans
    }
    fn to_int(c: &char) -> i32 {
        match c {
            &'I' => 1,
            &'V' => 5,
            &'X' => 10,
            &'L' => 50,
            &'C' => 100,
            &'D' => 500,
            &'M' => 1000,
            _ => panic!("invalid roman character"),
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::roman_to_int("I".into()), 1);
        assert_eq!(super::Solution::roman_to_int("M".into()), 1000);
        assert_eq!(super::Solution::roman_to_int("IV".into()), 4);
        assert_eq!(super::Solution::roman_to_int("MCMXCIV".into()), 1994);
        assert_eq!(super::Solution::roman_to_int("LVIII".into()), 58);
        assert_eq!(super::Solution::roman_to_int("MM".into()), 2000);
    }
}
