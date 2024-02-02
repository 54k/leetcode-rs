// https://leetcode.com/problems/sequential-digits/description/
pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    fn build_el(dn: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=dn {
            ans = ans * 10 + i;
        }
        ans
    }

    fn build_ones(dn: i32) -> i32 {
        let mut ans = 0;
        for _ in 0..dn {
            ans = ans * 10 + 1;
        }
        ans
    }

    let mut len = 2;
    let mut ones = build_ones(len);
    let mut el = build_el(2);
    let mut ans = vec![];

    while el <= high {
        if el >= low {
            ans.push(el);
        }
        if el % 10 == 9 {
            len += 1;
            el = build_el(len);
            ones = build_ones(len);
        } else {
            el += ones;
        }
    }

    ans
}
