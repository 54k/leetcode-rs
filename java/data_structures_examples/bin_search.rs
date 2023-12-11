// https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array/description/
pub fn find_special_integer_3(arr: Vec<i32>) -> i32 {
    fn upper_bound(arr: &Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = arr.len();

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if arr[mid] > target {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
    }

    fn lower_bound(arr: &Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = arr.len();

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if arr[mid] >= target {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
    }

    let n = arr.len();
    let candidates = vec![arr[n / 4], arr[n / 2], arr[3 * n / 4]];
    let target = n / 4;

    for c in candidates {
        let left = lower_bound(&arr, c);
        let right = upper_bound(&arr, c) - 1;

        if right - left + 1 > target as i32 {
            return c;
        }
    }

    -1
}
