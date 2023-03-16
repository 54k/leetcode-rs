#[allow(dead_code)]
pub fn number_of_cuts(n: i32) -> i32 {
    match n {
        1 => 0,
        n if n % 2 == 0 => n / 2,
        _ => n,
    }
}

#[allow(dead_code)]
pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let mut start = 1;
    let mut end = x;
    while start < end {
        let mid = start + (end - start) / 2;
        if mid <= x / mid && (mid + 1) > x / (mid + 1)
        // Found the result
        {
            return mid;
        } else if mid > x / mid {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    start
}

#[allow(dead_code)]
pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
    fn flood(r: i32, c: i32, grid: &mut Vec<Vec<i32>>) {
        if r < 0
            || c < 0
            || r == grid.len() as i32
            || c == grid[0].len() as i32
            || grid[r as usize][c as usize] == 0
        {
            return;
        }

        let r = r as usize;
        let c = c as usize;

        grid[r][c] = 0;

        flood(r as i32 + 1, c as i32, grid);
        flood(r as i32 - 1, c as i32, grid);
        flood(r as i32, c as i32 + 1, grid);
        flood(r as i32, c as i32 - 1, grid);
    }

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if r * c == 0 || r == grid.len() - 1 || c == grid[0].len() - 1 {
                flood(r as i32, c as i32, &mut grid);
            }
        }
    }

    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 1 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test86() {
        println!("{}", number_of_cuts(4)); // 2
        println!("{}", number_of_cuts(3)); // 3
        println!("{}", number_of_cuts(1)); // 0
    }

    #[test]
    fn test87() {
        println!("{}", my_sqrt(4)); // 2
        println!("{}", my_sqrt(3)); // 1
        println!("{}", my_sqrt(1)); // 1
    }

    #[test]
    fn test88() {
        println!(
            "{}",
            num_enclaves(vec![
                vec![0, 0, 0, 0],
                vec![1, 0, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0],
            ])
        ); // 3

        println!(
            "{}",
            num_enclaves(vec![
                vec![0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1],
                vec![1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0],
                vec![1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1],
                vec![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1],
                vec![0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0],
                vec![1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0],
            ])
        ); // 11
    }
}
