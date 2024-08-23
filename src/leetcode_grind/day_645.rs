// https://leetcode.com/problems/generalized-abbreviation/description/?envType=weekly-question&envId=2024-08-22
pub fn generate_abbreviations(word: String) -> Vec<String> {
    let n = word.len();
    let word = word.chars().collect::<Vec<_>>();
    let mut res = vec![];

    for mask in 0..(1 << n) {
        let mut curr_word = vec![];
        let mut abbreviated_count = 0;

        for index in 0..n {
            if mask & (1 << index) != 0 {
                abbreviated_count += 1;
            } else {
                if abbreviated_count > 0 {
                    for ch in format!("{}", abbreviated_count).chars() {
                        curr_word.push(ch);
                    }
                    abbreviated_count = 0;
                }
                curr_word.push(word[index]);
            }
        }

        if abbreviated_count > 0 {
            for ch in format!("{}", abbreviated_count).chars() {
                curr_word.push(ch);
            }
        }
        res.push(curr_word.into_iter().collect::<String>());
    }

    res
}
