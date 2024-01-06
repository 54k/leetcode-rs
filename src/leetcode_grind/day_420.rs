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

