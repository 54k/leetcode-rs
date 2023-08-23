// https://leetcode.com/problems/reorganize-string/description/
pub fn reorganize_string_i(s: String) -> String {
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for ch in s.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    let mut heap = BinaryHeap::new();
    for (ch, f) in freq {
        heap.push((f, ch));
    }
    let mut result = Vec::new();
    while heap.len() > 1 {
        let first = heap.pop().unwrap();
        let second = heap.pop().unwrap();
        result.push(first.1);
        result.push(second.1);
        if first.0 - 1 > 0 {
            heap.push((first.0 - 1, first.1));
        }
        if second.0 - 1 > 0 {
            heap.push((second.0 - 1, second.1));
        }
    }
    while let Some((f, ch)) = heap.pop() {
        result.push(ch);
        if f - 1 > 0 {
            heap.push((f - 1, ch));
        }
    }
    for i in 1..result.len() {
        if result[i - 1] == result[i] {
            return String::new();
        }
    }
    result.into_iter().collect()
}

pub fn reorganize_string_ii(s: String) -> String {
    use std::collections::BinaryHeap;
    let mut char_counts = vec![0; 26];
    for ch in s.chars() {
        char_counts[ch as usize - 'a' as usize] += 1;
    }

    let mut pq = BinaryHeap::new();
    for i in 0..26 {
        if char_counts[i] > 0 {
            pq.push((
                char_counts[i],
                char::from_u32(i as u32 + 'a' as u32).unwrap(),
            ));
        }
    }

    let mut ans = vec![];
    while !pq.is_empty() {
        let first = pq.pop().unwrap();
        if ans.is_empty() || ans[ans.len() - 1] != first.1 {
            ans.push(first.1);
            if first.0 - 1 > 0 {
                pq.push((first.0 - 1, first.1));
            }
        } else {
            if pq.is_empty() {
                return "".to_string();
            }

            let second = pq.pop().unwrap();
            ans.push(second.1);
            if second.0 - 1 > 0 {
                pq.push((second.0 - 1, second.1));
            }
            pq.push(first);
        }
    }
    ans.into_iter().collect()
}

pub fn reorganize_string_iii(s: String) -> String {
    let mut freq = vec![0; 26];
    for ch in s.chars() {
        freq[ch as usize - 'a' as usize] += 1;
    }
    let mut max_count = 0;
    let mut letter = 0;
    for i in 0..freq.len() {
        if freq[i] > max_count {
            max_count = freq[i];
            letter = i;
        }
    }

    if max_count > (s.len() + 1) / 2 {
        return "".to_string();
    }

    let mut ans = vec!['+'; s.len()];
    let mut index = 0;

    while freq[letter] > 0 {
        ans[index] = char::from_u32(letter as u32 + 'a' as u32).unwrap();
        index += 2;
        freq[letter] -= 1;
    }
    println!("{} {}", freq[letter], index);
    for i in 0..freq.len() {
        while freq[i] > 0 {
            if index >= s.len() {
                index = 1;
            }
            ans[index] = char::from_u32(letter as u32 + 'a' as u32).unwrap();
            index += 2;
            freq[i] -= 1;
        }
    }

    ans.into_iter().collect()
}
