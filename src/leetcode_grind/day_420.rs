// https://leetcode.com/problems/flip-game/description/
pub fn generate_possible_next_moves(current_state: String) -> Vec<String> {
    let current_state = current_state.chars().collect::<Vec<_>>();
    let mut states = vec![];
    for i in 1..current_state.len() {
        if current_state[i - 1] == current_state[i] && current_state[i] == '+' {
            let mut left = current_state[..i - 1].iter().copied().collect::<Vec<_>>();
            let right = current_state[i + 1..].iter().copied().collect::<Vec<_>>();
            let mid = vec!['-', '-'];
            left.extend(mid);
            left.extend(right);
            states.push(left.into_iter().collect());
        }
    }
    states
}

// https://leetcode.com/problems/reverse-vowels-of-a-string/description/
pub fn reverse_vowels(s: String) -> String {
    use std::collections::HashSet;
    let mut vowels = HashSet::from(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);

    let mut s = s.chars().collect::<Vec<_>>();

    let mut i = 0i32;
    let mut j = s.len() as i32 - 1;

    while i < j {
        while i < j && !vowels.contains(&s[i as usize]) {
            i += 1;
        }
        while i < j && !vowels.contains(&s[j as usize]) {
            j -= 1;
        }

        s.swap(i as usize, j as usize);
        i += 1;
        j -= 1;
    }

    s.into_iter().collect()
}
