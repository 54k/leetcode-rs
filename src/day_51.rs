#[allow(dead_code)]
pub fn detect_capital_use(word: String) -> bool {
    let n = word.len() as i32 - 1;
    let mut last_cap = -1;
    for (i, x) in word.chars().enumerate() {
        if !x.is_lowercase() {
            if i as i32 - last_cap > 1 {
                return false;
            } else {
                last_cap = i as i32;
            }
        }
    }
    last_cap <= 0 || last_cap == n
}

#[allow(dead_code)]
pub fn capitalize_title(title: String) -> String {
    let mut ans: Vec<String> = vec![];
    for w in title.split(' ') {
        let mut w = w.to_lowercase();
        if w.len() > 2 {
            w = w
                .chars()
                .enumerate()
                .map(|(i, ch)| {
                    if i == 0 {
                        ch.to_uppercase().last().unwrap()
                    } else {
                        ch
                    }
                })
                .collect();
        }
        ans.push(w);
    }
    ans.join(" ")
}

#[allow(dead_code)]
pub fn to_lower_case(s: String) -> String {
    s.chars()
        .map(|ch| {
            if ch.is_uppercase() {
                ch.to_lowercase().last().unwrap()
            } else {
                ch
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test138() {
        println!("{}", detect_capital_use("USA".to_string()));
        println!("{}", detect_capital_use("FlaG".to_string()));
        println!("{}", detect_capital_use("Google".to_string()));
        println!("{}", detect_capital_use("okword".to_string()));
        println!(
            "{}",
            detect_capital_use("FFFFFFFFFFFFFFFFFFFFf".to_string())
        );
    }
    #[test]
    fn test139() {
        println!("{}", capitalize_title("capiTalIze tHe titLe".to_string()));
        println!(
            "{}",
            capitalize_title("First leTTeR of EACH Word".to_string())
        );
        println!("{}", capitalize_title("i lOve leetcode".to_string()));
    }
}
