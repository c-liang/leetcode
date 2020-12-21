use super::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct CodecDFSMid;

impl CodecDFSMid {
    pub fn new() -> Self {
        CodecDFSMid {}
    }
    /// 深度优先，中序遍历
    /// LL(1)解析BNF，省略生成token
    /// DFS trave
    /// BNF Expr
    /// T->'X' | (T)val(T)
    /// (val) means node without left and right child node, 'None' this is a empty node
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return String::from("X");
        }
        let node = root.unwrap();
        format!(
            "({}){}({})",
            self.serialize(node.borrow().left.clone()),
            node.borrow().val,
            self.serialize(node.borrow().right.clone())
        )
    }
    fn parse_sub(&self, data: &str, cur: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        *cur += 1;
        let node = self.parse(data, cur);
        *cur += 1;
        node
    }
    fn parse_int(&self, data: &str, cur: &mut usize) -> i32 {
        let (mut ans, mut sign) = (0, 1);
        if data.as_bytes()[*cur] == b'-' {
            *cur += 1;
            sign = -1;
        }
        while data.as_bytes()[*cur].is_ascii_digit() {
            ans *= 10;
            ans += (data.as_bytes()[*cur] - b'0') as i32;
            *cur += 1;
        }
        ans * sign
    }
    fn parse(&self, data: &str, cur: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        if data.as_bytes()[*cur] == b'X' {
            *cur += 1;
            return None;
        }
        let left = self.parse_sub(data, cur);
        let node = Rc::new(RefCell::new(TreeNode::new(self.parse_int(data, cur))));
        let right = self.parse_sub(data, cur);
        node.borrow_mut().left = left;
        node.borrow_mut().right = right;
        return Some(node);
    }
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur = 0;
        self.parse(&data[..], &mut cur)
    }
}
impl super::TreeCodec for CodecDFSMid {
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.serialize(root)
    }
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        self.deserialize(data)
    }
}
