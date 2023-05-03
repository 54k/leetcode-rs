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

// https://leetcode.com/problems/last-stone-weight/description/
pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    for s in stones {
        heap.push(s);
    }

    while heap.len() > 1 {
        let f = heap.pop().unwrap();
        let s = heap.pop().unwrap();
        if f != s {
            heap.push(f - s);
        }
    }

    heap.pop().unwrap_or(0)
}

// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/description/
pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    fn using_heap(matrix: Vec<Vec<i32>>, mut k: i32) -> i32 {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        for i in 0..matrix.len() {
            heap.push((-matrix[i][0], i, 0));
        }
        let mut element = *heap.peek().unwrap();
        while k > 0 {
            element = heap.pop().unwrap();
            let (r, c) = (element.1, element.2);
            if c < matrix.len() - 1 {
                heap.push((-matrix[r][c + 1], r, c + 1));
            }

            k -= 1;
        }
        -element.0
    }
    fn using_binsearch(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        fn count_less_equal(
            matrix: &Vec<Vec<i32>>,
            mid: i32,
            small_large_pair: &mut (i32, i32),
        ) -> i32 {
            let mut count = 0;
            let n = matrix.len() as i32;
            let mut row = n - 1;
            let mut col = 0;

            while row >= 0 && col < n {
                if matrix[row as usize][col as usize] > mid {
                    small_large_pair.1 = small_large_pair.1.min(matrix[row as usize][col as usize]);
                    row -= 1;
                } else {
                    small_large_pair.0 = small_large_pair.0.max(matrix[row as usize][col as usize]);
                    count += row + 1;
                    col += 1;
                }
            }
            count
        }
        let n = matrix.len();
        let mut left = matrix[0][0];
        let mut right = matrix[n - 1][n - 1];
        while left < right {
            let mid = left + (right - left) / 2;
            let mut small_large_pair = (matrix[0][0], matrix[n - 1][n - 1]);
            let count = count_less_equal(&matrix, mid, &mut small_large_pair);
            if count == k {
                return small_large_pair.0;
            }
            if count < k {
                left = small_large_pair.1;
            } else {
                right = small_large_pair.0;
            }
        }
        left
    }
    using_binsearch(matrix, k)
}

// https://leetcode.com/problems/search-a-2d-matrix/editorial/
pub fn search_matrix_i(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len() as i32;
    if m == 0 {
        return false;
    }
    let n = matrix[0].len() as i32;

    let (mut left, mut right) = (0, m * n - 1);
    while left <= right {
        let pivot_idx = (left + right) / 2;
        let pivot_elem = matrix[(pivot_idx / n) as usize][(pivot_idx % n) as usize];
        if target == pivot_elem {
            return true;
        } else {
            if target < pivot_elem {
                right = pivot_idx - 1;
            } else {
                left = pivot_idx + 1;
            }
        }
    }
    false
}

// https://leetcode.com/problems/search-a-2d-matrix-ii/description/
pub fn search_matrix_ii(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    fn using_space_reduction(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row = matrix.len() as i32 - 1;
        let mut col = 0i32;

        while row >= 0 && col < matrix[0].len() as i32 {
            if matrix[row as usize][col as usize] > target {
                row -= 1;
            } else if matrix[row as usize][col as usize] < target {
                col += 1;
            } else {
                return true;
            }
        }
        false
    }
    using_space_reduction(matrix, target)
}

// https://leetcode.com/problems/furthest-building-you-can-reach/description/
pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
    fn min_heap_approach(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
        use std::collections::BinaryHeap;
        let mut ladder_allocations = BinaryHeap::new();
        for i in 0..heights.len() - 1 {
            let climb = heights[i + 1] - heights[i];
            if climb <= 0 {
                continue;
            }

            ladder_allocations.push(-climb);
            // If we haven't gone over the number of ladders, nothing else to do.
            if ladder_allocations.len() as i32 <= ladders {
                continue;
            }
            // otherwise, we will need to take a climb out of ladder_allocations
            bricks += ladder_allocations.pop().unwrap();
            if bricks < 0 {
                return i as i32;
            }
        }
        return heights.len() as i32 - 1;
    }

    fn max_heap_approach(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        use std::collections::BinaryHeap;
        let mut bricks_allocations = BinaryHeap::new();
        for i in 0..heights.len() - 1 {
            let climb = heights[i + 1] - heights[i];
            if climb <= 0 {
                continue;
            }

            bricks_allocations.push(climb);
            bricks -= climb;

            if bricks >= 0 {
                continue;
            }

            if ladders == 0 {
                return i as i32;
            }

            bricks += bricks_allocations.pop().unwrap();
            ladders -= 1;
        }
        heights.len() as i32 - 1
    }

    fn bin_search_approach(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        fn is_reachable(heights: &Vec<i32>, idx: usize, mut bricks: i32, mut ladders: i32) -> bool {
            let mut climbs = vec![];
            for i in 0..idx {
                let climb = heights[i + 1] - heights[i];
                if climb > 0 {
                    climbs.push(climb);
                }
            }
            climbs.sort();
            for climb in climbs {
                if bricks > climb {
                    bricks -= climb;
                } else if ladders > 0 {
                    ladders -= 1;
                } else {
                    return false;
                }
            }
            true
        }

        let mut lo = 0 as i32;
        let mut hi = heights.len() as i32 - 1;
        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            if is_reachable(&heights, mid as usize, bricks, ladders) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        lo
    }

    bin_search_approach(heights, bricks, ladders)
}

// https://leetcode.com/problems/find-median-from-data-stream/description/
use std::collections::BinaryHeap;

struct MedianFinder {
    min: BinaryHeap<i32>,
    max: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            min: BinaryHeap::new(),
            max: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.min.push(num);
        self.max.push(-self.min.pop().unwrap());
        if self.max.len() > self.min.len() {
            self.min.push(-self.max.pop().unwrap());
        }
    }

    fn find_median(&self) -> f64 {
        if self.min.len() > self.max.len() {
            (*self.min.peek().unwrap()) as f64
        } else {
            ((*self.min.peek().unwrap()) as f64 - (*self.max.peek().unwrap()) as f64) / 2.0
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

    #[test]
    fn test483() {
        println!(
            "{}",
            kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8)
        ); // 13

        println!("{}", kth_smallest(vec![vec![-5]], 1)); // -5
    }
}
