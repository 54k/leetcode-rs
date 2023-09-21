pub fn find_median_sorted_arrays_naive(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged = vec![0; nums1.len() + nums2.len()];
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] <= nums2[j] {
            merged[k] = nums1[i];
            i += 1;
        } else {
            merged[k] = nums2[j];
            j += 1;
        }
        k += 1;
    }

    while i < nums1.len() {
        merged[k] = nums1[i];
        i += 1;
        k += 1;
    }

    while j < nums2.len() {
        merged[k] = nums2[j];
        j += 1;
        k += 1;
    }

    if k % 2 == 0 {
        (merged[k / 2 - 1] as f64 + merged[k / 2] as f64) / 2.0
    } else {
        merged[k / 2] as f64
    }
}

pub fn find_median_sorted_arrays_bin_search(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    fn solve(
        a: &Vec<i32>,
        b: &Vec<i32>,
        k: i32,
        a_start: i32,
        a_end: i32,
        b_start: i32,
        b_end: i32,
    ) -> i32 {
        if a_start > a_end {
            return b[k as usize - a_start as usize];
        }

        if b_start > b_end {
            return a[k as usize - b_start as usize];
        }

        let (a_index, b_index) = ((a_start + a_end) / 2, (b_start + b_end) / 2);
        let (a_value, b_value) = (a[a_index as usize], b[b_index as usize]);

        if a_index + b_index < k {
            if a_value > b_value {
                return solve(a, b, k, a_start, a_end, b_index + 1, b_end);
            } else {
                return solve(a, b, k, a_index + 1, a_end, b_start, b_end);
            }
        }

        if a_value > b_value {
            return solve(a, b, k, a_start, a_index - 1, b_start, b_end);
        } else {
            return solve(a, b, k, a_start, a_end, b_start, b_index - 1);
        }
    }

    let (na, nb) = (nums1.len() as i32, nums2.len() as i32);
    let n = na + nb;

    if n % 2 == 1 {
        return solve(&nums1, &nums2, n / 2, 0, na - 1, 0, nb - 1) as f64;
    } else {
        let a = solve(&nums1, &nums2, n / 2, 0, na - 1, 0, nb - 1) as f64;
        let b = solve(&nums1, &nums2, n / 2 - 1, 0, na - 1, 0, nb - 1) as f64;
        return (a + b) / 2.0;
    }
}

pub fn find_median_sorted_arrays_bin_search_optimized(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    pub fn solve(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return solve(nums2, nums1);
        }

        let m = nums1.len() as i32;
        let n = nums2.len() as i32;
        let mut left = 0;
        let mut right = m as i32;

        while left <= right {
            let partition_a = (left + right) / 2;
            let parition_b = (m + n + 1) / 2 - partition_a;

            let max_left_a = if partition_a == 0 {
                i32::MIN
            } else {
                nums1[partition_a as usize - 1]
            };

            let min_right_a = if partition_a == m {
                i32::MAX
            } else {
                nums1[partition_a as usize]
            };

            let max_left_b = if parition_b == 0 {
                i32::MIN
            } else {
                nums2[parition_b as usize - 1]
            };

            let min_right_b = if parition_b == n {
                i32::MAX
            } else {
                nums2[parition_b as usize]
            };

            if max_left_a <= min_right_b && max_left_b <= min_right_a {
                if (m + n) % 2 == 0 {
                    return (max_left_a.max(max_left_b) as f64
                        + min_right_a.min(min_right_b) as f64)
                        / 2.0;
                } else {
                    return max_left_a.max(max_left_b) as f64;
                }
            } else if max_left_a > min_right_b {
                right = partition_a - 1;
            } else {
                left = partition_a + 1;
            }
        }

        0.0
    }

    solve(nums1, nums2)
}

#[test]
fn test_median_of_sorted() {
    let res = find_median_sorted_arrays_bin_search(vec![1, 3], vec![2]);
    println!("{res}");
}

pub fn matrix_median(grid: Vec<Vec<i32>>) -> i32 {
    fn bin_search(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] >= target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let target = m * n / 2;

    let mut left = 1;
    let mut right = 10e6 as i32;

    while left <= right {
        let mid = left + (right - left) / 2;

        let mut found_pos = 0;
        for col in &grid {
            found_pos += bin_search(col, mid);
        }

        if found_pos > target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    right
}

#[test]
fn test_matrix_median() {
    let res = matrix_median(vec![vec![1, 1, 2], vec![2, 3, 3], vec![1, 3, 4]]);
    println!("{res}"); // 2
}
