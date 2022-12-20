pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    fn dfs(rooms: Vec<Vec<i32>>) -> bool {
        fn dfs(room: usize, rooms: &Vec<Vec<i32>>, visited: &mut Vec<bool>) {
            if visited[room] {
                return;
            }
            visited[room] = true;
            for v in &rooms[room] {
                dfs(*v as usize, rooms, visited);
            }
        }
        let mut visited = vec![false; rooms.len()];
        dfs(0, &rooms, &mut visited);
        for v in visited {
            if !v {
                return false;
            }
        }
        true
    }

    fn stack(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; rooms.len()];
        let mut stack = vec![];
        stack.push(0);
        while !stack.is_empty() {
            let e: usize = stack.pop().unwrap() as usize;
            visited[e] = true;
            for v in &rooms[e] {
                if !visited[*v as usize] {
                    stack.push(*v);
                }
            }
        }
        for v in visited {
            if !v {
                return false;
            }
        }
        true
    }

    stack(rooms)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test122() {
        println!(
            "{}",
            can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]])
        ); // true
        println!(
            "{}",
            can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]])
        ); // false
    }
}
