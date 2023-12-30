// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/
pub fn make_equal(words: Vec<String>) -> bool {
    let n = words.len() as i32;
    if n == 1 {
        return true;
    }
    let mut freq = vec![0; 26];
    for w in words {
        for ch in w.chars() {
            freq[ch as usize - 'a' as usize] += 1;
        }
    }

    // println!("{} {:?}", n, freq);

    for f in freq {
        if f > 0 && f % n != 0 {
            return false;
        }
    }
    true
}
