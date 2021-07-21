//python3
// # Definition for singly-linked list.
//# class ListNode:
//#     def __init__(self, x):
//#         self.val = x
//#         self.next = None
//
//class Solution:
//def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
//cur = dump = ListNode(0)
//while l1 and l2:
//if l1.val > l2.val:
//cur.next = ListNode(l2.val)
//l2 = l2.next
//else:
//cur.next = ListNode(l1.val)
//l1 = l1.next
//cur = cur.next
//cur.next = l1 if l1 else l2
//return dump.next
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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut dum = Some(Box::new(ListNode { val: 0, next: None }));
        let mut head = dum.as_mut();
        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val > l2.as_ref().unwrap().val {
                let next = l2.as_mut().unwrap().next.take();
                head.as_mut().unwrap().next = l2;
                l2 = next;
            } else {
                let next = l1.as_mut().unwrap().next.take();
                head.as_mut().unwrap().next = l1;
                l1 = next;
            }
            head = head.unwrap().next.as_mut();
        }
        head.as_mut().unwrap().next = if l1.is_some() { l1 } else { l2 };
        dum.unwrap().next
    }
}