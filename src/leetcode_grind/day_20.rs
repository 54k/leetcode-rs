#[allow(dead_code)]
pub fn close_strings(word1: String, word2: String) -> bool {
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;

    fn build_frequencies(w: String) -> HashMap<char, i32> {
        let mut ans = HashMap::new();
        for ch in w.chars() {
            if let Entry::Vacant(e) = ans.entry(ch) {
                e.insert(1);
            } else {
                *ans.get_mut(&ch).unwrap() += 1;
            }
        }
        ans
    }

    let f1 = build_frequencies(word1);
    let mut freq1 = f1.values().copied().collect::<Vec<_>>();

    let f2 = build_frequencies(word2);
    let mut freq2 = f2.values().copied().collect::<Vec<_>>();

    freq1.sort();
    freq2.sort();

    let mut ch1 = f1.keys().copied().collect::<Vec<_>>();
    let mut ch2 = f2.keys().copied().collect::<Vec<_>>();

    ch1.sort();
    ch2.sort();

    freq1 == freq2 && ch1 == ch2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test93() {
        println!("{}", close_strings("abc".to_owned(), "bca".to_owned()));
        println!(
            "{}",
            close_strings("cabbba".to_owned(), "abbccc".to_owned())
        );
        println!("{}", close_strings("a".to_owned(), "aa".to_owned()));
    }
}
