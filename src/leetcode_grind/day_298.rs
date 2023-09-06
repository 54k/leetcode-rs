// https://leetcode.com/problems/split-linked-list-in-parts/description/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn split_list_to_parts(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    let mut n = 0;
    let mut next = head.as_ref();
    while let Some(nnext) = next {
        next = nnext.next.as_ref();
        n += 1;
    }

    let mut ans = vec![None; k as usize];
    let width = n / k as usize;
    let rem = n % k as usize;

    for i in 0..k as usize {
        ans[i] = head.take();
        let mut cur = &mut ans[i];
        for _ in 0..width + (if i < rem { 1 } else { 0 }) {
            if let Some(c) = cur {
                cur = &mut c.next;
            }
        }
        head = cur.take();
    }

    ans
}
