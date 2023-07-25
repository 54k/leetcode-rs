// https://leetcode.com/problems/peak-index-in-a-mountain-array/description/
pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let (mut lo, mut hi) = (0, arr.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        if arr[mid] < arr[mid + 1] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo as i32
}
