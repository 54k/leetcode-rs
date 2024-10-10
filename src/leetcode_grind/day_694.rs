// https://leetcode.com/problems/maximum-width-ramp/description/?envType=daily-question&envId=2024-10-10
pub fn max_width_ramp_i(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    let mut ind = vec![0; n];
    for i in 0..n {
        ind[i] = i;
    }
    ind.sort_by(|&i, j| {
        if nums[i] == nums[*j] {
            return i.cmp(j);
        }
        nums[i].cmp(&nums[*j])
    });
    let mut mini = n;
    let mut ans = 0;
    for i in ind {
        ans = ans.max(i as i32 - mini as i32);
        mini = mini.min(i);
    }
    ans
}

pub fn max_width_ramp_ii(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut right_max = vec![nums[n - 1]; n];
    let mut mx = nums[n - 1];
    for i in (0..n - 1).rev() {
        right_max[i] = mx.max(nums[i]);
        mx = right_max[i];
    }

    let mut left = 0;
    let mut right = 0;
    let mut max_width = 0;
    while right < n {
        while left < right && nums[left] > right_max[right] {
            left += 1;
        }
        max_width = max_width.max(right as i32 - left as i32);
        right += 1;
    }

    max_width
}

pub fn max_width_ramp_iii(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut mono = vec![];
    for i in 0..n {
        if mono.is_empty() || nums[mono[mono.len() - 1]] >= nums[i] {
            mono.push(i);
        }
    }
    println!("{:?}", mono);
    let mut max_width = 0;
    for j in (0..n).rev() {
        while !mono.is_empty() && nums[mono[mono.len() - 1]] <= nums[j] {
            max_width = max_width.max(j - mono.pop().unwrap());
        }
    }
    max_width as i32
}

// https://leetcode.com/problems/minimum-time-to-build-blocks/description/
pub fn min_build_time(mut blocks: Vec<i32>, split: i32) -> i32 {
    let n = blocks.len();
    blocks.sort_by(|a, b| b.cmp(a));
    let mut dp = vec![0; n + 1];
    dp[0] = i32::MAX;
    for b in (0..n).rev() {
        for w in (1..n + 1).rev() {
            if w >= n - b {
                dp[w] = blocks[b];
                continue;
            }

            let work_here = blocks[b].max(dp[w - 1]);
            let split_here = split + dp[(2 * w).min(n - b)];
            dp[w] = work_here.min(split_here);
        }
    }
    dp[1]
}
