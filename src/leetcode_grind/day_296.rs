// https://leetcode.com/problems/sort-an-array/description/
pub fn sort_array_qsort(mut nums: Vec<i32>) -> Vec<i32> {
    extern "C" {
        fn random() -> i32;
    }

    fn sort(nums: &mut Vec<i32>, i: i32, j: i32) {
        fn partition(nums: &mut Vec<i32>, i: usize, j: usize) -> i32 {
            let rpivot = unsafe { random() as usize % (j - i + 1) + i };
            nums.swap(j, rpivot);

            let ppoint = nums[j];

            let mut left = i;
            for k in i..j {
                if nums[k] <= ppoint {
                    nums.swap(left, k);
                    left += 1;
                }
            }
            nums.swap(left, j);
            left as i32
        }

        if i >= j {
            return;
        }

        let ppoint = partition(nums, i as usize, j as usize);
        sort(nums, i, ppoint - 1);
        sort(nums, ppoint, j);
    }

    let n = nums.len();
    sort(&mut nums, 0, n as i32 - 1);
    nums
}

pub fn sort_array_merge_sort_i(mut nums: Vec<i32>) -> Vec<i32> {
    fn merge_sort(nums: &mut Vec<i32>, tmp_arr: &mut Vec<i32>, left: usize, right: usize) {
        fn merge(
            nums: &mut Vec<i32>,
            tmp_arr: &mut Vec<i32>,
            left: usize,
            mid: usize,
            right: usize,
        ) {
            let start1 = left;
            let start2 = mid + 1;
            let n1 = mid - left + 1;
            let n2 = right - mid;

            for i in 0..n1 {
                tmp_arr[start1 + i] = nums[start1 + i];
            }
            for i in 0..n2 {
                tmp_arr[start2 + i] = nums[start2 + i];
            }

            let mut i = 0;
            let mut j = 0;
            let mut k = left;
            while i < n1 && j < n2 {
                if tmp_arr[start1 + i] <= tmp_arr[start2 + j] {
                    nums[k] = tmp_arr[start1 + i];
                    i += 1;
                } else {
                    nums[k] = tmp_arr[start2 + j];
                    j += 1;
                }
                k += 1;
            }

            while i < n1 {
                nums[k] = tmp_arr[start1 + i];
                i += 1;
                k += 1;
            }
            while j < n2 {
                nums[k] = tmp_arr[start2 + j];
                j += 1;
                k += 1;
            }
        }

        if left >= right {
            return;
        }

        let mid = (left + right) / 2;
        merge_sort(nums, tmp_arr, left, mid);
        merge_sort(nums, tmp_arr, mid + 1, right);
        merge(nums, tmp_arr, left, mid, right);
    }

    let n = nums.len();
    let mut tmp_arr = vec![0; nums.len()];
    merge_sort(&mut nums, &mut tmp_arr, 0, n - 1);
    nums
}

pub fn sort_array_merge_sort_ii(mut nums: Vec<i32>) -> Vec<i32> {
    fn merge_sort(nums: &mut Vec<i32>) {
        fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
            let mut merged = Vec::new();
            let mut i = 0;
            let mut j = 0;
            while i < left.len() && j < right.len() {
                if left[i] <= right[j] {
                    merged.push(left[i]);
                    i += 1
                } else {
                    merged.push(right[j]);
                    j += 1;
                }
            }
            while i < left.len() {
                merged.push(left[i]);
                i += 1;
            }
            while j < right.len() {
                merged.push(right[j]);
                j += 1;
            }
            merged
        }

        if nums.len() <= 1 {
            return;
        }

        let mid = nums.len() / 2;
        let mut left = vec![];
        let mut right = vec![];
        for k in 0..mid {
            left.push(nums[k]);
        }
        for k in mid..nums.len() {
            right.push(nums[k]);
        }

        merge_sort(&mut left);
        merge_sort(&mut right);

        *nums = merge(&left, &right);
    }

    merge_sort(&mut nums);
    nums
}
