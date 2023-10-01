// https://leetcode.com/problems/reverse-words-in-a-string-iii/
pub fn reverse_words_traverse_reverse(s: String) -> String {
    let mut ans = vec![];
    let s = s.chars().collect::<Vec<_>>();

    let mut last_space_index = -1;
    for str_index in 0..s.len() {
        if str_index == s.len() - 1 || s[str_index] == ' ' {
            let mut reverse_str_index = if str_index == s.len() - 1 {
                str_index as i32
            } else {
                str_index as i32 - 1
            };

            while reverse_str_index > last_space_index {
                ans.push(s[reverse_str_index as usize]);
                reverse_str_index -= 1;
            }

            if str_index != s.len() - 1 {
                ans.push(' ');
            }

            last_space_index = str_index as i32;
        }
    }
    ans.into_iter().collect::<String>()
}

pub fn reverse_words_two_pointers(s: String) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    let mut word_start = 0;
    for i in 0..s.len() {
        if i == s.len() - 1 || s[i] == ' ' {
            let mut word_end = if i == s.len() - 1 { s.len() - 1 } else { i - 1 };
            while word_start < word_end {
                let tmp = s[word_start];
                s[word_start] = s[word_end];
                s[word_end] = tmp;
                word_start += 1;
                word_end -= 1;
            }
            word_start = i + 1;
        }
    }
    s.into_iter().collect()
}
