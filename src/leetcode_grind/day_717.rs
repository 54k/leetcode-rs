// https://leetcode.com/problems/rotate-string/description/?envType=daily-question&envId=2024-11-03
pub fn rotate_string(s: String, goal: String) -> bool {
    s.len() == goal.len() && (s.clone() + &s.to_string()).contains(&goal)
}
