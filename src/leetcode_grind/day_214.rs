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

// https://leetcode.com/problems/single-number-ii/description/
pub fn single_number(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    for i in (0..nums.len() - 1).step_by(3) {
        if nums[i] == nums[i + 1] {
            continue;
        } else {
            return nums[i];
        }
    }
    return nums[nums.len() - 1];
}

// https://leetcode.com/problems/longest-increasing-subsequence/description/
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut sub = vec![nums[0]];
    let mut len = sub.len();
    for &num in nums.iter().skip(1) {
        if sub[len - 1] < num {
            sub.push(num);
            len += 1;
            continue;
        }
        for i in 0..len {
            if sub[i] >= num {
                sub[i] = num;
                break;
            }
        }
    }
    len as i32
}

// https://leetcode.com/problems/russian-doll-envelopes/description/
pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
    fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut sub = vec![nums[0]];
        let mut len = sub.len();
        for &num in nums.iter().skip(1) {
            if sub[len - 1] < num {
                sub.push(num);
                len += 1;
                continue;
            }

            let (mut lo, mut hi) = (0, len as i32 - 1);
            while lo <= hi {
                let mid = (lo + hi) / 2;
                if sub[mid as usize] < num {
                    lo = mid + 1;
                } else if sub[mid as usize] >= num {
                    hi = mid - 1;
                }
            }
            sub[lo as usize] = num;
        }
        len as i32
    }

    envelopes.sort_by(|a, b| {
        if a[0] == b[0] {
            b[1].cmp(&a[1])
        } else {
            a[0].cmp(&b[0])
        }
    });
    let heights = envelopes.into_iter().map(|e| e[1]).collect();
    length_of_lis(heights)
}
