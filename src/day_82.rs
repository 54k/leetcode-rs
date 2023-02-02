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
}
