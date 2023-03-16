// https://leetcode.com/problems/remove-duplicate-letters/description/
// https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/solutions/308194/c-o-n-identical-to-316/
pub fn remove_duplicate_letters(s: String) -> String {
    use std::collections::*;
    let mut counter = HashMap::new();
    let s = s.chars().collect::<Vec<_>>();
    for i in 0..s.len() {
        *counter.entry(s[i]).or_insert(0) += 1;
    }

    let mut selected = HashSet::new();
    let mut stack = vec![];

    for ch in s {
        *counter.get_mut(&ch).unwrap() -= 1;
        if !selected.contains(&ch) {
            while !stack.is_empty()
                && counter[stack.last().unwrap()] > 0
                && *stack.last().unwrap() as i32 > ch as i32
            {
                selected.remove(&stack.pop().unwrap());
            }
            stack.push(ch);
            selected.insert(ch);
        }
    }
    stack.into_iter().collect()
}

// https://leetcode.com/problems/remove-k-digits/description/
// https://leetcode.com/problems/remove-k-digits/solutions/88668/short-python-one-o-n-and-one-regex/
pub fn remove_kdigits(num: String, mut k: i32) -> String {
    let num = num.chars().collect::<Vec<_>>();
    let mut stack = vec![];
    for n in num {
        while !stack.is_empty() && (k > 0 && *stack.last().unwrap() as i32 > n as i32) {
            stack.pop();
            k -= 1;
        }
        stack.push(n);
    }
    while k > 0 {
        stack.pop();
        k -= 1;
    }
    let stack = stack
        .into_iter()
        .skip_while(|x| *x == '0')
        .collect::<Vec<_>>();
    if stack.is_empty() {
        "0".to_string()
    } else {
        stack.into_iter().collect()
    }
}

// https://leetcode.com/problems/monotone-increasing-digits/description/
// https://leetcode.com/problems/monotone-increasing-digits/solutions/109792/easy-java/
// Idea:
// Find the first digit which is smaller than previous, turn the digit and the ones afterwards into 0, then reduce 1.
// Note that if the "previous" digit is repeated,
// only the first of those repeated digit should be kept and others should also be turned into 0.
// Use string to simplify coding.
pub fn monotone_increasing_digits(n: i32) -> i32 {
    let mut n = n.to_string().chars().collect::<Vec<_>>();
    let len = n.len();
    let mut start = len;
    for i in (1..len).rev() {
        if n[i] < n[i - 1] {
            n[i - 1] = (n[i - 1] as u8 - 1) as char;
            start = i;
        }
    }
    n[start..len].iter_mut().for_each(|x| *x = '9');
    n.into_iter().collect::<String>().parse::<i32>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test302() {
        println!("{}", remove_duplicate_letters("bcabc".to_string())); // abc
        println!("{}", remove_duplicate_letters("cbacdcbc".to_string())); // acdb
    }

    #[test]
    fn test303() {
        println!("{}", remove_kdigits("112".to_string(), 1)); // 11
        println!("{}", remove_kdigits("1234567890".to_string(), 9)); // 0
        println!("{}", remove_kdigits("1432219".to_string(), 3)); // 1219
        println!("{}", remove_kdigits("10200".to_string(), 1)); // 200
        println!("{}", remove_kdigits("10".to_string(), 2)); // 0
        println!("{}", remove_kdigits("9".to_string(), 1)); // 0
        println!("{}", remove_kdigits("100".to_string(), 1)); // 0
    }

    #[test]
    fn test304() {
        println!("{}", monotone_increasing_digits(1234)); // 1234
        println!("{}", monotone_increasing_digits(10)); // 9
        println!("{}", monotone_increasing_digits(332)); // 299
    }
}
