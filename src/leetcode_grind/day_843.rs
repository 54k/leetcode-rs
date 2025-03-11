// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/description/?envType=daily-question&envId=2025-03-11
pub fn number_of_substrings(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let len = s.len();        
    let mut last_pos = vec![-1,-1,-1];
    let mut total = 0;
    for pos in 0..len {
        last_pos[s[pos] as usize - 'a' as usize] = pos as i32;
        total += 1 + last_pos[0].min(last_pos[1]).min(last_pos[2]);
    }
    total as i32
}