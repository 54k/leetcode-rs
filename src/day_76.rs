// https://leetcode.com/problems/word-break/solutions/127450/word-break/?orderBy=most_relevant
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    use std::collections::*;
    let mut set = HashSet::new();
    set.extend(word_dict.into_iter());
    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;
    for i in 1..=s.len() {
        for j in 0..i {
            if dp[j] && set.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[s.len()]
}

// https://leetcode.com/problems/concatenated-words/solutions/2822170/concatenated-words/?orderBy=most_relevant
pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
    use std::collections::*;
    let mut set = HashSet::new();
    set.extend(words.clone().into_iter());
    let mut ans = vec![];
    for s in words {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        for i in 1..=s.len() {
            for j in if i == s.len() { 1 } else { 0 }..i {
                if dp[i] {
                    break;
                }
                dp[i] = dp[j] && set.contains(&s[j..i]);
            }
        }
        if dp[s.len()] {
            ans.push(s);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test178() {
        println!(
            "{}",
            word_break(
                "leetcode".to_string(),
                vec!["leet".to_string(), "code".to_string()]
            )
        ); // true
        println!(
            "{}",
            word_break(
                "applepenapple".to_string(),
                vec!["apple".to_string(), "pen".to_string()]
            )
        ); // true
    }

    #[test]
    fn test179() {
        println!(
            "{:?}",
            find_all_concatenated_words_in_a_dict(vec![
                "cat".to_string(),
                "cats".to_string(),
                "catsdogcats".to_string(),
                "dog".to_string(),
                "dogcatsdog".to_string(),
                "hippopotamuses".to_string(),
                "rat".to_string(),
                "ratcatdogcat".to_string(),
            ])
        ); // ["catsdogcats","dogcatsdog","ratcatdogcat"]
    }
}
