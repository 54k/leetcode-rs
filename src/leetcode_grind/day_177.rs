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
        // This code calculates the number of unique pairs that can be formed from a set of n elements,
        // where n is an integer greater than or equal to 2.
        // The formula used to calculate the number of pairs is (n-2) * (n-1) / 2.
        // This formula is derived from the fact that each element can be paired with n-1 other elements,
        // but we must divide by 2 to avoid counting each pair twice (i.e. (a,b) and (b,a) are the same pair).
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

// https://leetcode.com/problems/generalized-abbreviation/
pub fn generate_abbreviations(word: String) -> Vec<String> {
    fn backtrack(word: &Vec<char>, ans: &mut Vec<String>, cur: &mut String, i: usize, k: usize) {
        let n = cur.len();
        if i == word.len() {
            if k != 0 {
                cur.push_str(&format!("{}", k));
            }
            ans.push(cur.clone());
        } else {
            backtrack(word, ans, cur, i + 1, k + 1);
            if k != 0 {
                cur.push_str(&format!("{}", k));
            }
            cur.push(word[i]);
            backtrack(word, ans, cur, i + 1, 0);
        }
        while cur.len() != n {
            cur.pop();
        }
    }
    let word = word.chars().collect::<Vec<_>>();
    let mut ans = vec![];
    backtrack(&word, &mut ans, &mut String::new(), 0, 0);
    ans
}

// https://leetcode.com/problems/game-of-life/description/
pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let neighbors = vec![0, 1, -1];
    let rows = board.len();
    let cols = board[0].len();

    for row in 0..rows {
        for col in 0..cols {
            let mut live_neighbors = 0;

            for i in 0..3 {
                for j in 0..3 {
                    if !(neighbors[i] == 0 && neighbors[j] == 0) {
                        let r = row as i32 + neighbors[i];
                        let c = col as i32 + neighbors[j];

                        if r < rows as i32
                            && r >= 0
                            && c < cols as i32
                            && c >= 0
                            && board[r as usize][c as usize].abs() == 1
                        {
                            live_neighbors += 1;
                        }
                    }
                }
            }
            // rule 1 or rule 3
            if board[row][col] == 1 && (live_neighbors < 2 || live_neighbors > 3) {
                board[row][col] = -1;
            }
            // rule 4
            if board[row][col] == 0 && live_neighbors == 3 {
                board[row][col] = 2;
            }
        }
    }

    for row in 0..rows {
        for col in 0..cols {
            if board[row][col] > 0 {
                board[row][col] = 1;
            } else {
                board[row][col] = 0;
            }
        }
    }
}

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

    #[test]
    fn test494() {
        let mut v = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        game_of_life(&mut v);
        println!("{:?}", v); // [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
    }
}
