// https://leetcode.com/problems/sum-of-subarray-ranges/description/
pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
    0
}

// https://leetcode.com/problems/sum-of-total-strength-of-wizards/description/
pub fn total_strength(strength: Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test296() {
        println!("{}", sub_array_ranges(vec![1, 2, 3])); // 4
        println!("{}", sub_array_ranges(vec![1, 3, 3])); // 4
        println!("{}", sub_array_ranges(vec![4, -2, -3, 4, 1])); // 59
    }

    #[test]
    fn test297() {
        println!("{}", total_strength(vec![1, 3, 1, 2])); // 44
        println!("{}", total_strength(vec![5, 4, 6])); // 213
    }
}
