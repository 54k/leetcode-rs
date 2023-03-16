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
}

// https://leetcode.com/problems/merge-k-sorted-lists/description/
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    use std::cmp::*;
    use std::collections::BinaryHeap;

    impl PartialOrd<Self> for ListNode {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.val.cmp(&other.val))
        }
    }

    impl Ord for ListNode {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
    let mut queue = BinaryHeap::new();
    for l in lists.into_iter().flatten() {
        queue.push(Reverse(l));
    }
    let mut head = None;
    let mut tail = &mut head;
    while let Some(Reverse(mut n)) = queue.pop() {
        if n.next.is_some() {
            queue.push(Reverse(n.next.take().unwrap()));
        }
        let cur = tail.insert(n);
        tail = &mut cur.next;
    }
    head
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test343() {
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let l3 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 6, next: None })),
        }));

        println!("{:?}", merge_k_lists(vec![l1, l2, l3]));
    }
}
