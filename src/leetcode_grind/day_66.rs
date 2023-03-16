pub fn min_flips_mono_incr(s: String) -> i32 {
    let mut ans = 0;
    let mut num = 0;
    for c in s.chars() {
        if c == '0' {
            ans = std::cmp::min(num, ans + 1)
        } else {
            num += 1
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test161() {
        println!("{}", min_flips_mono_incr("00011000".to_string())); // 2
    }
}
