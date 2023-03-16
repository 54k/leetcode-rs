#[allow(dead_code)]
pub fn is_ugly(mut n: i32) -> bool {
    fn divide(divider: i32, divisible: &mut i32) {
        while *divisible % divider == 0 {
            *divisible /= divider
        }
    }
    if n <= 0 {
        return false;
    }
    const DIVIDERS: [i32; 3] = [2, 3, 5];
    for d in DIVIDERS {
        divide(d, &mut n);
    }
    n == 1
}

#[allow(dead_code)]
pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack = haystack.chars().collect::<Vec<_>>();
    let needle = needle.chars().collect::<Vec<_>>();
    for (i, &ch) in haystack.iter().enumerate() {
        if ch == needle[0] {
            let mut j = i + 1;
            while j < haystack.len() && j - i < needle.len() && haystack[j] == needle[j - i] {
                j += 1;
            }
            if j - i == needle.len() {
                return i as i32;
            }
        }
    }
    -1
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(clippy::missing_safety_doc)]
#[allow(dead_code)]
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    fn shit_code_solution(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        if head.as_ref().unwrap().next.is_none() {
            return true;
        }

        unsafe {
            let first_half = Box::into_raw(head.unwrap());

            unsafe fn get_mid(head: *mut ListNode) -> *mut ListNode {
                let mut slow = head;
                let mut fast = head;
                while let Some(next) = (*fast).next.as_mut() {
                    if let Some(next) = next.next.as_mut() {
                        fast = next.as_mut() as *mut ListNode;
                        slow = (*slow).next.as_mut().unwrap().as_mut() as *mut ListNode;
                    } else {
                        break;
                    }
                }
                slow
            }

            fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
                let mut prev = None;
                let mut current = head;
                while let Some(mut current_val) = current {
                    let next = current_val.next.take();
                    current_val.next = prev;
                    prev = Some(current_val);
                    current = next;
                }
                prev
            }

            // split and reverse
            let mid = get_mid(first_half);
            let second_half = Box::into_raw(reverse((*mid).next.take()).unwrap());

            let first_half = Some(Box::from_raw(first_half));
            let second_half = Some(Box::from_raw(second_half));

            let mut p1 = first_half.as_ref();
            let mut p2 = second_half.as_ref();

            while let Some(s) = p2.as_ref() {
                if let Some(f) = p1.as_ref() {
                    if f.val != s.val {
                        return false;
                    }
                    p1 = f.next.as_ref();
                    p2 = s.next.as_ref();
                } else {
                    return false;
                }
            }

            // restore
            let _ = (*mid).next.insert(reverse(second_half).unwrap());
            true
        }
    }

    fn half_by_half(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        let mut rev = None;
        let mut _nxt = None;

        // half to half
        while head.is_some() {
            if rev == head || rev == head.as_ref().unwrap().next {
                return true;
            }

            _nxt = head.as_mut().unwrap().next.take();
            head.as_mut().unwrap().next = rev;
            rev = head;
            head = _nxt;
        }

        false
    }

    half_by_half(head)
}

#[allow(dead_code)]
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    type Node = Option<Box<ListNode>>;

    pub fn reverse_list(head: Node) -> Node {
        let (mut head, mut prev) = (head, None);
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    pub fn reverse_list_recursive(head: Node) -> Node {
        fn recurse(head: Node, prev: Node) -> Node {
            match head {
                Some(mut node) => {
                    let next = node.next.take();
                    node.next = prev;
                    recurse(next, Some(node))
                }
                None => prev,
            }
        }
        recurse(head, None)
    }

    reverse_list_recursive(head)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test44() {
        println!("{:?}", is_ugly(6));
        println!("{:?}", is_ugly(14));
    }

    #[test]
    fn test45() {
        println!("{:?}", str_str("sadbutsad".to_owned(), "sad".to_owned()));
        println!("{:?}", str_str("leetcode".to_owned(), "leeto".to_owned()));
        println!("{:?}", str_str("sadbutsad".to_owned(), "but".to_owned()));
        println!(
            "{:?}",
            str_str("sadbutsosad".to_owned(), "sosad".to_owned())
        );
    }

    #[test]
    fn test46() {
        println!(
            "{:?}",
            is_palindrome(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 1, next: None })),
                    })),
                })),
            })))
        );
    }

    #[test]
    fn test47() {
        println!(
            "{:?}",
            reverse_list(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
            })))
        );
    }
}
