// Algorithm
//
// Iterate over the integers in the array tasks, and for each integer store the frequency in the map freq.
//
// Initialize the answer variable minimumRounds to 0.
//
// Iterate over the frequencies in the map freq and for each frequency count:
//
// If count is 1, then we should stop and return -1.
// Add count / 3 to the answer variable minimumRounds, if count is divisible by 333.
// Otherwise, add count / 3 + 1 to minimumRounds.
// Return minimumRounds.
#[allow(dead_code)]
pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut m = HashMap::new();
    for t in tasks {
        *m.entry(t).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (_, v) in m {
        if v == 1 {
            return -1;
        }
        match v {
            count if v % 3 == 0 => ans += count / 3,
            count => ans += count / 3 + 1,
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test145() {
        println!("{}", minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]));
    }
}
