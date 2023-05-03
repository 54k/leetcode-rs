// https://leetcode.com/problems/find-the-difference-of-two-arrays/description/
pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    let s1 = nums1.into_iter().collect::<HashSet<i32>>();
    let s2 = nums2.into_iter().collect::<HashSet<i32>>();
    let mut ans = vec![vec![]; 2];
    for e in &s1 {
        if !s2.contains(e) {
            ans[0].push(*e);
        }
    }
    for e in &s2 {
        if !s1.contains(e) {
            ans[1].push(*e);
        }
    }
    ans
}

// https://leetcode.com/problems/flatten-nested-list-iterator/description/
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    stack: Vec<NestedInteger>,
}

impl NestedIterator {
    fn new(mut nestedList: Vec<NestedInteger>) -> Self {
        nestedList.reverse();
        Self { stack: nestedList }
    }

    fn next(&mut self) -> i32 {
        if !self.has_next() {
            panic!();
        }
        match self.stack.pop().unwrap() {
            NestedInteger::Int(v) => return v,
            _ => panic!(),
        }
    }

    fn has_next(&mut self) -> bool {
        self.make_stack_top_an_integer();
        self.stack.len() > 0
    }

    fn make_stack_top_an_integer(&mut self) {
        while !self.stack.is_empty()
            && match self.stack.last().unwrap() {
                NestedInteger::Int(_) => false,
                _ => true,
            }
        {
            match self.stack.pop().unwrap() {
                NestedInteger::List(list) => {
                    for l in list.into_iter().rev() {
                        self.stack.push(l);
                    }
                }
                _ => panic!(),
            }
        }
    }
}

// https://leetcode.com/problems/top-k-frequent-elements/description/
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::{BinaryHeap, HashMap};
    let (mut map, mut heap) = (HashMap::new(), BinaryHeap::new());
    for n in nums {
        *map.entry(n).or_insert(0) += 1;
    }
    for (key, v) in map {
        heap.push((-v, key));
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    heap.into_vec().into_iter().map(|(_, k)| k).collect()
}

// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/description/
pub fn k_weakest_rows(mat: Vec<Vec<i32>>, mut k: i32) -> Vec<i32> {
    fn get_strength(mat: &Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = mat.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if mat[mid] == 1 {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }

    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();
    for (i, row) in mat.into_iter().enumerate() {
        let strength = get_strength(&row);
        let item = (strength, i as i32);

        heap.push(item);
        if heap.len() > k as usize {
            heap.pop();
        }
    }

    let mut res = vec![0; k as usize];
    while let Some(v) = heap.pop() {
        res[k as usize - 1] = v.1;
        k -= 1;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test480() {
        println!("{:?}", find_difference(vec![1, 2, 3], vec![2, 4, 6])); // 1,3 / 4,6
    }

    #[test]
    fn test481() {
        let mut it = NestedIterator::new(vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(2),
                NestedInteger::List(vec![NestedInteger::Int(3)]),
            ]),
        ]);
        println!("{}", it.next()); // 1
        println!("{}", it.next()); // 2
        println!("{}", it.next()); // 3
    }

    #[test]
    fn test482() {
        println!(
            "{:?}",
            k_weakest_rows(
                vec![
                    vec![1, 1, 1, 0, 0, 0, 0],
                    vec![1, 1, 1, 1, 1, 1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0],
                    vec![1, 1, 1, 0, 0, 0, 0],
                    vec![1, 1, 1, 1, 1, 1, 1]
                ],
                4
            )
        ); // 2,0,3,1
    }
}
