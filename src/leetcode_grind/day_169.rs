// https://leetcode.com/problems/height-checker/description/
pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut h = heights.clone();
    h.sort();
    let mut ans = 0;
    for i in 0..heights.len() {
        if h[i] != heights[i] {
            ans += 1;
        }
    }
    ans
}

// https://leetcode.com/explore/learn/card/fun-with-arrays/523/conclusion/3230/
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut cz = 0;

    let mut left = 0;
    for right in 0..nums.len() {
        if nums[right] == 0 {
            cz += 1;
        }
        while cz > 1 {
            if nums[left] == 0 {
                cz -= 1;
            }
            left += 1;
        }
        ans = ans.max(right - left + 1);
    }
    ans as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test465() {
        println!("{}", height_checker(vec![1, 1, 4, 2, 1, 3])); // 3
    }

    #[test]
    fn test466() {
        println!("{}", find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1])); // 4
        println!("{}", find_max_consecutive_ones(vec![1, 0, 1, 1, 0])); // 4
    }
}
