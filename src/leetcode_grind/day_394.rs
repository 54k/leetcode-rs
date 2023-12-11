// https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array/description/
pub fn find_special_integer_1(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    let n = arr.len() as i32;
    for num in arr {
        *freq.entry(num).or_insert(0) += 1;
    }
    for (k, v) in freq {
        if v > n / 4 {
            return k;
        }
    }
    -1
}

pub fn find_special_integer_2(arr: Vec<i32>) -> i32 {
    let size = arr.len() / 4;
    for i in 0..arr.len() - size {
        if arr[i] == arr[i + size] {
            return arr[i];
        }
    }
    -1
}

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

// https://leetcode.com/problems/range-addition/description/
pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
    let mut arr = vec![0; length as usize];
    for upd in updates {
        let (l, r, v) = (upd[0] as usize, upd[1] as usize, upd[2]);
        arr[l] += v;
        if r < arr.len() - 1 {
            arr[r + 1] -= v;
        }
    }
    for i in 1..arr.len() {
        arr[i] += arr[i - 1];
    }
    arr
}

// https://leetcode.com/problems/range-addition-ii/description/
pub fn max_count(mut m: i32, mut n: i32, ops: Vec<Vec<i32>>) -> i32 {
    for op in ops {
        m = m.min(op[0]);
        n = n.min(op[1]);
    }
    m * n
}
