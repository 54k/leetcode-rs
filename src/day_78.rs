pub fn distinct_integers(n: i32) -> i32 {
    use std::collections::*;
    let mut set = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(n);
    while let Some(x) = queue.pop_front() {
        set.insert(x);
        for i in 1..=x {
            if x % i == 1 {
                queue.push_back(i);
            }
        }
    }
    set.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test181() {
        println!("{}", distinct_integers(5)); // 4
        println!("{}", distinct_integers(3)); // 2
    }
}
