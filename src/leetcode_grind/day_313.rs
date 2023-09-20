pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
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

#[test]
fn test_median_of_sorted() {
    let res = find_median_sorted_arrays(vec![1, 3], vec![2]);
    println!("{res}");
}
