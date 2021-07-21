use std::cell::RefCell;
use std::collections::VecDeque;
//python3
//# Definition for a binary tree node.
// # class TreeNode:
// #     def __init__(self, x):
// #         self.val = x
// #         self.left = None
// #         self.right = None
//
// class Solution:
//     def isSubStructure(self, A: TreeNode, B: TreeNode) -> bool:
//         if not A or not B:
//             return False
//         def recur(a, b):
//             if not b:
//                 return True
//             if not a or a.val != b.val:
//                 return False
//             return recur(a.left, b.left) and recur(a.right, b.right)
//         return recur(A, B) or self.isSubStructure(A.left, B) or self.isSubStructure(A.right, B)
use std::rc::Rc;

impl Solution {
    pub fn is_sub_structure(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if b == None { return false; }
        let mut queue = VecDeque::new();
        queue.push_back(a);
        while !queue.is_empty() {
            if let Some(a) = queue.pop_front().unwrap() {
                if Self::compare_to_end(Some(Rc::clone(&a)), b.clone()) {
                    return true;
                } else {
                    queue.push_back(a.borrow().left.clone());
                    queue.push_back(a.borrow().right.clone());
                }
            }
        }

        false
    }

    pub fn compare_to_end(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (a, b) {
            (Some(node_a), Some(node_b)) => {
                if node_a.borrow().val == node_b.borrow().val {
                    return Self::compare_to_end(node_a.borrow().left.clone(), node_b.borrow().left.clone()) &&
                        Self::compare_to_end(node_a.borrow().right.clone(), node_b.borrow().right.clone());
                } else {
                    return false;
                }
            },
            (None, Some(node_b)) => {
                return false;
            },
            (_, None) => {
                return true;
            },
        }

        false
    }
}
