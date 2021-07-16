// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut head = head;
        while let Some(cur) = head {
            res.push(cur.val);
            head = cur.next;
        }
        res.reverse();
        res
    }
}
//python3
//# Definition for singly-linked list.
// # class ListNode:
// #     def __init__(self, x):
// #         self.val = x
// #         self.next = None
//
// class Solution:
//     def reversePrint(self, head: ListNode) -> List[int]:
//         stack = []
//         while head:
//             stack.append(head.val)
//             head = head.next
//         return stack[::-1]