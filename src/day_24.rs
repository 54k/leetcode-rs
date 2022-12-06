#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn un_safe(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        use std::ptr::*;
        if head.is_none() {
            return None;
        }
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
}
