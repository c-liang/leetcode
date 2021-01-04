//! # 509. 斐波那契数
//! https://leetcode-cn.com/problems/fibonacci-number/
//! 斐波那契数，通常用 F(n) 表示，形成的序列称为 斐波那契数列 。该数列由 0 和 1 开始，后面的每一项数字都是前面两项数字的和

//! # 解题思路
//! f(n) = f(n-1)+f(n-2) f(0)=0, f(1)=1

pub struct Solution;
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let mut ans = 0;
        let mut n1 = 0;
        let mut n2 = 1;
        for _ in 2..=n {
            let v = n2;
            ans = n1 + n2;
            n1 = v;
            n2 = ans;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::fib(0), 0);
        assert_eq!(super::Solution::fib(1), 1);
        assert_eq!(super::Solution::fib(4), 3);
    }
}
