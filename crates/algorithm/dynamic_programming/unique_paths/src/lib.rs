//! # 62. 不同路径
//! 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。
//! 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。
//! 问总共有多少条不同的路径？

//! f(i,j)=f(i−1,j)+f(i,j−1)
//! f(0,j) = = f(i,0) = 1
pub struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut ans = vec![vec![1; m as usize]; n as usize];
        for i in 1..n as usize {
            for j in 1..m as usize {
                ans[i][j] = ans[i - 1][j] + ans[i][j - 1];
            }
        }
        ans[n as usize - 1][m as usize - 1]
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::unique_paths(1, 1), 1);
        assert_eq!(super::Solution::unique_paths(3, 2), 3);
        assert_eq!(super::Solution::unique_paths(7, 3), 28);
    }
}
