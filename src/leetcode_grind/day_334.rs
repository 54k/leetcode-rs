// https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/description/
pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    let mut left = vec![1; nums.len()];
    for i in 0..n {
        for j in 0..i {
            if nums[i] > nums[j] {
                left[i] = left[i].max(1 + left[j]);
            }
        }
    }

    let mut right = vec![1; nums.len()];
    for i in (0..n).rev() {
        for j in (i + 1..n).rev() {
            if nums[i] > nums[j] {
                right[i] = right[i].max(1 + right[j]);
            }
        }
    }

    let mut max_len = 0;
    for i in 0..n {
        if left[i] >= 2 && right[i] >= 2 {
            max_len = max_len.max(left[i] + right[i] - 1);
        }
    }

    n as i32 - max_len
}

// https://leetcode.com/problems/find-in-mountain-array/description
pub struct MountainArray;
impl MountainArray {
    fn get(&self, index: i32) -> i32 {
        0
    }
    fn length(&self) -> i32 {
        0
    }
}

pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
    let mut lo = 0;
    let mut hi = mountainArr.length() as i32 - 1;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if mountainArr.get(mid) < mountainArr.get(mid + 1) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let peak_idx = lo;

    lo = 0;
    hi = peak_idx;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if mountainArr.get(mid) < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if mountainArr.get(lo) != target {
        lo = peak_idx + 1;
        hi = mountainArr.length() - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if mountainArr.get(mid) > target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if mountainArr.get(lo) != target {
            return -1;
        }
        return lo;
    }
    lo
}

// https://leetcode.com/problems/longest-mountain-in-array/description/
pub fn longest_mountain(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut ans = 0;
    let mut base = 0;
    while base < n {
        let mut end = base;
        if end + 1 < n && arr[end] < arr[end + 1] {
            while end + 1 < n && arr[end] < arr[end + 1] {
                end += 1;
            }

            if end + 1 < n && arr[end] > arr[end + 1] {
                while end + 1 < n && arr[end] > arr[end + 1] {
                    end += 1;
                }
                ans = ans.max(end - base + 1)
            }
        }

        base = end.max(base + 1)
    }
    ans as i32
}
