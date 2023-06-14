pub fn to_hex(num: i32) -> String {
    if num == 0 {
        return "0".to_string();
    }
    use std::collections::HashMap;
    let table = vec![
        (10, 'a'),
        (11, 'b'),
        (12, 'c'),
        (13, 'd'),
        (14, 'e'),
        (15, 'f'),
    ]
    .into_iter()
    .collect::<HashMap<i32, char>>();
    let mut ans = String::new();
    for i in 0..8 {
        let b = num >> (i * 4) & 0xf;
        let ch = if (0..=9).contains(&b) {
            char::from_digit(b as u32, 10).unwrap()
        } else {
            table[&b]
        };
        ans.push(ch);
    }
    ans.chars().rev().skip_while(|ch| *ch == '0').collect()
}

// https://leetcode.com/problems/reverse-bits/description/
pub fn reverse_bits(mut n: u32) -> u32 {
    n = (n >> 16) | (n << 16);
    n = ((n & 0xff00ff00) >> 8) | ((n & 0x00ff00ff) << 8);
    n = ((n & 0xf0f0f0f0) >> 4) | ((n & 0x0f0f0f0f) << 4);
    n = ((n & 0xcccccccc) >> 2) | ((n & 0x33333333) << 2);
    n = ((n & 0xaaaaaaaa) >> 1) | ((n & 0x55555555) << 1);
    n
}

// https://leetcode.com/problems/encode-and-decode-strings/description/
struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut encoded = String::new();
        for s in strs {
            let chunk_sz = s.len() as i32;
            encoded.push(unsafe { std::mem::transmute(chunk_sz) });
            encoded.push_str(s.as_str());
        }
        encoded
    }

    fn decode(&self, s: String) -> Vec<String> {
        let mut decoded = vec![];
        let mut s = s.chars().collect::<Vec<_>>();
        let mut n = 0;
        while n < s.len() {
            let mut chunk_sz: i32 = unsafe { std::mem::transmute(s[n]) };
            n += 1;
            decoded.push(s[n..n + chunk_sz as usize].iter().copied().collect());
            n += chunk_sz as usize;
        }
        decoded
    }
}

// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/description/
pub fn min_moves2(nums: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/minimum-falling-path-sum-ii/
pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-v/description/
pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-vi/description/
pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-vii/description/
pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/description/
pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/range-sum-query-2d-mutable/description/
mod rmq2d {
    struct NumMatrix {}

    impl NumMatrix {
        fn new(matrix: Vec<Vec<i32>>) -> Self {
            todo!()
        }

        fn update(&self, row: i32, col: i32, val: i32) {
            todo!()
        }

        fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            todo!()
        }
    }
}

// https://leetcode.com/problems/maximize-grid-happiness/description/
pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
    todo!()
}
