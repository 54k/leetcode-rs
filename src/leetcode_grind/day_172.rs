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
}
