pub fn are_sentences_similar(s1: String, s2: String) -> bool {
    let s1_words = s1.split(" ").collect::<Vec<_>>();
    let s2_words = s2.split(" ").collect::<Vec<_>>();

    let mut start = 0;

    let mut ends1 = s1_words.len() as i32 - 1;
    let mut ends2 = s2_words.len() as i32 - 1;

    let mut s1_words_length = s1_words.len();
    let mut s2_words_length = s2_words.len();

    if s1_words_length > s2_words_length {
        return are_sentences_similar(s2, s1);
    }

    while start < s1_words_length && s1_words[start] == s2_words[start] {
        start += 1;
    }

    while ends1 >= 0 && s1_words[ends1 as usize] == s2_words[ends2 as usize] {
        ends1 -= 1;
        ends2 -= 1;
    }

    ends1 < start as i32
}
