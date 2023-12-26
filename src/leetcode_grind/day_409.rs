// https://leetcode.com/problems/maximum-length-of-pair-chain/description/
pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
    let mut pairs = pairs;
    pairs.sort_by_key(|x| x[1]);
    let mut ans = 0;
    let mut tail = i32::MIN;
    for pair in pairs {
        if pair[0] > tail {
            ans += 1;
            tail = pair[1];
        }
    }
    ans
}

// https://leetcode.com/problems/intersection-of-two-arrays-ii/
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];

    let mut nums1 = nums1;
    nums1.sort();
    let mut nums2 = nums2;
    nums2.sort();

    let mut i = 0;
    let mut j = 0;

    while i < nums1.len() && j < nums2.len() {
        if nums1[i] < nums2[j] {
            i += 1;
        } else if nums1[i] > nums2[j] {
            j += 1;
        } else {
            ans.push(nums1[i]);
            i += 1;
            j += 1;
        }
    }

    ans
}
