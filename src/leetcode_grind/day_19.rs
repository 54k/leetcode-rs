#[allow(dead_code)]
pub fn halves_are_alike(s: String) -> bool {
    fn vowel_count(ch: char) -> i32 {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => 1,
            _ => 0,
        }
    }
    let mut res = 0;
    let s = s.chars().collect::<Vec<_>>();
    for i in (0..s.len() / 2).rev() {
        let x = s[i];
        let y = s[i + s.len() / 2];
        res += -vowel_count(x) + vowel_count(y);
    }

    res == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test92() {
        println!("{}", halves_are_alike("pAteXxboOk".to_owned())); // true
    }
}
