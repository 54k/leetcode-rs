// https://leetcode.com/problems/binary-trees-with-factors/description
pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    const MOD: i64 = 1_000_000_007;
    let n = arr.len();
    let mut arr = arr;
    arr.sort();
    let mut dp = vec![1; n];

    let mut index = HashMap::new();
    for i in 0..n {
        index.insert(arr[i], i);
    }

    for i in 0..n {
        for j in 0..i {
            if arr[i] % arr[j] == 0 {
                // j is the left child
                let right = arr[i] / arr[j];
                if index.contains_key(&right) {
                    dp[i] = dp[i] + (dp[j] * dp[*index.get(&right).unwrap()]) % MOD;
                }
            }
        }
    }

    let mut ans = 0;
    for x in dp {
        ans += x;
    }

    (ans % MOD) as i32
}

// https://leetcode.com/problems/read-n-characters-given-read4/description/
struct Solution1;

impl Solution1 {
    fn read4(&self, buf4: &mut [char]) -> i32 {
        return 0;
    }

    pub fn read(&self, buf: &mut [char], n: i32) -> i32 {
        let mut copied_chars = 0;
        let mut read_chars = 4;

        let mut buf4 = vec!['\0'; 4];

        while copied_chars < n && read_chars == 4 {
            read_chars = self.read4(&mut buf4);
            for i in 0..read_chars as usize {
                if copied_chars == n {
                    return copied_chars;
                }

                buf[copied_chars] = buf4[i];
                copied_chars += 1;
            }
        }

        copied_chars
    }
}

struct Solution2;

impl Solution2 {
    fn read4(&self, buf4: &mut [char]) -> i32 {
        return 0;
    }

    pub fn read(&self, buf: &mut [char], n: i32) -> i32 {
        let mut copied_chars = 0;
        let mut read_chars = 4;
        let mut remaining_chars = n;

        while remaining_chars >= 4 && read_chars == 4 {
            read_chars = self.read4(&mut buf[copied_chars as usize..]);
            copied_chars += read_chars;
        }

        if remaining_chars > 0 && read_chars > 0 {
            let mut buf4 = vec!['\0'; 4];
            read_chars = self.read4(&mut buf4);
            for i in 0..remaining_chars.min(read_chars) {
                buf[copied_chars as usize] = buf4[i as usize];
                copied_chars += 1;
            }
        }

        n.min(copied_chars)
    }
}

// https://leetcode.com/problems/missing-ranges/description/
pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut missing_ranges = vec![];

    if n == 0 {
        missing_ranges.push(vec![lower, upper]);
        return missing_ranges;
    }

    if lower < nums[0] {
        missing_ranges.push(vec![lower, nums[0] - 1]);
    }

    for i in 0..n - 1 {
        if nums[i + 1] - nums[i] <= 1 {
            continue;
        }
        missing_ranges.push(vec![nums[i] + 1, nums[i + 1] - 1]);
    }

    if upper > nums[n - 1] {
        missing_ranges.push(vec![nums[n - 1] + 1, upper]);
    }

    missing_ranges
}

// https://leetcode.com/problems/shortest-word-distance/
pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
    let mut i1 = -1;
    let mut i2 = -1;
    let mut min_dist = words_dict.len() as i32;
    for i in 0..words_dict.len() {
        if words_dict[i] == word1 {
            i1 = i as i32;
        } else if words_dict[i] == word2 {
            i2 = i as i32;
        }

        if i1 != -1 && i2 != -1 {
            min_dist = min_dist.min((i1 - i2).abs());
        }
    }
    min_dist
}
