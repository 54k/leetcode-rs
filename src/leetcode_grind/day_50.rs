#[allow(dead_code)]
pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut dict = vec!["".to_string(); 26];
    let mut used = std::collections::HashSet::<String>::new();
    let pattern = pattern.chars().collect::<Vec<_>>();
    let words = s.split(' ').map(|s| s.to_string()).collect::<Vec<_>>();
    if pattern.len() != words.len() {
        return false;
    }
    for i in 0..pattern.len() {
        let alias = &mut dict[pattern[i] as usize - 'a' as usize];
        if alias.is_empty() {
            let al = words[i].clone();
            if used.contains(&al) {
                return false;
            }
            used.insert(al.clone());
            *alias = al;
        } else if *alias != words[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test137() {
        println!(
            "{}",
            word_pattern("abba".to_string(), "dog cat cat dog".to_string())
        ); // true
        println!(
            "{}",
            word_pattern("abba".to_string(), "dog cat cat fish".to_string())
        ); // false
        println!(
            "{}",
            word_pattern("abba".to_string(), "dog dog dog dog".to_string())
        ); // false
        println!(
            "{}",
            word_pattern("aaaa".to_string(), "dog cat cat dog".to_string())
        ); // false
        println!(
            "{}",
            word_pattern("aaa".to_string(), "aa aa aa aa".to_string())
        ); // false
        println!(
            "{}",
            word_pattern("jquery".to_string(), "jquery".to_string())
        ); // false
    }
}
