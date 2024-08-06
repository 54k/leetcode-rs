// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/description/
pub fn minimum_pushes(word: String) -> i32 {
    let word = word.chars().collect::<Vec<_>>();
    let mut cnt = vec![0; 26];
    for &c in &word {
        cnt[c as usize - 'a' as usize] += 1;
    }
    cnt.sort();
    cnt.reverse();
    let mut ans = 0;
    for i in 0..26 {
        if cnt[i] == 0 {
            break;
        }
        ans += (i / 8 + 1) as i32 * cnt[i];
    }
    ans as i32
}

// https://leetcode.com/problems/trapping-rain-water/description/
pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut left = 0;
    let mut right = n - 1;
    let mut left_max = 0;
    let mut right_max = 0;
    let mut ans = 0;
    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                ans += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right].max(right_max);
            } else {
                ans += right_max - height[right];
            }
            right -= 1;
        }
    }
    ans
}

