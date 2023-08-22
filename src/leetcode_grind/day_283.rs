// https://leetcode.com/problems/excel-sheet-column-title/description/
pub fn convert_to_title(mut column_number: i32) -> String {
    let mut ans = "".to_string();
    while column_number > 0 {
        column_number -= 1;
        ans.push(char::from_u32((column_number % 26) as u32 + 'A' as u32).unwrap());
        column_number /= 26;
    }
    ans.chars().into_iter().rev().collect()
}

// https://leetcode.com/problems/excel-sheet-column-number/description/
pub fn title_to_number(column_title: String) -> i32 {
    use std::collections::HashMap;
    let mut alpha_map = HashMap::new();
    for i in 0..26 {
        let ch = i + 65;
        alpha_map.insert(char::from_u32(ch as u32).unwrap(), i + 1);
    }
    let mut result = 0;
    for (i, ch) in column_title.chars().rev().enumerate() {
        result += alpha_map[&ch] * 26i32.pow(i as u32);
    }
    result
}

pub fn title_to_number2(column_title: String) -> i32 {
    let mut result = 0;
    let column_title = column_title.chars().collect::<Vec<_>>();
    let n = column_title.len();
    for i in 0..n {
        result = result * 26;
        result += (column_title[i] as i32 - 'A' as i32 + 1);
    }
    result
}

// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together/description
pub fn min_swaps(data: Vec<i32>) -> i32 {
    let ones = data.iter().fold(0, |acc, v| acc + *v) as usize;
    if ones == 0 {
        return 0;
    }
    let mut curr_ones = 0;
    let mut left = 0;
    let mut ans = i32::MAX;
    for right in 0..data.len() {
        curr_ones += data[right];
        if right - left + 1 == ones {
            ans = ans.min(ones as i32 - curr_ones);
            curr_ones -= data[left];
            left += 1;
        }
    }
    ans
}

pub fn min_swaps_deque(data: Vec<i32>) -> i32 {
    use std::collections::VecDeque;
    let ones = data.iter().copied().sum::<i32>();
    let mut max_one = 0;
    let mut cnt_one = 0;
    let mut deque = VecDeque::new();
    for i in 0..data.len() {
        deque.push_back(data[i]);
        cnt_one += data[i];
        if (deque.len() as i32) > ones {
            cnt_one -= deque.pop_front().unwrap();
        }
        max_one = max_one.max(cnt_one);
    }
    ones - max_one
}

// https://leetcode.com/problems/longest-repeating-character-replacement/description/
pub fn character_replacement_i(s: String, k: i32) -> i32 {
    fn is_valid(s: &Vec<char>, size: i32, k: i32) -> bool {
        let mut freq_map = vec![0; 26];
        let mut max_freq = 0;
        let mut start = 0;

        for end in 0..s.len() {
            freq_map[s[end] as usize - 'A' as usize] += 1;

            if end - start + 1 > size as usize {
                freq_map[s[start] as usize - 'A' as usize] -= 1;
                start += 1;
            }

            max_freq = max_freq.max(freq_map[s[end] as usize - 'A' as usize]);
            if size - max_freq <= k {
                return true;
            }
        }
        false
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut lo = 0;
    let mut hi = s.len() as i32 + 1;

    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if is_valid(&s, mid, k) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

pub fn character_replacement_ii(s: String, k: i32) -> i32 {
    use std::collections::HashSet;
    let s = s.chars().collect::<Vec<_>>();
    let mut all_letters = HashSet::new();
    for i in 0..s.len() {
        all_letters.insert(s[i]);
    }
    let mut max_len = 0;
    for &letter in all_letters.iter() {
        let mut start = 0;
        let mut count = 0;

        for end in 0..s.len() {
            if s[end] == letter {
                count += 1;
            }

            while end - start + 1 - count > k as usize {
                if s[start] == letter {
                    count -= 1;
                }
                start += 1;
            }
            max_len = max_len.max(end - start + 1);
        }
    }
    max_len as i32
}

pub fn character_replacement_iii(s: String, k: i32) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let mut start = 0;
    let mut freq_map = vec![0; 26];
    let mut max_freq = 0;
    let mut longest_substr_len = 0;

    for end in 0..s.len() {
        let curr_char = s[end] as usize - 'A' as usize;
        freq_map[curr_char] += 1;
        max_freq = max_freq.max(freq_map[curr_char]);

        if end - start + 1 - max_freq > k as usize {
            let outgoing_char = s[start] as usize - 'A' as usize;
            freq_map[outgoing_char] -= 1;
            start += 1;
        }

        longest_substr_len = longest_substr_len.max(end - start + 1);
    }
    longest_substr_len as i32
}

// https://leetcode.com/problems/maximum-beauty-of-an-array-after-applying-operation/description/
pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let mut start = 0;
    let mut ans = 0;
    for end in 0..nums.len() {
        while nums[end] - nums[start] > 2 * k {
            start += 1;
        }
        ans = ans.max(end - start + 1);
    }
    ans as i32
}

// https://leetcode.com/problems/find-the-longest-semi-repetitive-substring/description/
pub fn longest_semi_repetitive_substring(s: String) -> i32 {
    use std::collections::VecDeque;
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = 0;

    let mut current = VecDeque::new();
    let mut k = 1;

    for end in 0..s.len() {
        let incoming = s[end];
        if current.len() >= 1 && current[current.len() - 1] == incoming {
            k -= 1;
        }
        current.push_back(incoming);

        while k < 0 {
            if current.len() >= 2 && current[0] == current[1] {
                k += 1;
            }
            current.pop_front();
        }

        ans = ans.max(current.len());
    }

    ans as i32
}
