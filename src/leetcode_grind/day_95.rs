// https://leetcode.com/problems/add-to-array-form-of-integer/
pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
    const RADIX: i32 = 10;
    num.reverse();
    let mut ans = vec![];
    let mut carry = 0;
    let mut i = 0;
    loop {
        let ai = if i < num.len() { num[i] } else { 0 };
        let bj = if k > 0 { k % RADIX } else { 0 };
        i += 1;
        k /= RADIX;
        let mut sum = ai + bj + carry;
        carry = sum / RADIX;
        sum %= RADIX;
        ans.push(sum);

        if k == 0 && i >= num.len() {
            if carry > 0 {
                ans.push(carry)
            }
            break;
        }
    }
    ans.into_iter().rev().collect()
}

// https://leetcode.com/problems/find-median-from-data-stream/description/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct MedianFinder {
    left: BinaryHeap<i32>,           // max heap
    right: BinaryHeap<Reverse<i32>>, // min heap
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.left.is_empty() || *self.left.peek().unwrap() <= num {
            self.right.push(Reverse(num))
        } else {
            self.left.push(num)
        }
        if self.left.len() > self.right.len() + 1 {
            self.right.push(Reverse(self.left.pop().unwrap()));
        }
        if self.right.len() > self.left.len() {
            self.left.push(self.right.pop().unwrap().0)
        }
    }

    fn find_median(&self) -> f64 {
        if self.left.len() > self.right.len() {
            *self.left.peek().unwrap() as f64
        } else {
            (self.right.peek().unwrap().0 + *self.left.peek().unwrap()) as f64 / 2.0
        }
    }
}

// https://leetcode.com/problems/sliding-window-median/description/ todo
pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    fn too_slow(nums: Vec<i32>, k: i32) -> Vec<f64> {
        use std::collections::*;
        let mut window = VecDeque::new();
        let k = k as usize;
        let mut ans = vec![];

        for i in 0..nums.len() {
            window.push_back(nums[i]);

            if window.len() == k {
                let w = window.clone();
                let mut x = w.iter().copied().collect::<Vec<_>>();
                x.sort();
                if k & 1 == 1 {
                    let mid = k / 2;
                    ans.push(x[mid] as f64);
                } else {
                    ans.push((x[k / 2 - 1] as f64 + x[k / 2] as f64) / 2.0);
                }
                window.pop_front();
            }
        }
        ans
    }

    fn two_heaps(nums: Vec<i32>, k: i32) -> Vec<f64> {
        use std::cmp::*;
        use std::collections::*;
        fn insert(n: i32, left: &mut BinaryHeap<i32>, right: &mut BinaryHeap<Reverse<i32>>) {
            if left.is_empty() || *left.peek().unwrap() <= n {
                right.push(Reverse(n));
            } else {
                left.push(n);
            }
            if left.len() > right.len() + 1 {
                right.push(Reverse(left.pop().unwrap()));
            }
            if right.len() > left.len() {
                left.push(right.pop().unwrap().0);
            }
        }
        fn find_median(left: &mut BinaryHeap<i32>, right: &mut BinaryHeap<Reverse<i32>>) -> f64 {
            if left.len() > right.len() {
                *left.peek().unwrap() as f64
            } else {
                (*left.peek().unwrap() as f64 + right.peek().unwrap().0 as f64) / 2.0
            }
        }

        fn remove_from<T: Clone + Ord>(n: &T, heap: &mut BinaryHeap<T>) {
            let mut v = heap.clone().into_sorted_vec();
            v.remove(v.binary_search(n).unwrap());
            heap.clear();
            heap.extend(v);
        }

        fn remove(n: i32, left: &mut BinaryHeap<i32>, right: &mut BinaryHeap<Reverse<i32>>) {
            let median = find_median(left, right);
            if (n as f64) <= median {
                remove_from(&n, left);
            } else {
                remove_from(&Reverse(n), right);
            }
        }

        let mut ans = vec![];
        let mut left = BinaryHeap::new();
        let mut right = BinaryHeap::new();

        for i in 0..k as usize {
            insert(nums[i], &mut left, &mut right);
        }
        ans.push(find_median(&mut left, &mut right));

        for i in k as usize..nums.len() {
            remove(nums[i - k as usize], &mut left, &mut right);
            insert(nums[i], &mut left, &mut right);
            ans.push(find_median(&mut left, &mut right));
        }

        ans
    }
    two_heaps(nums, k)
}

// https://leetcode.com/problems/finding-mk-average/
// The MKAverage can be calculated using these steps:
//
// If the number of the elements in the stream is less than m you should consider the MKAverage to be -1.
// Otherwise, copy the last m elements of the stream to a separate container.
// Remove the smallest k elements and the largest k elements from the container.
// Calculate the average value for the rest of the elements rounded down to the nearest integer.

// Solution:
// The idea for the problem is to maintain a data structure so that it can list the first k numbers' sum and last k numbers' sum quickly.
// It will also have the capability of counting how many elements there are between specific range of the values.
// The data structure has the aformentioned abilities is a Fenwick Tree.
// You can have the implementation of a segment tree as well. But it will overcomplicate the issue.
//
// How do we find the sum of first k elements?
// Essentially, whenever a new element flies in, we have a sliding window to window out an element that is out of the last m elements.
// We can use -1 and -value on the Fenwick tree to undo its effect.
// Now, we want to find the first k elements' sum.
// We can use binary search to find the value such that the sum of the occurence of values less than a given value x is less than k,
// and we will have to find the first spot such property violated.
// We just need to do some simple math to figure out what exactly the sum of the first k elements will be given
// the value of the violator and the value of the sum before the violator.
// And we will get the first k elements' sum.
// For the last k elements, it works the same.
//

// Idea :
//
// We can have a queue to maintain m elements
// Use two Fenwick tree, 1 for count and 1 for prefix sum
// Do 2 times binary search for the first k elements and the last k elements by using the count from our first fenwick tree
// We can get the sum by subtrating the sum of first k elements and sum of last k element by using our second fenwick tree
mod mk_avg_fenwick_tree {
    use std::collections::*;

    const MAX_SIZE: i32 = 100000;

    struct Fenwick {
        buf: Vec<i32>,
    }

    impl Fenwick {
        fn new() -> Self {
            Self {
                buf: vec![0; (MAX_SIZE + 1) as usize],
            }
        }

        fn add(&mut self, mut k: i32, x: i32) {
            while (k as usize) < self.buf.len() {
                self.buf[k as usize] += x;
                k += k & -k;
            }
        }

        fn sum(&mut self, mut k: i32) -> i32 {
            let mut sum = 0;
            while k > 0 {
                sum += self.buf[k as usize];
                k -= k & -k;
            }
            sum
        }

        fn sum_range(&mut self, l: i32, r: i32) -> i32 {
            self.sum(r) - self.sum(l - 1)
        }
    }

    struct MKAverage {
        queue: VecDeque<i32>,
        counts: Fenwick,
        sums: Fenwick,
        sum: i32,
        k: i32,
        m: i32,
    }

    impl MKAverage {
        fn new(m: i32, k: i32) -> Self {
            Self {
                queue: VecDeque::new(),
                counts: Fenwick::new(),
                sums: Fenwick::new(),
                sum: 0,
                k,
                m,
            }
        }

        fn add_element(&mut self, num: i32) {
            self.counts.add(num, 1);
            self.sums.add(num, num);
            self.queue.push_back(num);
            self.sum += num;
            if self.queue.len() > self.m as usize {
                let to_delete = self.queue.pop_front().unwrap();
                self.sum -= to_delete;
                self.counts.add(to_delete, -1);
                self.sums.add(to_delete, -to_delete);
            }
        }

        fn calculate_mk_average(&mut self) -> i32 {
            if self.queue.len() < self.m as usize {
                return -1;
            }

            let mut left_bound = 0;
            let mut right_bound = 0;

            let mut l = 0;
            let mut r = MAX_SIZE;
            while l <= r {
                let mid = l + (r - l) / 2;
                let sum = self.counts.sum_range(0, mid);
                if sum >= self.k {
                    left_bound = mid;
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            }

            let mut l = 0;
            let mut r = MAX_SIZE;
            while l <= r {
                let mid = l + (r - l) / 2;
                let counts = self.counts.sum_range(mid, MAX_SIZE);
                if counts >= self.k {
                    right_bound = mid;
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }

            let mut sum1 = self.sums.sum_range(0, left_bound);
            let mut sum2 = self.sums.sum_range(right_bound, MAX_SIZE);

            let cnt1 = self.counts.sum_range(0, left_bound);
            let cnt2 = self.counts.sum_range(right_bound, MAX_SIZE);

            if cnt1 > self.k {
                sum1 -= left_bound * (cnt1 - self.k);
            }
            if cnt2 > self.k {
                sum2 -= right_bound * (cnt2 - self.k);
            }

            let mid_sum = self.sum - sum1 - sum2;
            let div = self.m - 2 * self.k;
            mid_sum / div
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn fenwick() {
            let mut fenwick = Fenwick::new();
            fenwick.add(9, 10);
            fenwick.add(7, 10);
            println!("{}", fenwick.sum(9) - fenwick.sum(6)); // 20
            println!("{}", fenwick.sum_range(7, 9)); // 20
        }

        #[test]
        fn test_mk_avg() {
            let (m, k) = (3, 1);
            let mut mk_avg = MKAverage::new(m, k);
            // [[3,1],[17612],[74607],[],[8272],[33433],[],[15456],[64938],[],[99741]]
            mk_avg.add_element(17612);
            mk_avg.add_element(74607);
            println!("{}", mk_avg.calculate_mk_average()); // -1
            mk_avg.add_element(8272);
            mk_avg.add_element(33433);
            println!("{}", mk_avg.calculate_mk_average()); // 33433
            mk_avg.add_element(15456);
            mk_avg.add_element(64938);
            println!("{}", mk_avg.calculate_mk_average()); // 33433
            mk_avg.add_element(99741);
        }
    }
}

mod mk_avg_time_simple_tle {
    use std::collections::*;

    pub struct MKAverage {
        seq: VecDeque<i32>,
        m: i32,
        k: i32,
    }

    impl MKAverage {
        pub fn new(m: i32, k: i32) -> Self {
            Self {
                seq: VecDeque::new(),
                m,
                k,
            }
        }

        pub fn add_element(&mut self, num: i32) {
            self.seq.push_back(num);
            if self.seq.len() > self.m as usize {
                self.seq.pop_front();
            }
        }

        pub fn calculate_mk_average(&mut self) -> i32 {
            if self.seq.len() < self.m as usize {
                return -1;
            }
            let mut arr = self.seq.clone().into_iter().collect::<Vec<_>>();
            arr.sort();
            let div = self.m - 2 * self.k;
            let sum = arr[self.k as usize..(self.m as usize - self.k as usize)]
                .iter()
                .sum::<i32>();
            sum / div
        }

        pub fn clear(&mut self) {
            self.seq.clear();
        }
    }
}

mod mk_avg_time_limited {
    use std::cmp::*;
    use std::collections::*;

    pub struct MKAverage {
        seq: VecDeque<i32>,
        buf: BinaryHeap<Reverse<i32>>,
        m: i32,
        k: i32,
    }

    impl MKAverage {
        pub fn new(m: i32, k: i32) -> Self {
            Self {
                seq: VecDeque::new(),
                buf: BinaryHeap::new(),
                m,
                k,
            }
        }

        pub fn add_element(&mut self, num: i32) {
            self.seq.push_back(num);
            self.buf.push(Reverse(num));
            if self.buf.len() > (self.m as usize) {
                let mut vec = self.buf.clone().into_sorted_vec();
                let idx = vec
                    .binary_search(&self.seq.pop_front().as_ref().map(|x| Reverse(*x)).unwrap())
                    .unwrap();
                vec.remove(idx);
                self.buf = vec.into_iter().collect();
            }
        }

        pub fn calculate_mk_average(&mut self) -> i32 {
            if (self.buf.len() as i32) < self.m {
                return -1;
            }
            let len = self.buf.len();
            let mid = len - 2 * self.k as usize;
            let sum = self
                .buf
                .iter()
                .skip(self.k as usize)
                .take(mid)
                .map(|r| r.0)
                .sum::<i32>();
            sum / mid as i32
        }

        pub fn clear(&mut self) {
            self.seq.clear();
            self.buf.clear();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test225() {
        println!("{:?}", add_to_array_form(vec![1, 2, 0, 0], 34)); // 1234
        println!("{:?}", add_to_array_form(vec![2, 7, 4], 181)); // 455
        println!("{:?}", add_to_array_form(vec![2, 1, 5], 806)); // 1021
    }

    #[test]
    fn test226() {
        // Explanation
        // MedianFinder medianFinder = new MedianFinder();
        // medianFinder.addNum(1);    // arr = [1]
        // medianFinder.addNum(2);    // arr = [1, 2]
        // medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
        // medianFinder.addNum(3);    // arr[1, 2, 3]
        // medianFinder.findMedian(); // return 2.0

        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1); // arr = [1]
        median_finder.add_num(2); // arr = [1, 2]
        println!("{}", median_finder.find_median()); // return 1.5 (i.e., (1 + 2) / 2)
        median_finder.add_num(3); // arr[1, 2, 3]
        println!("{}", median_finder.find_median()); // return 2.0
        println!();

        let mut median_finder = MedianFinder::new();
        median_finder.add_num(-1);
        println!("{}", median_finder.find_median()); // return -1
        median_finder.add_num(-2);
        println!("{}", median_finder.find_median()); // return -1.5
        median_finder.add_num(-3);
        println!("{}", median_finder.find_median()); // return -2
        median_finder.add_num(-4);
        println!("{}", median_finder.find_median()); // return -2.5
        median_finder.add_num(-5);
        println!("{}", median_finder.find_median()); // return -3
        println!();

        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1);
        println!("{}", median_finder.find_median()); // return 1
        median_finder.add_num(2);
        println!("{}", median_finder.find_median()); // return 1.5
        median_finder.add_num(3);
        println!("{}", median_finder.find_median()); // return 2
        median_finder.add_num(4);
        println!("{}", median_finder.find_median()); // return 2.5
        median_finder.add_num(5);
        println!("{}", median_finder.find_median()); // return 3
        println!();

        let mut median_finder = MedianFinder::new();
        for i in 1..=100 {
            median_finder.add_num(i);
            println!("{}", median_finder.find_median());
        }
        println!("{:?}", median_finder);
    }

    #[test]
    fn test227() {
        println!(
            "{:?}",
            median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
        ); // [1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000]
        println!(
            "{:?}",
            median_sliding_window(vec![1, 2, 3, 4, 2, 3, 1, 4, 2], 3)
        ); // [2.00000,3.00000,3.00000,3.00000,2.00000,3.00000,2.00000]
        println!(
            "{:?}",
            median_sliding_window(vec![2147483647, 2147483647], 2)
        ); // [2147483647.00000]
    }

    #[test]
    fn test228() {
        let (m, k) = (3, 1);
        let mut mk_avg = mk_avg_time_simple_tle::MKAverage::new(m, k);

        mk_avg.add_element(3);
        mk_avg.add_element(1);
        println!("{}", mk_avg.calculate_mk_average()); // return -1, because m = 3 and only 2 elements exist.
        mk_avg.add_element(10);
        println!("{}", mk_avg.calculate_mk_average()); // The last 3 elements are [3,1,10].
                                                       // After removing smallest and largest 1 element the container will be [3].
                                                       // The average of [3] equals 3/1 = 3, return 3
        mk_avg.add_element(5);
        mk_avg.add_element(5);
        mk_avg.add_element(5);
        println!("{}", mk_avg.calculate_mk_average()); // The last 3 elements are [5,5,5].
                                                       // After removing smallest and largest 1 element the container will be [5].
                                                       // The average of [5] equals 5/1 = 5, return 5

        println!();

        mk_avg.clear();
        // [[3,1],[17612],[74607],[],[8272],[33433],[],[15456],[64938],[],[99741]]
        mk_avg.add_element(17612);
        mk_avg.add_element(74607);
        println!("{}", mk_avg.calculate_mk_average()); // -1
        mk_avg.add_element(8272);
        mk_avg.add_element(33433);
        println!("{}", mk_avg.calculate_mk_average()); // 33433
        mk_avg.add_element(15456);
        mk_avg.add_element(64938);
        println!("{}", mk_avg.calculate_mk_average()); // 33433
        mk_avg.add_element(99741);
    }
}
