//! # 103. 二叉树的锯齿形层序遍历
//! https://leetcode-cn.com/problems/binary-tree-zigzag-level-order-traversal/
//! 给定一个二叉树，返回其节点值的锯齿形层序遍历。（即先从左往右，再从右往左进行下一层遍历，以此类推，层与层之间交替进行）。
//! 例如：
//! 给定二叉树 [3,9,20,null,null,15,7],
//!     3
//!    / \
//!   9  20
//!     /  \
//!    15   7
//! 返回锯齿形层序遍历如下：
//! [
//!   [3],
//!   [20,9],
//!   [15,7]
//! ]

//! # 解题思路
//! 采用广度优先遍历，隔层结果翻转

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::rc::Rc;
use std::{cell::RefCell, vec};
pub struct Solution;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut level_nodes = vec![root; 1];
        let mut level = 0;
        loop {
            if level_nodes.is_empty() {
                break;
            }
            let mut next_level = Vec::new();
            let mut cur_level_val = Vec::new();
            for v in level_nodes {
                if let Some(node) = v {
                    cur_level_val.push(node.borrow().val);
                    next_level.push(node.borrow().left.clone());
                    next_level.push(node.borrow().right.clone());
                }
            }
            level += 1;
            level_nodes = next_level;
            if cur_level_val.is_empty() {
                continue;
            }

            if level % 2 == 0 {
                ans.push(cur_level_val.into_iter().rev().collect());
            } else {
                ans.push(cur_level_val);
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
