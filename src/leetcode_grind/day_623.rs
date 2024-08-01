// https://leetcode.com/problems/number-of-senior-citizens/description/?envType=daily-question&envId=2024-08-01
pub fn count_seniors(details: Vec<String>) -> i32 {
    let mut ans = 0;
    for d in details {
        if &d[11..13] > "60" {
            ans += 1;
        }
    }
    ans
}
