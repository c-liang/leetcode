//! # 61. 旋转链表
//! https://leetcode-cn.com/problems/rotate-list/
//!给定一个链表，旋转链表，将链表每个节点向右移动 k 个位置，其中 k 是非负数。

//! # 解题思路
//!算出链表的长度L，对k求模，获取新的右移位置，循环找到新的头结点，断链，把原始头拼到链表最后

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[allow(dead_code)]
    fn new_node(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }
}
pub struct Solution;
impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut dumy_head = Box::new(ListNode::new(0));
        dumy_head.next = head;
        let mut len = 0;
        {
            let mut h = &dumy_head;
            while h.next.is_some() {
                h = h.next.as_ref().unwrap();
                len += 1;
            }
        }
        let new_k = k % len;
        if new_k == 0 {
            return dumy_head.next;
        }
        let mut end = &mut dumy_head;
        for _ in 0..(len - new_k) {
            end = end.next.as_mut().unwrap();
        }
        //新的头节点，从这里断链
        let mut new_head = end.next.take();
        let mut h = &mut new_head;
        //这里有些重复，理论上在计算链表长度的时候就可以获取尾节点
        while h.is_some() {
            if h.as_ref().unwrap().next.is_none() {
                //找到尾部，插入原始的头结点
                h.as_mut().unwrap().next = dumy_head.next.take();
                break;
            }
            h = &mut h.as_mut().unwrap().next;
        }
        dumy_head.next = new_head;

        dumy_head.next
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::Solution::rotate_right(
                Some(Box::new(super::ListNode::new_node(
                    1,
                    Some(Box::new(super::ListNode::new_node(
                        1,
                        Some(Box::new(super::ListNode::new_node(3, None))),
                    ))),
                ))),
                1,
            ),
            Some(Box::new(super::ListNode::new_node(
                3,
                Some(Box::new(super::ListNode::new_node(
                    1,
                    Some(Box::new(super::ListNode::new_node(1, None))),
                ))),
            )))
        );
    }
}
