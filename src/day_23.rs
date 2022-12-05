#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn safe(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut s = &head;
        let mut f = &head;
        while f.is_some() && f.as_ref().unwrap().next.is_some() {
            f = &f.as_ref().unwrap().next.as_ref().unwrap().next;
            s = &s.as_ref().unwrap().next;
        }
        s.clone()
    }

    fn un_safe(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        unsafe {
            let mut s = Box::into_raw(head.unwrap());
            let mut f = s.as_ref();

            while f.is_some() && f.unwrap().next.is_some() {
                f = f.unwrap().next.as_ref().unwrap().next.as_deref();
                s = Box::into_raw((*s).next.take().unwrap());
            }

            Some(Box::from_raw(s))
        }
    }

    un_safe(head)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test96() {
        println!(
            "{:?}",
            middle_node(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: None //Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            })))
        );
    }
}
