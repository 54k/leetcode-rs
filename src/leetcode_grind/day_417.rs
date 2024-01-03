// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/description
pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut prev = 0;
    let mut ans = 0;

    for s in bank {
        let mut count = 0;

        for ch in s.chars() {
            if ch == '1' {
                count += 1;
            }
        }

        if count > 0 {
            ans += prev * count;
            prev = count;
        }
    }

    ans
}
