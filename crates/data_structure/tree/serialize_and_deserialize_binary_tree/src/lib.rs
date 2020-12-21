//! # 297. 二叉树的序列化与反序列化
//! https://leetcode-cn.com/problems/serialize-and-deserialize-binary-tree/
//! 序列化是将一个数据结构或者对象转换为连续的比特位的操作，进而可以将转换后的数据存储在一个文件或者内存中，
//! 同时也可以通过网络传输到另一个计算机环境，采取相反方式重构得到原数据。

use std::cell::RefCell;
use std::rc::Rc;
mod dfs;
mod dfs_mid;
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
    pub fn new_node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        TreeNode { val, left, right }
    }
}
pub trait TreeCodec {
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String;

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>>;
}
pub struct Codec {
    codec: Box<dyn TreeCodec>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    pub fn new() -> Self {
        Codec {
            // codec: Box::new(dfs::CodecDFS::new()),
            codec: Box::new(dfs_mid::CodecDFSMid::new()),
        }
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.codec.serialize(root)
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        self.codec.deserialize(data)
    }
}

#[cfg(test)]
mod tests {
    use super::Codec;
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;
    #[test]
    fn it_works() {
        //Rc<RefCell<TreeNode>>>
        let node = Rc::new(RefCell::new(TreeNode::new_node(
            -1,
            Some(Rc::new(RefCell::new(TreeNode::new_node(
                2,
                Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                None,
            )))),
            Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        )));
        let seri = Codec::new().serialize(Some(node.clone()));
        println!("{}", seri.clone());
        let deser = Codec::new().deserialize(seri);
        println!("{}", Codec::new().serialize(deser.clone()));
        // println!("{:?}", deser.clone());
        assert_eq!(Some(node), deser);
    }
}
