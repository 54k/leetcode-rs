// https://leetcode.com/problems/number-of-ways-to-split-a-string/description/
pub fn num_ways(s: String) -> i32 {
    // Just in case anyone is still finding it difficult to understand the algorithm for the general case..

    // Consider the example :-
    // s = "100100010100110"

    // We see that there are 6 ones in the string.
    // For a valid way to split,each part must have 2 ones.

    // If we traverse the string and track the number of ones encountered so far at each index,we see:
    // ones = [1,1,1,2,2,2,2,3,3,4,4,4,5,6,6]

    // Now,the 1st of the three parts may end at any of the indices from 3-6 and it will still be a valid split..
    // So,we have 4 ways to choose for the end index of the 1st cut

    // Similarly,for the 2nd part,it may end at any index from 9-11 and still be a valid cut i.e. 3 ways

    // Multiplying these,we get 4*3 = 12 ways..
    // Hope that helps clear it up for anybody who still couldn't understand the algorithm.
    let n = s.len() as i64;
    const MOD: i64 = 1000000007;
    let mut prefix = vec![0; s.len() + 1];
    let mut total_ones = 0;
    for (i, ch) in s.chars().enumerate() {
        if ch == '1' {
            prefix[i + 1] += prefix[i] + 1;
            total_ones += 1;
        } else {
            prefix[i + 1] += prefix[i];
        }
    }
    if total_ones % 3 > 0 {
        return 0;
    }
    if total_ones == 0 {
        return ((((n - 2) * (n - 1)) / 2) % MOD) as i32;
    }
    println!("{:?}", prefix);

    let (mut c1, mut c2) = (0, 0);
    for i in 0..prefix.len() {
        if prefix[i] == total_ones / 3 {
            c1 += 1;
        } else if prefix[i] == total_ones / 3 * 2 {
            c2 += 1;
        }
    }
    (c1 * c2 % MOD) as i32
}

// https://leetcode.com/problems/unique-word-abbreviation/
use std::collections::{HashMap, HashSet};
fn abbr(s: String) -> String {
    if s.len() <= 2 {
        s
    } else {
        let mut cnt = 0;
        let mut res = String::new();
        for (i, ch) in s.chars().enumerate() {
            if i == 0 || i == s.len() - 1 {
                if i == s.len() - 1 {
                    res.push_str(&format!("{}", cnt));
                }
                res.push(ch);
            } else {
                cnt += 1;
            }
        }
        res
    }
}
struct ValidWordAbbr {
    m: HashMap<String, HashSet<String>>,
}
impl ValidWordAbbr {
    fn new(dictionary: Vec<String>) -> Self {
        let mut m = HashMap::new();
        for word in dictionary {
            m.entry(abbr(word.clone()))
                .or_insert(HashSet::new())
                .insert(word.clone());
        }
        Self { m }
    }
    fn is_unique(&self, word: String) -> bool {
        if self.m.contains_key(&abbr(word.clone())) {
            let words = self.m.get(&abbr(word.clone())).unwrap();
            return words.len() == 1 && words.contains(&word);
        }
        return true;
    }
}

// https://leetcode.com/problems/game-of-life/description/
pub fn game_of_life(board: &mut Vec<Vec<i32>>) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test492() {
        println!("{}", num_ways("100100010100110".to_string())); // 12
        println!("{}", num_ways("10101".to_string())); // 4

        println!("{}", num_ways("0000".to_string())); // 3
        println!("{}", num_ways("00000000".to_string())); // 12
    }
    #[test]
    fn test493() {
        let vwa = ValidWordAbbr::new(vec![
            "deer".to_string(),
            "door".to_string(),
            "cake".to_string(),
            "card".to_string(),
        ]);
        println!("{}", vwa.is_unique("dear".to_string()));
        println!("{}", vwa.is_unique("door".to_string()));
        println!("{}", vwa.is_unique("cart".to_string()));
        println!("{}", vwa.is_unique("cake".to_string()));
    }
}
