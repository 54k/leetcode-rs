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

// https://leetcode.com/problems/beautiful-arrangement-ii/description/
pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
    let mut ans = vec![0; n as usize];
    let mut c = 0;
    for v in 1..n - k {
        ans[c] = v;
        c += 1;
    }
    for i in 0..=k {
        ans[c] = if i % 2 == 0 { n - k + i / 2 } else { n - i / 2 };
        c += 1;
    }
    ans
}

// https://leetcode.com/problems/determine-color-of-a-chessboard-square/description/
pub fn square_is_white(coordinates: String) -> bool {
    let c = coordinates.as_bytes();
    let f = c[0] as usize - 'a' as usize;
    let s = c[1] as usize - '1' as usize;

    let mut color = f % 2 == 1;
    if s % 2 == 1 {
        color = !color;
    }
    color
}
