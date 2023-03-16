// https://leetcode.com/problems/intersection-of-two-arrays/description/
// Given two integer arrays nums1 and nums2, return an array of their intersection.
// Each element in the result must be unique and you may return the result in any order.
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::*;
    let mut res = vec![];
    let mut nums1 = nums1.into_iter().collect::<HashSet<_>>();
    for n in nums2 {
        if nums1.contains(&n) {
            res.push(n);
            nums1.remove(&n);
        }
    }
    res
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// https://leetcode.com/problems/merge-two-sorted-lists/description/
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    fn rec(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            list2
        } else if list2.is_none() {
            list1
        } else {
            let mut l2 = list2.as_mut()?;
            let mut l1 = list1.as_mut()?;
            if l1.val < l2.val {
                l1.next = rec(l1.next.take(), list2);
                list1
            } else {
                l2.next = rec(list1, l2.next.take());
                list2
            }
        }
    }
    rec(list1, list2)
}

// Let's just call this what it is, this has nothing to do with picking seats,
// this is the algorithm for picking a urinal in a public bathroom :D
// https://leetcode.com/problems/maximize-distance-to-closest-person/
pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let mut i = 0i32;
    let mut j = seats.len() as i32 - 1;
    while i < seats.len() as i32 && seats[i as usize] == 0 {
        i += 1;
    }
    while j >= 0 && seats[j as usize] == 0 {
        j -= 1;
    }
    let mut ans = 0;
    ans = ans.max(i).max(seats.len() as i32 - 1 - j);
    let mut cur = 0;
    for k in i..=j {
        if seats[k as usize] == 0 {
            cur += 1;
        } else {
            ans = ans.max((cur + 1) / 2);
            cur = 0;
        }
    }
    ans
}

// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/
// Maintain a sliding window where there is at most one zero on it.
pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut zeroes = 0;
    let mut ans = 0;

    for end in 0..nums.len() {
        if nums[end] == 0 {
            zeroes += 1;
        }
        while zeroes > 1 {
            if nums[start] == 0 {
                zeroes -= 1;
            }
            start += 1;
        }
        ans = ans.max(end - start);
    }
    ans as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test191() {
        println!("{:?}", intersection(vec![1, 2, 2, 1], vec![2, 2]));
        println!("{:?}", intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]));
    }

    #[test]
    fn test192() {
        println!(
            "{:?}",
            merge_two_lists(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
            )
        );
    }

    #[test]
    fn test193() {
        println!("{}", max_dist_to_closest(vec![0, 1])); // 1
        println!("{}", max_dist_to_closest(vec![0, 0, 1])); // 2

        println!(
            "{}",
            max_dist_to_closest(vec![0, 1, 0, 0, 0, 1, 1, 0, 1, 1])
        ); // 2
        println!("{}", max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1])); // 2
        println!("{}", max_dist_to_closest(vec![1, 0, 0, 1])); // 1

        println!("{}", max_dist_to_closest(vec![1, 0, 0, 0])); // 3
        println!("{}", max_dist_to_closest(vec![1, 0])); // 1
    }

    #[test]
    fn test194() {
        println!("{}", longest_subarray(vec![1, 1, 0, 1])); // 3
        println!("{}", longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1])); // 5
        println!("{}", longest_subarray(vec![1, 1, 1])); // 2 You must delete one element.
    }
}
