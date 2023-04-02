// https://leetcode.com/problems/binary-search/description/
// https://leetcode.com/problems/binary-search/editorial/
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo = 0 as i32;
    let mut hi = nums.len() as i32 - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid as usize] == target {
            return mid;
        } else if nums[mid as usize] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test389() {
        println!(
            "{}",
            search(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 7)
        ); // 6
    }
}
