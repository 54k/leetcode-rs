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

// https://leetcode.com/problems/longest-increasing-subsequence-ii/
// https://leetcode.com/problems/longest-increasing-subsequence-ii/solutions/2560085/python-explanation-with-pictures-segment-tree/
pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
    struct SegmentTree {
        tree: Vec<i32>,
        len: usize,
    }
    impl SegmentTree {
        fn new(len: usize) -> Self {
            Self {
                tree: vec![0; len * 2],
                len,
            }
        }
        fn add(&mut self, mut k: usize, val: i32) {
            k += self.len;
            self.tree[k] = val;
            k /= 2;
            while k > 1 {
                self.tree[k] = self.tree[2 * k].max(self.tree[2 * k + 1]);
                k /= 2;
            }
        }
        fn query(&self, mut l: usize, mut r: usize) -> i32 {
            l += self.len;
            r += self.len;
            let mut ans = 0;
            while l < r {
                if l % 2 == 1 {
                    ans = ans.max(self.tree[l]);
                    l += 1;
                }
                if r % 2 == 1 {
                    r -= 1;
                    ans = ans.max(self.tree[r]);
                }
                l /= 2;
                r /= 2;
            }
            ans
        }
    }

    let n = nums.iter().copied().max().unwrap();
    let mut ans = 1;
    let mut seg = SegmentTree::new(n as usize);
    for mut a in nums {
        a -= 1;
        let pre_max = seg.query(0.max(a - k) as usize, a as usize);
        ans = ans.max(pre_max + 1);
        seg.add(a as usize, pre_max + 1);
    }
    ans
}
