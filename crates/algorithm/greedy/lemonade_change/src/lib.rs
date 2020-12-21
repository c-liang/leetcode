//! # 860. 柠檬水找零
//!https://leetcode-cn.com/problems/lemonade-change/
//!在柠檬水摊上，每一杯柠檬水的售价为 5 美元。
//!顾客排队购买你的产品，（按账单 bills 支付的顺序）一次购买一杯。
//!每位顾客只买一杯柠檬水，然后向你付 5 美元、10 美元或 20 美元。你必须给每个顾客正确找零，也就是说净交易是每位顾客向你支付 5 美元。
//!注意，一开始你手头没有任何零钱。
//!如果你能给每位顾客正确找零，返回 true ，否则返回 false 。

//!「贪心的本质是选择每一阶段的局部最优，从而达到全局最优」
//！贪心条件每次找找零找尽可能找最大的

pub struct Solution;
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut fives, mut tens) = (0, 0);
        for bill in &bills {
            match bill {
                &20 => {
                    if tens > 0 && fives > 0 {
                        tens -= 1;
                        fives -= 1;
                    } else if fives >= 3 {
                        fives -= 3;
                    } else {
                        return false;
                    }
                }
                &10 => {
                    if fives > 0 {
                        fives -= 1;
                    } else {
                        return false;
                    }
                    tens += 1;
                }
                &5 => {
                    fives += 1;
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
        assert_eq!(super::Solution::lemonade_change([5, 5, 10].into()), true);
        assert_eq!(super::Solution::lemonade_change([10, 10].into()), false);
        assert_eq!(
            super::Solution::lemonade_change([5, 5, 10, 10, 20].into()),
            false
        );
    }
}
