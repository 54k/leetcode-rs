// https://leetcode.com/problems/max-chunks-to-make-sorted/description/
pub fn max_chunks_to_sorted_i(arr: Vec<i32>) -> i32 {
    if arr.len() == 0 {
        return 0;
    }
    let mut count = 0;
    let mut max = vec![0; arr.len()];
    max[0] = arr[0];
    for i in 1..arr.len() {
        max[i] = max[i - 1].max(arr[i]);
    }

    for i in 0..arr.len() {
        if max[i] == i as i32 {
            count += 1;
        }
    }
    count
}

pub fn max_chunks_to_sorted_ii(arr: Vec<i32>) -> i32 {
    let mut cur = 0;
    let mut chunks = 0;
    for i in 0..arr.len() {
        cur += arr[i];
        if cur == (i as i32 * (i as i32 + 1)) / 2 {
            chunks += 1;
        }
    }
    chunks
}
