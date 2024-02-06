// https://leetcode.com/problems/beautiful-arrangement/description/
pub fn count_arrangement(n: i32) -> i32 {
    fn perm(pos: i32, n: i32, ans: &mut i32, vis: i32) {
        if pos > n {
            *ans += 1;
        }

        for i in 1..=n {
            if (vis & (1 << i)) == 0 && (pos % i == 0 || i % pos == 0) {
                perm(pos + 1, n, ans, vis | (1 << i));
            }
        }
    }

    let mut ans = 0;
    perm(1, n, &mut ans, 0);
    ans
}
