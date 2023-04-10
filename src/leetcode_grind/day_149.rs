// https://leetcode.com/problems/design-linked-list/
use std::ptr;

struct UnsafeListNode {
    val: i32,
    prev: *mut UnsafeListNode,
    next: *mut UnsafeListNode,
}
struct MyLinkedList {
    head: *mut UnsafeListNode,
    tail: *mut UnsafeListNode,
    len: i32,
}
impl MyLinkedList {
    fn new() -> Self {
        unsafe {
            let mut head = Box::into_raw(Box::new(UnsafeListNode {
                val: i32::MIN,
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            }));
            let mut tail = Box::into_raw(Box::new(UnsafeListNode {
                val: i32::MIN,
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            }));
            (*head).prev = head;
            (*head).next = tail;
            (*tail).next = tail;
            (*tail).prev = head;
            Self { head, tail, len: 0 }
        }
    }
    fn get(&self, mut index: i32) -> i32 {
        unsafe {
            if index < 0 || index >= self.len {
                return -1;
            }
            let mut cur = (*self.head).next;
            while index > 0 {
                cur = (*cur).next;
                index -= 1;
            }
            (*cur).val
        }
    }
    fn add_at_head(&mut self, val: i32) {
        unsafe {
            let mut new_head = Box::into_raw(Box::new(UnsafeListNode {
                val,
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            }));
            let old_head = (*self.head).next;
            (*new_head).prev = self.head;
            (*new_head).next = old_head;
            (*old_head).prev = new_head;
            (*self.head).next = new_head;
            self.len += 1;
        }
    }
    fn add_at_tail(&mut self, val: i32) {
        unsafe {
            let mut new_tail = Box::into_raw(Box::new(UnsafeListNode {
                val,
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            }));
            let prev = (*self.tail).prev;
            (*new_tail).prev = prev;
            (*new_tail).next = self.tail;
            (*prev).next = new_tail;
            (*self.tail).prev = new_tail;
            self.len += 1;
        }
    }
    fn add_at_index(&mut self, mut index: i32, val: i32) {
        unsafe {
            if index > self.len {
                return;
            }
            let mut cur = (*self.head).next;
            while index > 0 {
                cur = (*cur).next;
                index -= 1;
            }
            let mut new_node = Box::into_raw(Box::new(UnsafeListNode {
                val,
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            }));
            let prev = (*cur).prev;
            (*prev).next = new_node;
            (*cur).prev = new_node;
            (*new_node).prev = prev;
            (*new_node).next = cur;
            self.len += 1;
        }
    }
    fn delete_at_index(&mut self, mut index: i32) {
        unsafe {
            if index >= self.len {
                return;
            }
            let mut cur = (*self.head).next;
            while index > 0 {
                cur = (*cur).next;
                index -= 1;
            }
            let prev = (*cur).prev;
            let next = (*cur).next;
            (*prev).next = next;
            (*next).prev = prev;
            let _ = Box::from_raw(cur);
            self.len -= 1;
        }
    }
}

// https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/
pub fn robot_with_string(s: String) -> String {
    fn has_smaller(c: char, freq: &[i32]) -> bool {
        let bound = c as usize - 'a' as usize;
        for i in 0..bound {
            if freq[i] > 0 {
                return true;
            }
        }
        false
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut freq = vec![0; 26];
    for i in 0..s.len() {
        freq[s[i] as usize - 'a' as usize] += 1;
    }

    let mut res = String::new();
    let mut stack = vec![];

    for i in 0..s.len() {
        freq[s[i] as usize - 'a' as usize] -= 1;
        stack.push(s[i]);
        while !stack.is_empty() && !has_smaller(*stack.last().unwrap(), &freq) {
            res.push(stack.pop().unwrap());
        }
    }
    while let Some(c) = stack.pop() {
        res.push(c);
    }
    res
}

// https://leetcode.com/problems/validate-stack-sequences/
pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack = vec![];
    let mut j = 0;
    for i in 0..pushed.len() {
        stack.push(pushed[i]);
        while j < popped.len() && *stack.last().unwrap() == popped[j] {
            stack.pop();
            j += 1;
        }
    }
    j == popped.len()
}

// https://leetcode.com/problems/find-the-most-competitive-subsequence/editorial/
pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut additional_count = nums.len() as i32 - k;

    for i in 0..nums.len() {
        while !queue.is_empty() && *queue.back().unwrap() > nums[i] && additional_count > 0 {
            queue.pop_back();
            additional_count -= 1;
        }
        queue.push_back(nums[i]);
    }

    let mut result = vec![0; k as usize];
    for i in 0..k as usize {
        result[i] = queue.pop_front().unwrap();
    }
    result
}

// https://leetcode.com/problems/number-of-visible-people-in-a-queue/
pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; heights.len()];
    let mut stack = vec![];
    for i in (0..heights.len()).rev() {
        let mut count = 0;
        while !stack.is_empty() && heights[*stack.last().unwrap()] <= heights[i] {
            stack.pop();
            count += 1;
        }
        if i < heights.len() - 1 {
            result[i] = count + !stack.is_empty() as i32;
        }
        stack.push(i);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test425() {
        let mut ll = MyLinkedList::new();
        ll.add_at_head(1);
        ll.add_at_tail(3);
        ll.add_at_index(1, 2);
        println!("{}", ll.get(0));
        println!("{}", ll.get(1));
        println!("{}", ll.get(2));
        ll.delete_at_index(1);
        println!("{}", ll.get(1));

        let mut ll = MyLinkedList::new();
        ll.add_at_head(7);
        ll.add_at_head(2);
        ll.add_at_head(1);
        ll.add_at_index(3, 0);
        ll.delete_at_index(2);
        ll.add_at_head(6);
        ll.add_at_tail(4);
        println!("{}", ll.get(4));
        ll.add_at_index(5, 0);
        ll.add_at_head(6);
    }

    #[test]
    fn test426() {
        println!("{}", robot_with_string("bydizfve".to_string())); // bdevfziy
        println!("{}", robot_with_string("bac".to_string())); // abc
        println!("{}", robot_with_string("bdda".to_string())); // addb
    }

    #[test]
    fn test427() {
        println!(
            "{}",
            validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1])
        ); // true
        println!(
            "{}",
            validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2])
        ); // false
    }

    #[test]
    fn test428() {
        println!("{:?}", most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4)); // [2,3,3,4]
    }

    #[test]
    fn test429() {
        println!("{:?}", can_see_persons_count(vec![10, 6, 8, 5, 11, 9])); // [3,1,2,1,1,0]
    }
}
