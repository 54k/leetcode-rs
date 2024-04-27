// https://leetcode.com/problems/freedom-trail/description
pub fn find_rotate_steps(ring: String, key: String) -> i32 {
    let m = ring.len();
    let n = key.len();
    let ring = ring.as_bytes();
    let key = key.as_bytes();

    let sb = |cur: i32, next: i32| {
        let sbetween = (cur - next).abs();
        let saround = (m as i32 - sbetween);
        saround.min(sbetween)
    };

    let mut dp = vec![0i32; m];
    for i in (0..n).rev() {
        let mut next = vec![i32::MAX; m];
        for j in 0..m {
            for k in 0..m {
                if ring[k] == key[i] {
                    let dist = sb(j as i32, k as i32);
                    next[j] = next[j].min(1 + dist + dp[k]);
                }
            }
        }
        dp = next;
    }
    dp[0]
}
