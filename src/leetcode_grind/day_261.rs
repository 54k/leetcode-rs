// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/description/
pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    use std::collections::HashMap;

    fn compute_cost(
        s1: &Vec<char>,
        s2: &Vec<char>,
        i: i32,
        j: i32,
        memo: &mut HashMap<(i32, i32), i32>,
    ) -> i32 {
        let key = (i, j);

        if i < 0 && j < 0 {
            return 0;
        }

        if memo.contains_key(&key) {
            return memo[&key];
        }

        let res = if i < 0 {
            let mut delete_cost = 0;
            for pointer in 0..=j {
                delete_cost += s2[pointer as usize] as i32;
            }
            delete_cost
        } else if j < 0 {
            let mut delete_cost = 0;
            for pointer in 0..=i {
                delete_cost += s1[pointer as usize] as i32;
            }
            delete_cost
        } else if s1[i as usize] == s2[j as usize] {
            compute_cost(s1, s2, i - 1, j - 1, memo)
        } else {
            (s1[i as usize] as i32 + compute_cost(s1, s2, i - 1, j, memo)).min(
                (s2[j as usize] as i32 + compute_cost(s1, s2, i, j - 1, memo)).min(
                    s1[i as usize] as i32
                        + s2[j as usize] as i32
                        + compute_cost(s1, s2, i - 1, j - 1, memo),
                ),
            )
        };

        memo.insert(key, res);
        memo[&key]
    }

    let mut memo = HashMap::new();
    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();

    compute_cost(
        &s1,
        &s2,
        s1.len() as i32 - 1,
        s2.len() as i32 - 1,
        &mut memo,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        minimum_delete_sum("eat".to_string(), "sat".to_string());
    }
}

// https://leetcode.com/problems/length-of-the-longest-valid-substring/
pub fn longest_valid_substring(word: String, forbidden: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let mut forbidden = forbidden.into_iter().collect::<HashSet<_>>();

    let word = word.chars().collect::<Vec<_>>();
    let n = word.len();

    let mut ans = String::new();
    let mut fans = 0;

    for i in 0..n {
        ans.push(word[i]);
        let m = ans.len();
        let mut idx = -1;

        for j in (0.max(m as i32 - 10) as usize)..m {
            let mut curr = &ans[j..];
            if forbidden.contains(curr) {
                idx = j as i32;
            }
        }

        if idx != -1 {
            ans = ans[idx as usize + 1..].to_string();
        }
        fans = fans.max(ans.len() as i32);
    }
    fans
}

// https://leetcode.com/problems/minimum-knight-moves/
pub fn min_knight_moves(x: i32, y: i32) -> i32 {
    use std::collections::{HashSet, VecDeque};

    const DIRS: [(i32, i32); 8] = [
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
        (-1, -2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
    ];

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    let mut seen = HashSet::new();
    seen.insert((0, 0));

    while let Some((r, c, moves)) = queue.pop_front() {
        if r == x && c == y {
            return moves;
        }

        for d in &DIRS {
            let (nx, ny) = (r + d.0, c + d.1);
            if !seen.contains(&(nx, ny)) {
                seen.insert((nx, ny));
                queue.push_back((nx, ny, moves + 1));
            }
        }
    }

    panic!("oops")
}

// https://leetcode.com/problems/split-strings-by-separator/
pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
    let mut ans = vec![];
    for w in words {
        let split = w
            .split(separator)
            .into_iter()
            .filter(|x| x != &"")
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();
        println!("{:?}", split);
        ans.extend(split);
    }
    ans
}

// https://leetcode.com/problems/largest-element-in-an-array-after-merge-operations/
pub fn max_array_value(nums: Vec<i32>) -> i64 {
    let mut nums = nums.iter().map(|x| *x as i64).collect::<Vec<_>>();
    let mut ans = nums[0];
    for i in (1..nums.len()).rev() {
        if nums[i] >= nums[i - 1] {
            nums[i - 1] += nums[i];
            ans = ans.max(nums[i - 1]);
        }
    }
    ans
}

// https://leetcode.com/problems/available-captures-for-rook/
pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    let (m, n) = (board.len(), board[0].len());
    let mut ans = 0;
    for r in 0..m {
        for c in 0..n {
            if board[r][c] == 'R' {
                if r < m - 1 {
                    let mut k = r + 1;
                    while k < m {
                        if board[k][c] == 'p' {
                            ans += 1;
                            break;
                        }
                        if board[k][c] == 'B' {
                            break;
                        }
                        k += 1;
                    }
                }

                if 0 < r {
                    let mut k = r as i32 - 1;
                    while k >= 0 {
                        if board[k as usize][c] == 'p' {
                            ans += 1;
                            break;
                        }
                        if board[k as usize][c] == 'B' {
                            break;
                        }
                        k -= 1;
                    }
                }

                if c < n - 1 {
                    let mut k = c + 1;
                    while k < n {
                        if board[r][k as usize] == 'p' {
                            ans += 1;
                            break;
                        }
                        if board[r][k as usize] == 'B' {
                            break;
                        }
                        k += 1;
                    }
                }

                if 0 < c {
                    let mut k = c as i32 - 1;
                    while k >= 0 {
                        if board[r][k as usize] == 'p' {
                            ans += 1;
                            break;
                        }
                        if board[r][k as usize] == 'B' {
                            break;
                        }
                        k -= 1;
                    }
                }

                break;
            }
        }
    }
    ans
}
