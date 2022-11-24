// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// Maintain two pointers and update one with a delay of n steps.
#[allow(dead_code)]
pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    fn rec(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
        match head {
            None => (None, 1),
            Some(mut node) => {
                let (prev, num) = rec(node.next.take(), n);

                if num == n {
                    (prev, num + 1)
                } else {
                    node.next = prev;
                    (Some(node), num + 1)
                }
            }
        }
    }

    rec(head, n).0
}

#[allow(dead_code)]
pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn size(mut head: &mut Option<Box<ListNode>>) -> i32 {
        let mut size = 0;
        while let Some(h) = head {
            head = &mut h.next;
            size += 1;
        }
        size
    }

    let mid = size(&mut head) / 2;
    let mut pre_mid = &mut head;
    for _ in 0..mid - 1 {
        pre_mid = &mut pre_mid.as_mut().unwrap().next;
    }
    let mid_list = pre_mid.as_mut()?.next.take();
    pre_mid.as_mut()?.next = mid_list?.next.take();
    head
}

#[allow(dead_code)]
pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut before_head = ListNode { val: 0, next: None };
    let mut after_head = ListNode { val: 0, next: None };

    let mut before = &mut before_head;
    let mut after = &mut after_head;

    while let Some(mut h) = head {
        let next = h.next.take();
        if h.val < x {
            before.next = Some(h);
            before = before.next.as_mut().unwrap();
        } else {
            after.next = Some(h);
            after = after.next.as_mut().unwrap();
        }
        head = next;
    }

    before.next = after_head.next;

    before_head.next
}

#[cfg(test)]
mod test {
    use crate::day_12::*;

    #[test]
    fn test69() {
        println!(
            "{:?}",
            remove_nth_from_end(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: Some(Box::new(ListNode { val: 7, next: None }))
                            })),
                        })),
                    })),
                })),
                2
            )
        );
    }

    #[test]
    fn test70() {
        println!(
            "{:?}",
            partition(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 2,
                                next: Some(Box::new(ListNode { val: 5, next: None }))
                            })),
                        })),
                    })),
                })),
                3
            )
        );
    }

    #[test]
    fn test71() {
        println!(
            "{:?}",
            delete_middle(Some(Box::new(ListNode { val: 1, next: None })))
        );
    }
}
