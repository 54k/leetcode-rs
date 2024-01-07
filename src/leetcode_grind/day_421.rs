// https://leetcode.com/problems/maximum-difference-between-increasing-elements/description/
pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    let mut diff = i32::MIN;
    let mut mn = i32::MAX;
    for n in nums {
        if n <= mn {
            mn = n;
        } else {
            diff = diff.max(n - mn);
        }
    }
    if diff == i32::MIN {
        return -1;
    }
    diff
}

// https://leetcode.com/problems/two-furthest-houses-with-different-colors/description/
pub fn max_distance(colors: Vec<i32>) -> i32 {
    let cur = (colors[0], 0);
    let mut diff = 0;
    for (i, &color) in colors.iter().enumerate().skip(1) {
        if color != cur.0 {
            diff = diff.max(i as i32 - cur.1);
        }
    }

    let cur = (*colors.last().unwrap(), colors.len() as i32 - 1);
    for (i, &color) in colors.iter().enumerate().rev().skip(1) {
        if color != cur.0 {
            diff = diff.max(cur.1 - i as i32);
        }
    }

    diff
}
