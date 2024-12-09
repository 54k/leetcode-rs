// https://leetcode.com/problems/cutting-ribbons/description/
pub fn max_length(ribbons: Vec<i32>, k: i32) -> i32 {
    let mut lo = 0;
    let mut hi = *ribbons.iter().max().unwrap_or(&0) + 1;

    fn check(ribbons: &Vec<i32>, mut k: i32, cut: i32) -> bool {
        let mut i = 0;
        while k > 0 && i < ribbons.len() {
            k -= ribbons[i] / cut;
            i += 1;
        }
        k <= 0
    }

    while lo + 1 < hi {
        let mid = lo + (hi - lo) / 2;
        if check(&ribbons, k, mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    lo
}

// https://leetcode.com/problems/couples-holding-hands/description/
pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in (0..row.len()).step_by(2) {
        let x = row[i];
        if row[i + 1] == (x ^ 1) {
            continue;
        }
        ans += 1;
        for j in i + 1..row.len() {
            if row[j] == (x ^ 1) {
                row[j] = row[i + 1];
                row[i + 1] = x ^ 1;
                break;
            }
        }
    }
    ans
}
