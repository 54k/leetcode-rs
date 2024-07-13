// https://leetcode.com/problems/robot-collisions/description/?envType=daily-question&envId=2024-07-13
pub fn survived_robots_healths(
    positions: Vec<i32>,
    mut healths: Vec<i32>,
    directions: String,
) -> Vec<i32> {
    let directions = directions.chars().collect::<Vec<_>>();
    let n = positions.len();
    let mut indices = vec![0; n];
    let mut result = vec![];
    let mut stack = vec![];
    for i in 0..n {
        indices[i] = i;
    }
    indices.sort_by_key(|x| positions[*x]);

    for current_index in indices {
        if directions[current_index] == 'R' {
            stack.push(current_index);
        } else {
            while !stack.is_empty() && healths[current_index] > 0 {
                let top_index = stack.pop().unwrap();
                if healths[top_index] > healths[current_index] {
                    healths[top_index] -= 1;
                    healths[current_index] = 0;
                    stack.push(top_index);
                } else if healths[top_index] < healths[current_index] {
                    healths[current_index] -= 1;
                    healths[top_index] = 0;
                } else {
                    healths[current_index] = 0;
                    healths[top_index] = 0;
                }
            }
        }
    }
    for index in 0..n {
        if healths[index] > 0 {
            result.push(healths[index]);
        }
    }
    result
}
