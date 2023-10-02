// https://leetcode.com/problems/remove-colored-pieces-if-both-neighbors-are-the-same-color/description
pub fn winner_of_game(colors: String) -> bool {
    let colors = colors.chars().collect::<Vec<_>>();
    let mut alice = 0;
    let mut bob = 0;
    for i in 1..colors.len() - 1 {
        if colors[i - 1] == colors[i] && colors[i] == colors[i + 1] {
            if colors[i] == 'A' {
                alice += 1;
            } else {
                bob += 1;
            }
        }
    }
    alice - bob >= 1
}
