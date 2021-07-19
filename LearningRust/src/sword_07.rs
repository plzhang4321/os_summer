//python3
//# Definition for a binary tree node.
// # class TreeNode:
// #     def __init__(self, x):
// #         self.val = x
// #         self.left = None
// #         self.right = None
//
// class Solution:
//     def buildTree(self, preorder: List[int], inorder: List[int]) -> TreeNode:
//         def recur(root, left, right):
//             if left > right:
//                 return
//             node = TreeNode(preorder[root])
//             i = dic[preorder[root]]
//             node.left = recur(root + 1, left, i - 1)
//             node.right = recur(i - left + root + 1, i + 1, right)
//             return node
//
//         dic = dict()
//         for i in range(len(inorder)):
//             dic[inorder[i]] = i
//         return recur(0, 0, len(preorder) - 1)
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut neworder: HashMap<i32, usize>= HashMap::new();
        for i in 0..inorder.len() {
            neworder.insert(inorder[i], i);
        }
        Solution::recur(&neworder, &preorder, 0, 0, (inorder.len() - 1) as i32)
    }
    fn recur(order: &HashMap<i32, usize>, preorder: &Vec<i32>, root: i32, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }
        let val = preorder[root as usize];
        let i = order[&val] as i32;
        let left_node = Solution::recur(&order, &preorder, root + 1, left, i - 1);
        let right_node = Solution::recur(&order, &preorder, i + 1 - left + root, i + 1, right);
        Some(Rc::new(RefCell::new(TreeNode{
            val,
            left: left_node,
            right: right_node
        })))

    }
}
