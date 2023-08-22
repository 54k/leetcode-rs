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
pub fn character_replacement_bin_search(s: String, k: i32) -> i32 {
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
