#[allow(dead_code)]
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total_gas = 0;
    let mut total_cost = 0;
    let mut cur_gas = 0;
    let mut starting_point = 0;
    for i in 0..gas.len() {
        total_gas += gas[i];
        total_cost += cost[i];
        cur_gas += gas[i] - cost[i];
        if cur_gas < 0 {
            starting_point = i + 1;
            cur_gas = 0;
        }
    }
    if total_gas < total_cost {
        -1
    } else {
        starting_point as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test148() {
        println!(
            "{}",
            can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
        );
    }
}
