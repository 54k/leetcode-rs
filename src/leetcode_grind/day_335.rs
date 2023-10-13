// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/
pub fn min_swaps(nums: Vec<i32>) -> i32 {
    let ones = nums.iter().copied().sum::<i32>();
    let mut ans = i32::MAX;
    let mut curr = 0;
    let mut j = 0;
    for i in 0..nums.len() * 2 {
        curr += nums[i % nums.len()];
        if i >= ones as usize {
            curr -= nums[j % nums.len()];
            j += 1;
        }
        ans = ans.min(ones - curr);
    }
    ans
}

// https://leetcode.com/problems/time-needed-to-rearrange-a-binary-string/description/
pub fn seconds_to_remove_occurrences(s: String) -> i32 {
    let mut s = s.chars().collect::<Vec<_>>();
    let mut time = 0;

    loop {
        let mut changed = false;
        let mut i = 0;
        while i < s.len() - 1 {
            if s[i] == '0' && s[i + 1] == '1' {
                s.swap(i + 1, i);
                changed = true;
                i += 1;
            }
            i += 1;
        }
        if !changed {
            break;
        }
        time += 1;
    }

    time
}

#[test]
fn test_seconds_to_remove_occurrences() {
    let res = seconds_to_remove_occurrences("001011".to_string());
    println!("{res}"); // 4
}
