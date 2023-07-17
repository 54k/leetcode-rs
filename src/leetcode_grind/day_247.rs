// https://leetcode.com/problems/add-two-numbers-ii/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn add_two_numbers_stack(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut s1, mut s2) = (vec![], vec![]);
    while let Some(mut n) = l1 {
        s1.push(n.val);
        l1 = n.next.take();
    }
    while let Some(mut n) = l2 {
        s2.push(n.val);
        l2 = n.next.take();
    }
    let mut total = 0;
    let mut ans = None;
    while s1.len() > 0 || s2.len() > 0 {
        total += if let Some(p) = s1.pop() { p } else { 0 };
        total += if let Some(p) = s2.pop() { p } else { 0 };

        ans = Some(Box::new(ListNode {
            val: total % 10,
            next: ans,
        }));

        total /= 10
    }

    if total > 0 {
        ans = Some(Box::new(ListNode {
            val: total,
            next: ans,
        }));
    }

    ans
}

pub fn add_two_numbers_reverse_list(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    type N = Option<Box<ListNode>>;
    fn reverse(mut l: N) -> N {
        let mut prev = None;
        while let Some(mut cur) = l {
            let nxt = cur.next.take();
            cur.next = prev;
            prev = Some(cur);
            l = nxt;
        }
        prev
    }

    let mut l1 = reverse(l1);
    let mut l2 = reverse(l2);

    let mut head = None;
    let mut nxt = &mut head;
    let mut curry = 0;

    loop {
        let v1 = if let Some(mut n1) = l1 {
            l1 = n1.next.take();
            n1.val
        } else {
            0
        };

        let v2 = if let Some(mut n2) = l2 {
            l2 = n2.next.take();
            n2.val
        } else {
            0
        };

        let sum = v1 + v2 + curry;
        curry = sum / 10;

        let mut x = nxt.insert(Box::new(ListNode {
            val: sum % 10,
            next: None,
        }));
        nxt = &mut x.next;

        if l1.is_none() && l2.is_none() {
            if curry > 0 {
                let _ = nxt.insert(Box::new(ListNode {
                    val: curry,
                    next: None,
                }));
            }
            break;
        }
    }

    reverse(head)
}
