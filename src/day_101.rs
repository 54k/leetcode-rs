// https://leetcode.com/problems/single-element-in-a-sorted-array/description/
pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    fn linear(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for x in nums {
            ans ^= x;
        }
        ans
    }
    fn log_n(nums: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            /* if even then check next, if odd then check previous*/
            if nums[mid] == nums[mid ^ 1] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        nums[lo]
    }
    log_n(nums)
}

// https://leetcode.com/problems/queens-that-can-attack-the-king/description/
pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

// https://leetcode.com/problems/reverse-string-ii/description/
pub fn reverse_str(s: String, k: i32) -> String {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test261() {
        println!("{}", single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8])); // 2
        println!("{}", single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11])); // 10
    }

    #[test]
    fn test262() {}

    #[test]
    fn test263() {}
}
