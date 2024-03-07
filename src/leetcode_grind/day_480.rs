// https://leetcode.com/problems/middle-of-the-linked-list/description/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let r = head.as_ref();
    let mut f = r;
    let mut s = r;

    while let Some(f1) = f {
        if let Some(fast) = f1.next.as_ref() {
            s = s.unwrap().next.as_ref();
            f = fast.next.as_ref();
        } else {
            break;
        }
    }

    Some(s.clone().unwrap().clone())
}

// https://leetcode.com/problems/moving-stones-until-consecutive-ii/description/
pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
    let mut stones = stones;
    stones.sort();
    let mut i = 0;
    let n = stones.len();
    let mut low = n as i32;
    let high =
        (stones[n - 1] - n as i32 + 2 - stones[1]).max(stones[n - 2] - stones[0] - n as i32 + 2);
    for j in 0..n {
        while stones[j] - stones[i] >= n as i32 {
            i += 1;
        }

        if j - i + 1 == n - 1 && stones[j] - stones[i] == n as i32 - 2 {
            low = low.min(2);
        } else {
            low = low.min(n as i32 - (j as i32 - i as i32 + 1));
        }
    }
    vec![low, high]
}
