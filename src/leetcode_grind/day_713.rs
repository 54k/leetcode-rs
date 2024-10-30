// https://leetcode.com/problems/paint-house-ii/description/
pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
    if costs.len() == 0 {
        return 0;
    }
    let k = costs[0].len();
    let n = costs.len();

    let mut prev_row = costs[0].clone();

    for house in 1..n {
        let mut current_row = vec![0; k];
        for color in 0..k {
            let mut min = i32::MAX;
            for previous_color in 0..k {
                if color == previous_color {
                    continue;
                }
                min = min.min(prev_row[previous_color]);
            }
            current_row[color] += min + costs[house][color];
        }
        prev_row = current_row;
    }

    let mut min = i32::MAX;
    for c in prev_row {
        min = min.min(c);
    }
    min
}
