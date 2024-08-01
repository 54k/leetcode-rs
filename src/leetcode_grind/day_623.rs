// https://leetcode.com/problems/number-of-senior-citizens/description/?envType=daily-question&envId=2024-08-01
pub fn count_seniors(details: Vec<String>) -> i32 {
    let mut ans = 0;
    for d in details {
        if &d[11..13] > "60" {
            ans += 1;
        }
    }
    ans
}

// https://leetcode.com/problems/trapping-rain-water/description/
pub fn trap(height: Vec<i32>) -> i32 {
    if height.len() == 0 {
        return 0;
    }
    let mut ans = 0;
    let size = height.len();
    let mut left_max = vec![0; size];
    let mut right_max = vec![0; size];

    left_max[0] = height[0];
    for i in 1..size {
        left_max[i] = height[i].max(left_max[i - 1]);
    }

    right_max[size - 1] = height[size - 1];
    for i in (0..size - 1).rev() {
        right_max[i] = height[i].max(right_max[i + 1]);
    }

    for i in 1..size - 1 {
        ans += left_max[i].min(right_max[i]) - height[i];
    }
    ans
}
