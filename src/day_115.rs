//https://leetcode.com/problems/count-ways-to-group-overlapping-ranges/description/
pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/subarray-sum-equals-k/description/
// https://leetcode.com/problems/subarray-sum-equals-k/editorial/
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    todo!()
}

// todo https://leetcode.com/problems/replace-non-coprime-numbers-in-array/description/

mod test {
    use super::*;

    #[test]
    fn test327() {
        println!("{}", count_ways(vec![vec![6, 10], vec![5, 15]])); // 2
        println!(
            "{}",
            count_ways(vec![vec![1, 3], vec![10, 20], vec![2, 5], vec![4, 8]])
        ); // 4
    }
}
