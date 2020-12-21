//! #649. Dota2 参议院
//!https://leetcode-cn.com/problems/dota2-senate/
//!Dota2 的世界里有两个阵营：Radiant(天辉)和 Dire(夜魇)
//!Dota2 参议院由来自两派的参议员组成。现在参议院希望对一个 Dota2 游戏里的改变作出决定。他们以一个基于轮为过程的投票进行。在每一轮中，每一位参议员都可以行使两项权利中的一项：
//!禁止一名参议员的权利：
//!参议员可以让另一位参议员在这一轮和随后的几轮中丧失所有的权利。
//!宣布胜利：
//!如果参议员发现有权利投票的参议员都是同一个阵营的，他可以宣布胜利并决定在游戏中的有关变化。
//!给定一个字符串代表每个参议员的阵营。字母 “R” 和 “D” 分别代表了 Radiant（天辉）和 Dire（夜魇）。然后，如果有 n 个参议员，给定字符串的大小将是 n。
//!以轮为基础的过程从给定顺序的第一个参议员开始到最后一个参议员结束。这一过程将持续到投票结束。所有失去权利的参议员将在过程中被跳过。
//!假设每一位参议员都足够聪明，会为自己的政党做出最好的策略，你需要预测哪一方最终会宣布胜利并在 Dota2 游戏中决定改变。输出应该是 Radiant 或 Dire。

//! #解题思路
//!采用贪心规则，当前参议员总是执行权力一，让后一个紧跟的另一派的参议员失去跳票权力

use std::collections::VecDeque;
pub struct Solution;
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiant = VecDeque::new();
        let mut dire = VecDeque::new();
        let len = senate.len();
        for i in 0..len {
            if senate.as_bytes()[i] == b'R' {
                radiant.push_back(i);
            } else {
                dire.push_back(i);
            }
        }

        while !radiant.is_empty() && !dire.is_empty() {
            if radiant.front().unwrap() < dire.front().unwrap() {
                radiant.push_back(radiant.front().unwrap() + len);
            } else {
                dire.push_back(dire.front().unwrap() + len);
            }
            radiant.pop_front();
            dire.pop_front();
        }
        match radiant.is_empty() {
            true => "Dire".into(),
            false => "Radiant".into(),
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::Solution::predict_party_victory("RRDD".into()),
            String::from("Radiant")
        );
        assert_eq!(
            super::Solution::predict_party_victory("DDRRRD".into()),
            String::from("Dire")
        );
    }
}