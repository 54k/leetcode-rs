pub fn count_odds(low: i32, high: i32) -> i32 {
    fn my(low: i32, high: i32) -> i32 {
        let mut ans = 0;
        let mut low_odd = false;
        let mut high_odd = false;
        if low & 1 == 1 {
            ans += 1;
            low_odd = true;
        }
        if high & 1 == 1 {
            ans += 1;
            high_odd = true;
        }
        ans += (high - low) / 2;
        if low_odd && high_odd {
            ans -= 1;
        }
        ans
    }
    fn from_leetcode(mut low: i32, mut high: i32) -> i32 {
        // If low is even, make it odd.
        if (low & 1) == 0 {
            low += 1;
        }

        // low could become greater than high due to incrementation
        // if it is, the answer is 0; otherwise, use the formula.
        if low > high {
            0
        } else {
            (high - low) / 2 + 1
        }
    }
    from_leetcode(low, high)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test222() {
        println!("{}", count_odds(3, 7)); // 3 [3, 5, 7]
        println!("{}", count_odds(3, 6)); // 2 [3, 5]
        println!("{}", count_odds(8, 10)); // 1 [9]
        println!("{}", count_odds(4, 9)); // 3 [5,7,9]
    }
}
