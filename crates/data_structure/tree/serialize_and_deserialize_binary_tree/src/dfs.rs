use super::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct CodecDFS;

impl CodecDFS {
    pub fn new() -> Self {
        CodecDFS {}
    }
    /// 深度优先，先序遍历
    /// DFS trave
    /// BNF Expr
    /// T->'None' | val,T,T | (val)
    /// (val) means node without left and right child node, 'None' this is a empty node
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return String::from("None");
        }
        let node = root.unwrap();
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return format!("({})", node.borrow().val.to_string());
        }
        format!(
            "{},{},{}",
            node.borrow().val.to_string(),
            Self::serialize(&self, node.borrow().left.clone()),
            Self::serialize(&self, node.borrow().right.clone())
        )
    }
    fn deserialize_impl(&self, data: &Vec<&str>, cur: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        //check empty
        if *cur == data.len() {
            return None;
        }
        if data[*cur] == "None" {
            *cur += 1;
            return None;
        } else if data[*cur].as_bytes()[0] == b'(' {
            let mut ans = 0;
            let mut flags = 1;
            for c in &data[*cur].as_bytes()[1..data[*cur].as_bytes().len() - 1] {
                if *c == b'-' {
                    flags = -1;
                    continue;
                }
                ans *= 10;
                ans += (*c - b'0') as i32;
            }
            *cur += 1;
            return Some(Rc::new(RefCell::new(TreeNode::new(ans * flags))));
        }
        let mut val = 0;
        let mut flags = 1;
        for c in data[*cur].as_bytes() {
            if *c == b'-' {
                flags = -1;
                continue;
            }
            val *= 10;
            val += (*c - b'0') as i32;
        }
        let node = Rc::new(RefCell::new(TreeNode::new(val * flags)));
        *cur += 1;
        let left = Self::deserialize_impl(&self, data, cur);
        node.borrow_mut().left = left;
        let right = Self::deserialize_impl(&self, data, cur);
        node.borrow_mut().right = right;
        Some(node)
    }
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let vals: Vec<_> = data.split(",").collect();
        let mut cur = 0;
        Self::deserialize_impl(&self, &vals, &mut cur)
    }
}
impl super::TreeCodec for CodecDFS {
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.serialize(root)
    }
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        self.deserialize(data)
    }
}
