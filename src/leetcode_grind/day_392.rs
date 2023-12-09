// https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/description/
pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let n = card_points.len();
    let mut prefix = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] += prefix[i] + card_points[i];
    }

    let mut ans = 0;
    let mut curr_sum = 0;

    for i in n - k as usize..=n {
        curr_sum = prefix[i] - prefix[i - (n - k as usize)];
        ans = ans.max(prefix[n] - curr_sum);
    }

    ans
}
