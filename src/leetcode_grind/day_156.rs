use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut ans = vec![];
    let max = candies.iter().copied().max().unwrap();
    for c in candies {
        ans.push(c + extra_candies >= max);
    }
    ans
}

// https://leetcode.com/problems/delete-leaves-with-a-given-value/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn remove_leaf_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn post_order(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root.clone() {
            let left = r.borrow().left.clone();
            r.borrow_mut().left = post_order(left, target);
            let right = r.borrow().right.clone();
            r.borrow_mut().right = post_order(right, target);

            if r.borrow().left.is_none() && r.borrow().right.is_none() && r.borrow().val == target {
                None
            } else {
                root
            }
        } else {
            None
        }
    }
    post_order(root, target)
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::new();
        for n in nums {
            heap.push(Reverse(n));
        }
        while heap.len() > k as usize {
            heap.pop();
        }
        Self {
            heap,
            k: k as usize,
        }
    }
    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

// https://leetcode.com/problems/minimum-cost-to-connect-sticks/description/
pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    for s in sticks {
        heap.push(Reverse(s));
    }
    let mut ans = 0;
    while heap.len() > 1 {
        let i = heap.pop().unwrap().0;
        let j = heap.pop().unwrap().0;
        let cost = i + j;
        ans += cost;
        heap.push(Reverse(cost));
    }
    ans
}

// https://leetcode.com/problems/seat-reservation-manager/
struct SeatManager {
    heap: BinaryHeap<Reverse<i32>>,
}
impl SeatManager {
    fn new(n: i32) -> Self {
        let mut heap = BinaryHeap::new();
        for i in 0..n {
            heap.push(Reverse(i));
        }
        Self { heap }
    }
    fn reserve(&mut self) -> i32 {
        self.heap.pop().unwrap().0
    }
    fn unreserve(&mut self, seat_number: i32) {
        self.heap.push(Reverse(seat_number));
    }
}

// https://leetcode.com/problems/construct-string-with-repeat-limit/
pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    let mut freq = vec![0; 26];
    for ch in s.chars() {
        freq[ch as usize - 'a' as usize] += 1;
    }
    let mut ans = String::new();
    loop {
        let mut insert_one = false;
        let mut i = 25i32;
        while i >= 0 {
            if ans.len() > 0
                && ans.chars().last().unwrap() as i32 - 'a' as i32 == i
                && freq[i as usize] > 0
            {
                insert_one = true;
                i -= 1;
                continue;
            }
            if freq[i as usize] > 0 {
                break;
            }
            i -= 1;
        }
        if i == -1 {
            break;
        }
        let mut count = if insert_one {
            1
        } else {
            freq[i as usize].min(repeat_limit)
        };
        freq[i as usize] -= count;
        while count > 0 {
            ans.push((b'a' + (i as u8)) as char);
            count -= 1;
        }
    }
    ans
}

// https://leetcode.com/problems/meeting-rooms-iii/description/
// Explanation
//
// ready contains the ready room index for meetings.
// rooms contains the rooms in use with [end_time, room_index] as element.
//
// For [start, end] in the sorted meetings,
// we firstly release the rooms that is ready before start time.
// If there is room in ready state,
// we choose the room with smallest index.
// Otherwise, we choose the room with smallest end_time in rooms.
pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    meetings.sort();
    let mut ready = BinaryHeap::from((0..n).map(|x| Reverse(x)).collect::<Vec<_>>());
    let mut rooms: BinaryHeap<Reverse<(i64, i32)>> = BinaryHeap::new();
    let mut ans = vec![0; n as usize];

    for m in meetings {
        let start = m[0] as i64;
        let end = m[1] as i64;

        while !rooms.is_empty() && rooms.peek().unwrap().0 .0 <= start {
            ready.push(Reverse((rooms.pop().unwrap().0).1));
        }

        if !ready.is_empty() {
            let r = ready.pop().unwrap().0;
            rooms.push(Reverse((end, r)));
            ans[r as usize] += 1;
        } else {
            let Reverse((t, r)) = rooms.pop().unwrap();
            rooms.push(Reverse((t + end - start, r)));
            ans[r as usize] += 1;
        }
    }

    ans.into_iter()
        .enumerate()
        .map(|(i, v)| (v, Reverse(i)))
        .max()
        .unwrap()
        .1
         .0 as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test442() {
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        println!("{}", kth.add(3));
        println!("{}", kth.add(5));
        println!("{}", kth.add(10));
        println!("{}", kth.add(9));
        println!("{}", kth.add(4));
    }

    #[test]
    fn test443() {
        println!("{}", repeat_limited_string("cczazcc".to_string(), 3));
    }

    #[test]
    fn test444() {
        println!(
            "{}",
            most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]])
        ); // 0
    }
}
