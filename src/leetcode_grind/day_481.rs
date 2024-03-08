// https://leetcode.com/problems/linked-list-frequency/description
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn frequencies_of_elements(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    use std::collections::HashMap;

    let mut head = head;
    let mut hm = HashMap::new();
    let mut it = head.as_ref();

    while let Some(n) = it {
        *hm.entry(n.val).or_insert(0) += 1;
        it = n.next.as_ref();
    }

    let mut dummy = Some(Box::new(ListNode::new(-1)));
    let mut d_it = dummy.as_mut();

    let mut it = head.as_mut();
    while let Some(mut n) = it {
        if hm.contains_key(&n.val) {
            if let Some(mut d) = d_it {
                d.next.insert(Box::new(ListNode::new(hm[&n.val])));
                d_it = d.next.as_mut();
            }
            hm.remove(&n.val);
        }
        it = n.next.as_mut();
    }
    dummy.unwrap().next.take()
}
