// https://leetcode.com/problems/find-the-longest-valid-obstacle-course-at-each-position/description/
pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
    fn bisect(arr: &Vec<i32>, target: i32, mut right: usize) -> usize {
        if right == 0 {
            return 0;
        }
        let mut left = 0;
        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
    let n = obstacles.len();
    let mut lis_len = 0;
    let mut ans = vec![0; n];
    let mut lis = vec![0; n];

    for i in 0..n {
        let height = obstacles[i];
        let idx = bisect(&lis, height, lis_len);
        if idx == lis_len {
            lis_len += 1;
        }
        lis[idx] = height;
        ans[i] = idx as i32 + 1;
    }
    ans
}

