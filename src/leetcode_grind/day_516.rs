// https://leetcode.com/problems/trapping-rain-water/description/
pub fn trap_i(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut left_max = 0;
    let mut right_max = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                ans += left_max - height[left];
            }
            left += 1
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                ans += right_max - height[right];
            }
            right -= 1;
        }
    }

    ans
}

pub fn trap_ii(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut st = vec![];
    let mut ans = 0;
    for i in 0..n {
        while !st.is_empty() && height[i] > height[*st.last().unwrap()] {
            let top = st.pop().unwrap();
            if st.is_empty() {
                break;
            }
            let distance = i - *st.last().unwrap() - 1;
            let bounded_height = height[i].min(height[*st.last().unwrap()]) - height[top];
            ans += distance as i32 * bounded_height;
        }
        st.push(i);
    }
    ans
}
