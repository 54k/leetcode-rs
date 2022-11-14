#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    let mut res_head = Some(Box::new(ListNode {
        val: -1,
        next: head,
    }));

    let mut con = &mut res_head;
    for _ in 1..left {
        con = &mut con.as_mut()?.next;
    }

    let mut prev = con.as_mut()?.next.take();
    let mut cur = prev.as_mut()?.next.take();
    for _ in left..right {
        let next = cur.as_mut()?.next.take();
        cur.as_mut()?.next = prev;
        prev = cur;
        cur = next;
    }

    let mut tail = &mut prev;
    for _ in left..right {
        tail = &mut tail.as_mut()?.next;
    }

    tail.as_mut()?.next = cur;
    con.as_mut()?.next = prev;

    res_head?.next
}

#[allow(dead_code)]
pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    fn first(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
        if n == 0 {
            return;
        }

        let mut i = m - 1;
        let mut j = n - 1;

        for k in (0..(m + n)).rev() {
            let k = k as usize;

            if i < 0 {
                nums1[k] = nums2[j as usize];
                j -= 1;
            } else if j < 0 {
                nums1[k] = nums1[i as usize];
                i -= 1;
            } else if nums1[i as usize] < nums2[j as usize] {
                nums1[k] = nums2[j as usize];
                j -= 1;
            } else {
                nums1[k] = nums1[i as usize];
                i -= 1;
            }
        }
    }

    fn second(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
        let mut k = m + n - 1;
        let mut i = m - 1;
        let mut j = n - 1;

        while i >= 0 && j >= 0 {
            if nums1[i as usize] <= nums2[j as usize] {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            } else {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            }
            k -= 1;
        }

        while j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
    second(nums1, m, nums2, n);
}

#[allow(dead_code)]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0i32;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
        let mid = (left + (right - left) / 2) as usize;
        let i = nums[mid];

        if i < target {
            left = mid as i32 + 1;
        } else if i > target {
            right = mid as i32 - 1;
        } else {
            return mid as i32;
        }
    }

    left as i32
}

type Node = Option<Box<ListNode>>;
#[allow(dead_code)]
pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    fn rot(mut head: Node) -> Node {
        let mut tail = &mut head;
        while tail.is_some() && tail.as_ref()?.next.is_some() {
            tail = &mut tail.as_mut()?.next;
        }
        let mut tail = tail.take();
        tail.as_mut()?.next = head;
        tail
    }

    fn size(head: &Node) -> i32 {
        let mut size = 0;
        let mut cur = head;
        while let Some(node) = cur {
            cur = &node.as_ref().next;
            size += 1;
        }
        size
    }

    if head.is_none() {
        return head;
    }

    let size = size(&head);
    let mut res = head;
    for _ in 0..k % size {
        res = rot(res)
    }
    res
}

#[cfg(test)]
mod test {
    use crate::day_7::*;

    #[test]
    fn test48() {
        println!(
            "{:?}",
            reverse_between(
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
                                        next: Some(Box::new(ListNode { val: 7, next: None }))
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
                3,
                5
            )
        );
    }

    #[test]
    fn test49() {
        let mut v = vec![4, 5, 6, 0, 0, 0];
        merge(&mut v, 3, &mut [1, 2, 3], 3);
        println!("{:?}", v);
    }

    #[test]
    fn test50() {
        println!("{}", search_insert(vec![1, 3, 5, 6], 5));
        println!("{}", search_insert(vec![1, 3, 5, 6], 2));
        println!("{}", search_insert(vec![1, 3, 5, 6], 7));
        println!("{}", search_insert(vec![1], 1));
        println!("{}", search_insert(vec![1, 3], 2));
        println!("{}", search_insert(vec![1, 3, 5, 6], 2));
        println!("{}", search_insert(vec![1, 3], 3));
        println!("{}", search_insert(vec![1, 3, 5, 6], 0));
    }

    #[test]
    fn test51() {
        println!(
            "{:?}",
            rotate_right(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode { val: 5, next: None })),
                            })),
                        })),
                    })),
                })),
                2
            )
        );
    }
}
