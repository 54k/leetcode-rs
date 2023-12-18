// https://leetcode.com/problems/maximum-product-difference-between-two-pairs/description
pub fn max_product_difference(nums: Vec<i32>) -> i32 {
    let mut biggest = 0;
    let mut second_biggest = 0;
    let mut smallest = i32::MAX;
    let mut second_smallest = i32::MAX;

    for n in nums {
        if n > biggest {
            second_biggest = biggest;
            biggest = n;
        } else {
            second_biggest = second_biggest.max(n);
        }
        if n < smallest {
            second_smallest = smallest;
            smallest = n;
        } else {
            second_smallest = second_smallest.min(n);
        }
    }

    biggest * second_biggest - smallest * second_smallest
}
