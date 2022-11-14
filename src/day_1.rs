// Definition for singly-linked list.
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

#[allow(dead_code)]
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    use std::cmp::{Ordering, Reverse};
    use std::collections::binary_heap::BinaryHeap;

    impl PartialOrd<Self> for Box<ListNode> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.val.cmp(&other.val) {
                ord @ Ordering::Greater => Some(ord),
                ord @ Ordering::Less => Some(ord),
                ord @ Ordering::Equal => Some(ord),
            }
        }
    }

    impl Ord for Box<ListNode> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    let mut head = None;
    let mut tail = &mut head;

    let mut heap = BinaryHeap::new();
    for x in lists.iter().flatten() {
        heap.push(Reverse(x));
    }

    while !heap.is_empty() {
        let pop = heap.pop().unwrap().0;
        if let Some(n) = pop.next.as_ref() {
            heap.push(Reverse(n));
        }
        let i = tail.insert(Box::new(ListNode::new(pop.val)));
        tail = &mut i.next;
    }

    head
}

#[allow(dead_code)]
pub fn reverse(mut x: i32) -> i32 {
    let mut rev = 0;
    while x != 0 {
        let pop = x % 10;
        x /= 10;
        if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && pop > 7) {
            return 0;
        }
        if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && pop < -8) {
            return 0;
        }
        rev = rev * 10 + pop;
    }
    rev
}

#[allow(dead_code)]
pub fn my_atoi(s: String) -> i32 {
    let s = s.trim();

    let mut res = 0;
    let mut sign = 1;
    let mut sign_seen = false;

    for ch in s.chars() {
        match ch {
            dig if ch.is_ascii_digit() => {
                sign_seen = true;
                let dig = dig as i32 - 48;
                if res > i32::MAX / 10 || (res == i32::MAX / 10 && dig > 7) {
                    return i32::MAX;
                }
                if res < i32::MIN / 10 || (res == i32::MIN / 10 && dig > 8) {
                    return i32::MIN;
                }
                res = res * 10 + (dig * sign);
            }
            _ if ch == '-' && !sign_seen => {
                sign *= -1;
                res *= sign;
                sign_seen = true
            }
            _ if ch == '+' && !sign_seen => sign_seen = true,
            _ => break,
        }
    }
    res * 2
}

#[allow(dead_code)]
pub fn longest_valid_parentheses(s: String) -> i32 {
    use std::cmp::max;

    let mut res = 0;
    let mut stack = vec![-1i32];
    for (i, ch) in s.chars().enumerate() {
        match ch {
            '(' => {
                stack.push(i as i32);
            }
            ')' => {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    res = max(res, i as i32 - stack[stack.len() - 1])
                }
            }
            _ => continue,
        }
    }
    res
}

#[allow(dead_code)]
pub fn longest_palindrome(s: String) -> String {
    use std::cmp::max;
    use std::collections::HashSet;
    if s.chars().collect::<HashSet<char>>().len() == 1 {
        return s;
    }
    fn expand_around_center(s: &str, mut left: i32, mut right: i32) -> usize {
        while left >= 0
            && right < s.len() as i32
            && s.chars().nth(left as usize).unwrap() == s.chars().nth(right as usize).unwrap()
        {
            left -= 1;
            right += 1;
        }
        (right - left - 1) as usize
    }

    let mut start = 0;
    let mut end = 0;

    for i in 0..s.len() {
        let len1 = expand_around_center(&s, i as i32, i as i32);
        let len2 = expand_around_center(&s, i as i32, (i + 1) as i32);
        let len = max(len1, len2);

        if len > end - start {
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }

    let x = &s.chars().collect::<Vec<char>>()[start..=end];
    x.iter().collect::<String>()
}

#[allow(dead_code)]
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    head.as_ref()?;

    fn get_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut vv = vec![];
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            cur = node.next.as_ref();
            vv.push(node.val);
        }

        println!("{:?}", vv);
        vv
    }

    let mut vv = get_vec(&head);

    let len = vv.len() as i32;
    let di = len - n;
    vv.remove(di as usize);

    if vv.is_empty() {
        return None;
    }

    let mut result = ListNode::new(vv.remove(0));
    let mut result_iter = &mut result.next;

    for v in vv.into_iter() {
        let n = Box::new(ListNode::new(v));
        let n = result_iter.insert(n);
        result_iter = &mut n.next;
    }

    println!("{:?}", head);
    Some(Box::new(result))
}

#[allow(dead_code)]
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.as_ref()?;
    let mut vv = vec![];
    let mut cc = head.as_ref();
    while let Some(n) = cc {
        vv.push(n.val);
        cc = n.next.as_ref();
    }

    let mut head = None;
    let mut result_iter = &mut head;
    for chunk in vv.chunks(2) {
        if chunk.len() == 1 {
            let _ = result_iter.insert(Box::new(ListNode::new(chunk[0])));
            continue;
        }

        let left = chunk[0];
        let right = chunk[1];
        let right = result_iter.insert(Box::new(ListNode::new(right)));
        let left = Box::new(ListNode::new(left));
        let left = right.next.insert(left);
        result_iter = &mut left.next;
    }

    head
}

#[allow(dead_code)]
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let k = k as usize;

    let mut vv = vec![];
    let mut head = head.as_ref();
    while let Some(n) = head {
        vv.push(n.val);
        head = n.next.as_ref();
    }

    for chunk in vv.chunks_mut(k) {
        if chunk.len() == k {
            chunk.reverse()
        }
    }

    let mut head = None;
    let mut tail = &mut head;
    for x in vv {
        let i = tail.insert(Box::new(ListNode::new(x)));
        tail = &mut i.next;
    }
    head
}

#[allow(dead_code)]
pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let k = k as usize;
    let mut vv = vec![];
    let mut head = head.as_ref();
    while let Some(n) = head {
        vv.push(n.val);
        head = n.next.as_ref();
    }

    let len = vv.len();
    vv.swap(k - 1, len - k);

    let mut head = None;
    let mut tail = &mut head;
    for x in vv {
        let i = tail.insert(Box::new(ListNode::new(x)));
        tail = &mut i.next;
    }
    head
}

#[allow(dead_code)]
pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();

    for num in nums.iter_mut().take(n) {
        if *num <= 0 {
            *num = (n + 1) as i32;
        }
    }

    for i in 0..n {
        let num = nums[i].unsigned_abs() as usize;
        if num <= n && nums[num - 1] > 0 {
            nums[num - 1] *= -1;
        }
    }

    for (i, num) in nums.iter().enumerate().take(n) {
        if *num > 0 {
            return (i + 1) as i32;
        }
    }

    (n + 1) as i32
}

#[allow(dead_code)]
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut result = (n * (n + 1) / 2) as i32;
    for x in nums.iter() {
        result -= *x;
    }
    result
}

#[allow(dead_code)]
pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    let mut result = String::new();
    let n = s.len() as i32;
    let cycle_len = 2 * num_rows - 2;
    let chars = s.chars().collect::<Vec<_>>();

    for i in 0..num_rows {
        let mut j = 0;
        while i + j < n {
            result.push(chars[(i + j) as usize]);
            if i != 0 && i != num_rows - 1 && j + cycle_len - i < n {
                result.push(chars[(j + cycle_len - i) as usize]);
            }
            j += cycle_len;
        }
    }
    result
}

#[allow(dead_code)]
pub fn is_palindrome(mut x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut reverted_number = 0;
    while x > reverted_number {
        reverted_number = reverted_number * 10 + x % 10;
        x /= 10;
    }
    x == reverted_number || x == reverted_number / 10
}

#[cfg(test)]
mod test {
    use crate::day_1::*;

    #[test]
    fn test1() {
        //[[1,4,5],[1,3,4],[2,6]]
        let vec = vec![
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 6, next: None })),
            })),
        ];
        let merged = merge_k_lists(vec);
        println!("Result: {:?}", merged);
    }

    #[test]
    fn test2() {
        println!("{}", reverse(12345));
    }

    #[test]
    fn test3() {
        println!(
            "{}",
            my_atoi("   -001234588899999000000000 with words  ".to_owned())
        );
        println!("{}", my_atoi("-91283472332".to_owned()));
        println!("{}", my_atoi("+1".to_owned()));
        println!("{}", my_atoi("+-1".to_owned()));
        println!("{}", my_atoi("2147483648".to_owned()));
    }

    #[test]
    fn test4() {
        println!("{}", longest_valid_parentheses("()(()".to_owned()));
    }

    #[test]
    fn test5() {
        println!("{}", longest_palindrome("z".repeat(40)));
    }

    #[test]
    fn test6() {
        println!(
            "{:?}",
            remove_nth_from_end(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
                1,
            )
        );
    }

    #[test]
    fn test7() {
        println!(
            "{:?}",
            swap_pairs(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),)
        );
    }

    #[test]
    fn test8() {
        println!(
            "{:?}",
            reverse_k_group(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
                3,
            )
        );
    }

    #[test]
    fn test9() {
        println!(
            "{:?}",
            swap_nodes(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
                1,
            )
        );
    }

    #[test]
    fn test10() {
        println!("{:?}", first_missing_positive(vec![1, 2, 0]));
    }

    #[test]
    fn test11() {
        println!("{:?}", missing_number(vec![1, 3, 0]));
    }

    #[test]
    fn test12() {
        println!("{:?}", convert("PAYPALISHIRING".to_owned(), 3));
    }

    #[test]
    fn test13() {
        println!("{:?}", is_palindrome(1221));
    }
}
