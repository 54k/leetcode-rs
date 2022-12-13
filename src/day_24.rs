#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn un_safe(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        use std::ptr::*;
        head.as_ref()?;
        unsafe {
            let mut idx = 1;
            let mut odd_head: *mut ListNode = null_mut();
            let mut odd_tail: *mut ListNode = null_mut();
            let mut even_head: *mut ListNode = null_mut();
            let mut even_tail: *mut ListNode = null_mut();

            while let Some(mut node) = head.take() {
                head = node.next.take();
                let raw_node = Box::into_raw(node);
                if idx % 2 > 0 {
                    if odd_head.is_null() {
                        odd_head = raw_node;
                        odd_tail = raw_node;
                    } else {
                        (*odd_tail).next = Some(Box::from_raw(raw_node));
                        odd_tail = raw_node;
                    }
                } else if even_head.is_null() {
                    even_head = raw_node;
                    even_tail = raw_node;
                } else {
                    (*even_tail).next = Some(Box::from_raw(raw_node));
                    even_tail = raw_node;
                }
                idx += 1;
            }

            if !even_head.is_null() {
                (*odd_tail).next = Some(Box::from_raw(even_head));
            }

            Some(Box::from_raw(odd_head))
        }
    }

    fn safe(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_head = Some(head.take()?); // return early if list is empty
        head = odd_head.as_mut()?.next.take();
        let mut odd_tail = &mut odd_head;

        if head.is_none() {
            return odd_head;
        }

        let mut even_head = head.take();
        head = even_head.as_mut()?.next.take();
        let mut even_tail = &mut even_head;

        let mut is_odd = true;

        while let Some(mut node) = head.take() {
            head = node.next.take();
            if is_odd {
                odd_tail.as_mut()?.next = Some(node);
                odd_tail = &mut odd_tail.as_mut()?.next;
            } else {
                even_tail.as_mut()?.next = Some(node);
                even_tail = &mut even_tail.as_mut()?.next;
            }
            is_odd = !is_odd;
        }

        odd_tail.as_mut()?.next = even_head;

        odd_head
    }

    safe(head)
}

#[allow(dead_code)]
pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    fn long(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        fn list_size(head: &Option<Box<ListNode>>) -> usize {
            let mut res = 0;
            let mut next = head;
            while let Some(n) = next {
                next = &n.next;
                res += 1;
            }
            res
        }

        let k = k as usize;
        let size = list_size(&head);
        let slice_size = size / k;
        let rem = size % k;

        if size == 0 {
            return vec![None; k];
        }

        let mut heads: Vec<Option<Box<ListNode>>> = vec![None; k];
        let mut tail: &mut Option<Box<ListNode>> = &mut None;

        let mut slot = 0usize;
        let mut cur = 0usize;

        while let Some(mut n) = head.take() {
            unsafe {
                head = n.next.take();

                if cur == 0 {
                    heads[slot] = Some(n);
                    tail = &mut heads[slot];
                } else {
                    tail.as_mut().unwrap().next = Some(n);
                    tail = &mut tail.as_mut().unwrap().next;
                }
            }

            cur += 1;
            let bound = slice_size + (if slot < rem { 1 } else { 0 });
            if cur == bound {
                slot += 1;
                cur = 0;
            }
        }

        heads
    }

    fn short(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut size = 0;
        let mut next = &head;
        while let Some(n) = next {
            size += 1;
            next = &n.next;
        }

        let k = k as usize;
        let width = size / k;
        let rem = size % k;
        let mut ans = vec![None; k];

        for i in 0..k {
            ans[i] = head.take();
            let mut next = &mut ans[i];
            for _ in 0..width + (if i < rem { 1 } else { 0 }) {
                if let Some(n) = next {
                    next = &mut n.next;
                }
            }
            head = next.take();
        }
        ans
    }

    short(head, k)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test100() {
        println!(
            "{:?}",
            odd_even_list(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            })))
        );

        println!(
            "{:?}",
            odd_even_list(Some(Box::new(ListNode { val: 1, next: None })))
        );
    }

    #[test]
    fn test101() {
        println!(
            "{:?}",
            split_list_to_parts(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 2, next: None }))
                })),
                4
            )
        );

        println!(
            "{:?}",
            split_list_to_parts(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode {
                                        val: 6,
                                        next: Some(Box::new(ListNode {
                                            val: 7,
                                            next: Some(Box::new(ListNode {
                                                val: 8,
                                                next: Some(Box::new(ListNode {
                                                    val: 9,
                                                    next: Some(Box::new(ListNode {
                                                        val: 10,
                                                        next: None
                                                    }))
                                                }))
                                            }))
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                })),
                3
            )
        );

        println!(
            "{:?}",
            split_list_to_parts(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode {
                                        val: 6,
                                        next: Some(Box::new(ListNode {
                                            val: 7,
                                            next: Some(Box::new(ListNode {
                                                val: 8,
                                                next: Some(Box::new(ListNode {
                                                    val: 9,
                                                    next: Some(Box::new(ListNode {
                                                        val: 10,
                                                        next: Some(Box::new(ListNode {
                                                            val: 11,
                                                            next: None
                                                        }))
                                                    }))
                                                }))
                                            }))
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                })),
                3
            )
        );

        println!(
            "{:?}",
            split_list_to_parts(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
                    }))
                })),
                2
            )
        );

        println!(
            "{:?}",
            split_list_to_parts(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 4, next: None }))
                        }))
                    }))
                })),
                2
            )
        );

        println!(
            "{:?}",
            split_list_to_parts(Some(Box::new(ListNode { val: 1, next: None })), 3)
        );
    }
}
