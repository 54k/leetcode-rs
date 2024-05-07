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

// https://leetcode.com/problems/double-a-number-represented-as-a-linked-list/description
pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut st = vec![];
    while let Some(mut h) = head {
        st.push(h.val);
        head = h.next.take();
    }
    let mut val = 0;
    let mut new_tail: Option<Box<ListNode>> = None;
    while st.len() > 0 || val != 0 {
        let mut ln = ListNode {
            next: new_tail.take(),
            val: 0,
        };
        if st.len() > 0 {
            val += st.pop().unwrap() * 2;
        }
        ln.val = val % 10;
        val /= 10;
        new_tail = Some(Box::new(ln));
    }
    new_tail
}
