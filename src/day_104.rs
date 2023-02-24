// https://leetcode.com/problems/minimize-deviation-in-array/description/
// https://leetcode.com/problems/minimize-deviation-in-array/solutions/955262/c-intuitions-and-flip/
pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
    use std::collections::*;
    let mut heap = BinaryHeap::new();
    let mut result = i32::MAX;
    let mut min_n = i32::MAX;
    for i in 0..nums.len() {
        let n = if nums[i] % 2 == 0 {
            nums[i]
        } else {
            nums[i] * 2
        };
        min_n = min_n.min(n);
        heap.push(n);
    }
    while heap.peek().unwrap() % 2 == 0 {
        let &top = heap.peek().unwrap();
        result = result.min(top - min_n);
        let divided = top / 2;
        min_n = min_n.min(divided);
        heap.push(divided);
        heap.pop();
    }
    result.min(heap.peek().unwrap() - min_n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test289() {
        println!("{}", minimum_deviation(vec![1, 2, 3, 4])); // 1
        println!("{}", minimum_deviation(vec![4, 1, 5, 20, 3])); // 3
    }
}
