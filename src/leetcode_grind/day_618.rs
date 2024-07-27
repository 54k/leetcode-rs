pub fn minimum_cost(
    source: String,
    target: String,
    original: Vec<char>,
    changed: Vec<char>,
    cost: Vec<i32>,
) -> i64 {
    let source = source.chars().collect::<Vec<_>>();
    let target = target.chars().collect::<Vec<_>>();

    let mut total_cost = 0;
    let mut min_cost = vec![vec![i32::MAX as i64; 26]; 26];

    for i in 0..original.len() {
        let start_char = original[i] as usize - 'a' as usize;
        let end_char = changed[i] as usize - 'a' as usize;

        min_cost[start_char][end_char] = min_cost[start_char][end_char].min(cost[i] as i64);
    }

    for k in 0..26 {
        for i in 0..26 {
            for j in 0..26 {
                min_cost[i][j] = min_cost[i][j].min(min_cost[i][k] + min_cost[k][j]);
            }
        }
    }

    for i in 0..source.len() {
        if source[i] == target[i] {
            continue;
        }
        let source_char = source[i] as usize - 'a' as usize;
        let target_char = target[i] as usize - 'a' as usize;

        if min_cost[source_char][target_char] >= i32::MAX as i64 {
            return -1;
        }
        total_cost += min_cost[source_char][target_char];
    }

    total_cost
}
