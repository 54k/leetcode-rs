// https://leetcode.com/problems/paint-house-ii/description/
pub fn min_cost_ii_i(costs: Vec<Vec<i32>>) -> i32 {
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

pub fn min_cost_ii_ii(mut costs: Vec<Vec<i32>>) -> i32 {
    if costs.len() == 0 {
        return 0;
    }
    let k = costs[0].len();
    let n = costs.len();

    for house in 1..n {
        let mut min_color = usize::MAX;
        let mut second_min_color = usize::MAX;

        for color in 0..k {
            let cost = costs[house - 1][color];
            if min_color == usize::MAX || cost < costs[house - 1][min_color] {
                second_min_color = min_color;
                min_color = color;
            } else if second_min_color == usize::MAX || cost < costs[house - 1][second_min_color] {
                second_min_color = color;
            }
        }

        for color in 0..k {
            if color == min_color {
                costs[house][color] += costs[house - 1][second_min_color];
            } else {
                costs[house][color] += costs[house - 1][min_color];
            }
        }
    }

    let mut min = i32::MAX;
    for &c in &costs[n - 1] {
        min = min.min(c);
    }
    min
}
