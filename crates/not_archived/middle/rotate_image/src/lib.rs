//! # 48. 旋转图像
//! https://leetcode-cn.com/problems/rotate-image/
//! 给定一个 n × n 的二维矩阵表示一个图像。
//! 将图像顺时针旋转 90 度。
//! 说明：
//! 你必须在原地旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要使用另一个矩阵来旋转图像。

//! # 解题思路
//! 从外层往内层递归旋转 最外层旋转规律(0<=i<n) [0,i]->[i,n]->[n,n-i]->[n-i,0]->[0,i]

pub struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Self::rotate_recursive(matrix, 0, 0, matrix.len())
    }
    pub fn rotate_recursive(matrix: &mut Vec<Vec<i32>>, x: usize, y: usize, len: usize) {
        if len <= 1 {
            return;
        }
        for i in 0..len - 1 {
            //[0,i]->[i,n]->[n,n-i]->[n-i,0]->[0,i]
            let v = matrix[y][i + x];
            matrix[y][i + x] = matrix[len - 1 - i + y][x]; //[n-i,0]
            matrix[len - 1 - i + y][x] = matrix[len - 1 + y][len - 1 - i + x]; //[n,n-i]
            matrix[len - 1 + y][len - 1 - i + x] = matrix[i + y][len - 1 + x]; //[i,n]
            matrix[i + y][len - 1 + x] = v; //[0,i]
        }
        Self::rotate_recursive(matrix, x + 1, y + 1, len - 2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut v = [[1, 2, 3].into(), [4, 5, 6].into(), [7, 8, 9].into()].into();
        super::Solution::rotate(&mut v);
        assert_eq!(2 + 2, 4);
    }
}
