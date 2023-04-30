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

// https://leetcode.com/problems/third-maximum-number/description/
pub fn third_max(nums: Vec<i32>) -> i32 {
    let k = 3;
    let mut max = vec![i64::MIN; k];
    for num in nums {
        let mut t = num as i64;
        for i in 0..k {
            if max[i] < t {
                let t1 = max[i];
                max[i] = t;
                t = t1;
            } else if max[i] == t {
                break;
            }
        }
    }
    if max[k - 1] > i64::MIN {
        max[k - 1] as i32
    } else {
        max[0] as i32
    }
}

// https://leetcode.com/problems/largest-number-at-least-twice-of-others/description/
pub fn dominant_index(nums: Vec<i32>) -> i32 {
    let mut max = nums.iter().copied().max().unwrap();
    let mut ans = 0;
    for (i, &num) in nums.iter().enumerate() {
        if num != max {
            if num * 2 > max {
                return -1;
            }
        } else {
            ans = i;
        }
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

    #[test]
    fn test467() {
        println!("{}", third_max(vec![3, 2, 1])); // 1
        println!("{}", third_max(vec![1, 2])); // 2
        println!("{}", third_max(vec![2, 2, 3, 1])); // 1
    }

    #[test]
    fn test468() {
        println!("{}", dominant_index(vec![3, 6, 1, 0])); // 1
    }
}
