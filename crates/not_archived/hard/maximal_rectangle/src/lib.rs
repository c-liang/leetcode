//! # 85. 最大矩形
//! https://leetcode-cn.com/problems/maximal-rectangle/
pub struct Solution;
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let mut ans = 0;
        let mut area = vec![0; matrix[0].len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == '1' {
                    area[j] = area[j] + 1;
                } else {
                    area[j] = 0;
                }
            }
            ans = ans.max(Self::largest_rectangle_area(&area));
        }
        ans
    }
    pub fn largest_rectangle_area_stack(heights: &Vec<i32>) -> i32 {
        let mut ans = 0;
        let (mut left, mut right, mut stack) = (
            vec![-1; heights.len()],
            vec![heights.len() as i32; heights.len()],
            vec![],
        );
        for i in 0..heights.len() {
            while !stack.is_empty() && heights[*stack.last().unwrap() as usize] > heights[i] {
                right[*stack.last().unwrap() as usize] = i as i32;
                stack.pop();
            }
            if stack.is_empty() {
                left[i] = -1;
            } else {
                left[i] = *stack.last().unwrap();
            }
            stack.push(i as i32);
        }
        for i in 0..heights.len() {
            ans = ans.max((right[i] - left[i] - 1) * heights[i]);
        }
        ans
    }
    pub fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..heights.len() {
            let mut min = heights[i];
            for j in i..heights.len() {
                min = min.min(heights[j]);
                ans = ans.max(min * (j - i + 1) as i32)
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
