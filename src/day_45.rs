pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, mut additional_rocks: i32) -> i32 {
    let mut remaining = vec![];

    for i in 0..capacity.len() {
        remaining.push(capacity[i] - rocks[i]);
    }
    remaining.sort();

    for i in 0..remaining.len() {
        if remaining[i] <= additional_rocks {
            additional_rocks -= remaining[i];
            remaining[i] -= remaining[i];
        }
    }

    remaining.into_iter().filter(|x| *x == 0).count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test130() {
        println!("{}", maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2)); // 3
        println!("{}", maximum_bags(vec![10, 2, 2], vec![2, 2, 0], 100)); // 3
    }
}
