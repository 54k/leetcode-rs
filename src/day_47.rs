#[allow(dead_code)]
pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut next_tasks = BinaryHeap::new();

    let mut sorted_tasks = vec![];
    for (i, v) in tasks.iter().enumerate() {
        sorted_tasks.push((v[0], v[1], i));
    }
    sorted_tasks.sort_by(|x, y| x.0.cmp(&y.0));

    let mut result = vec![];

    let mut curr_time = 0;
    let mut task_idx = 0;

    while task_idx < tasks.len() || !next_tasks.is_empty() {
        if next_tasks.is_empty() && curr_time < sorted_tasks[task_idx].0 {
            curr_time = sorted_tasks[task_idx].0;
        }

        while task_idx < tasks.len() && curr_time >= sorted_tasks[task_idx].0 {
            next_tasks.push(Reverse((
                sorted_tasks[task_idx].1,
                sorted_tasks[task_idx].2,
            )));
            task_idx += 1;
        }

        let next_task = next_tasks.pop().unwrap().0;
        let proc_time = next_task.0;
        let idx = next_task.1;

        curr_time += proc_time;
        result.push(idx as i32);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test134() {
        println!(
            "{:?}",
            get_order(vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]])
        ); // [0, 2, 3, 1]

        println!("{:?}", get_order(vec![vec![7, 1], vec![6, 3], vec![1, 3]])); // [2, 1, 0]

        println!(
            "{:?}",
            get_order(vec![
                vec![7, 10],
                vec![7, 12],
                vec![7, 5],
                vec![7, 4],
                vec![7, 2]
            ])
        ); // [4, 3, 2, 0, 1]
    }
}
