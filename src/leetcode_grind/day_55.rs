#[allow(dead_code)]
pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
    costs.sort();
    let mut ans = 0;
    for cost in costs {
        if coins >= cost {
            coins -= cost;
            ans += 1;
        } else {
            break;
        }
    }
    ans
}

#[allow(dead_code)]
pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    use std::collections::HashMap;
    target.into_iter().fold(HashMap::new(), |mut acc, v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    }) == arr.into_iter().fold(HashMap::new(), |mut acc, v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    })
}

// https://leetcode.com/problems/repeated-dna-sequences/solutions/53857/java-beats-97-moving-window-and-real-bits-manipulation/?orderBy=most_relevant
#[allow(dead_code)]
pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    use std::collections::HashSet;
    if s.len() < 10 {
        return vec![];
    }
    let mut seen = HashSet::new();
    let mut repeated = HashSet::new();
    let s = s.chars().collect::<Vec<_>>();
    for i in 0..s.len() - 9 {
        let x = &s[i..i + 10];
        if !seen.insert(x) {
            repeated.insert(x.iter().collect::<String>());
        }
    }
    repeated.into_iter().collect()
}

// https://leetcode.com/problems/maximum-length-of-repeated-subarray/solutions/127407/maximum-length-of-repeated-subarray/?orderBy=most_relevant
#[allow(dead_code)]
pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
    for i in (0..nums1.len()).rev() {
        for j in (0..nums2.len()).rev() {
            if nums1[i] == nums2[j] {
                dp[i][j] = dp[i + 1][j + 1] + 1;
                if ans < dp[i][j] {
                    ans = dp[i][j];
                }
            }
        }
    }
    ans
}
