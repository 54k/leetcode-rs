// https://leetcode.com/problems/data-stream-as-disjoint-intervals/description/
mod sum_ranges_sorted_set {
    use std::collections::BTreeMap;
    struct SummaryRanges {
        intervals: BTreeMap<i32, i32>,
    }

    impl SummaryRanges {
        fn new() -> Self {
            Self {
                intervals: BTreeMap::new(),
            }
        }

        fn add_num(&mut self, value: i32) {
            let mut left = value;
            let mut right = value;

            let floor = self.intervals.range(..=value).last();
            if let Some((l, r)) = floor {
                if *r >= value {
                    return;
                }
                if *r + 1 == value {
                    left = *l;
                }
            }

            let ceil = self.intervals.range(value + 1..).take(1).last();
            if let Some((l, r)) = ceil {
                if *l == value + 1 {
                    right = *r;
                    self.intervals.remove(&(value + 1));
                }
            }
            self.intervals.insert(left, right);
        }

        fn get_intervals(&self) -> Vec<Vec<i32>> {
            let mut ans = Vec::new();
            for (l, r) in &self.intervals {
                ans.push(vec![*l, *r]);
            }
            ans
        }
    }
}

// https://leetcode.com/problems/next-greater-element-i/description/
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut stack = vec![];
    let mut map = HashMap::new();
    for i in 0..nums2.len() {
        while !stack.is_empty() && nums2[i] > *stack.last().unwrap() {
            map.insert(stack.pop().unwrap(), nums2[i]);
        }
        stack.push(nums2[i]);
    }

    while !stack.is_empty() {
        map.insert(stack.pop().unwrap(), -1);
    }

    let mut res = vec![0; nums1.len()];
    for i in 0..nums1.len() {
        res[i] = map[&nums1[i]];
    }
    res
}

// https://leetcode.com/problems/next-greater-element-iv/description/
pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
    pub fn second_greater_element_heap(nums: Vec<i32>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let n = nums.len();
        let mut middle = vec![vec![]; n];
        let mut stack = vec![];

        for i in 0..n {
            while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[i] {
                middle[i].push(stack.pop().unwrap());
            }
            stack.push(i);
        }

        let mut ans = vec![-1; n];
        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

        for i in 0..n {
            while !heap.is_empty() && nums[heap.peek().unwrap().0 .1] < nums[i] {
                ans[heap.pop().unwrap().0 .1] = nums[i];
            }
            for &j in &middle[i] {
                heap.push(Reverse((nums[j], j)));
            }
        }

        ans
    }

    pub fn second_greater_element_2_stacks(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;
        let n = nums.len();
        let mut ans = vec![-1; n];

        let mut s1 = vec![];
        let mut s2 = vec![];

        for i in 0..n {
            while !s2.is_empty() && nums[i] > nums[*s2.last().unwrap()] {
                ans[s2.pop().unwrap()] = nums[i];
            }
            let mut temp = VecDeque::new();
            while !s1.is_empty() && nums[i] > nums[*s1.last().unwrap()] {
                temp.push_front(s1.pop().unwrap());
            }
            s2.extend(temp);
            s1.push(i);
        }
        ans
    }

    second_greater_element_2_stacks(nums)
}

#[test]
fn test_second_greater_element() {
    let res = second_greater_element(vec![2, 4, 0, 9, 6]);
    println!("{:?}", res);
}
