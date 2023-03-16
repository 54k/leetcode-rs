pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    fn is_alien_sorted1(words: Vec<String>, order: String) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, ch) in order.chars().enumerate() {
            map.insert(ch, i);
        }

        let seq = words
            .iter()
            .map(|w| w.chars().collect::<Vec<_>>())
            .map(|c| c.iter().map(|ch| *map.get(ch).unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for i in 1..seq.len() {
            if seq[i - 1] > seq[i] {
                return false;
            }
        }
        true
    }

    fn is_alien_sorted2(words: Vec<String>, order: String) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, ch) in order.chars().enumerate() {
            map.insert(ch, i);
        }
        for i in 1..words.len() {
            let f = words[i - 1].chars().collect::<Vec<_>>();
            for j in 0..f.len() {
                let s = words[i].chars().collect::<Vec<_>>();
                if j >= s.len() {
                    return false;
                }
                if f[j] != s[j] {
                    if *map.get(&f[j]).unwrap() > *map.get(&s[j]).unwrap() {
                        return false;
                    } else {
                        break;
                    }
                }
            }
        }
        true
    }

    is_alien_sorted2(words, order)
}

fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    use std::cmp::*;
    use std::collections::*;
    let mut map = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut ans = vec![];
    for w in words {
        *map.entry(w).or_insert(0) += 1;
    }
    for (key, val) in map {
        heap.push(Reverse((val, Reverse(key))));
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    while let Some(Reverse((_, Reverse(s)))) = heap.pop() {
        ans.push(s);
    }
    ans.reverse();
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test189() {
        println!(
            "{}",
            is_alien_sorted(
                vec!["hello".to_string(), "leetcode".to_string()],
                "hlabcdefgijkmnopqrstuvwxyz".to_string()
            )
        ); // true

        println!(
            "{}",
            is_alien_sorted(
                vec!["word".to_string(), "world".to_string(), "row".to_string()],
                "worldabcefghijkmnpqstuvxyz".to_string()
            )
        ); // false
    }

    #[test]
    fn test190() {
        println!(
            "{:?}",
            top_k_frequent(
                vec![
                    "the".to_string(),
                    "day".to_string(),
                    "is".to_string(),
                    "sunny".to_string(),
                    "the".to_string(),
                    "the".to_string(),
                    "the".to_string(),
                    "sunny".to_string(),
                    "is".to_string(),
                    "is".to_string()
                ],
                4,
            )
        ); // ["the","is","sunny","day"]

        println!(
            "{:?}",
            top_k_frequent(
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "coding".to_string()
                ],
                2,
            )
        ); // ["i","love"]

        println!(
            "{:?}",
            top_k_frequent(
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "coding".to_string()
                ],
                1,
            )
        ); // ["i"]
    }
}
