// https://leetcode.com/submissions/detail/898152176/
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
}
