// https://leetcode.com/problems/path-with-minimum-effort/description/
pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    fn dfs(heights: &Vec<Vec<i32>>, effort: i32) -> bool {
        const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut seen = vec![vec![false; heights[0].len()]; heights.len()];
        let mut stack = vec![];
        stack.push((0, 0));
        seen[0][0] = true;
        while let Some((x, y)) = stack.pop() {
            if x == heights.len() as i32 - 1 && y == heights[0].len() as i32 - 1 {
                return true;
            }
            for d in DIR {
                let (nx, ny) = (x + d.0, y + d.1);
                if nx >= 0
                    && nx < heights.len() as i32
                    && ny >= 0
                    && ny < heights[0].len() as i32
                    && !seen[nx as usize][ny as usize]
                    && effort
                        >= (heights[nx as usize][ny as usize] - heights[x as usize][y as usize])
                            .abs()
                {
                    seen[nx as usize][ny as usize] = true;
                    stack.push((nx, ny));
                }
            }
        }
        false
    }

    let mut lo = 0;
    let mut hi = 0;
    for h in &heights {
        for j in h {
            hi = hi.max(*j);
        }
    }

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;

        if dfs(&heights, mid) {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

// https://leetcode.com/problems/minimum-speed-to-arrive-on-time/description/
pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
    fn check(dist: &Vec<i32>, hour: f64, speed: f64) -> bool {
        let mut time = 0.0;
        for d in dist {
            time = f64::ceil(time);
            time += *d as f64 / speed;
        }
        time <= hour
    }

    if dist.len() as i32 > hour.ceil() as i32 {
        return -1;
    }
    let mut lo = 1;
    let mut hi = 10000000;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if check(&dist, hour, mid as f64) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

// https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/description/
pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    fn check(nums: &Vec<i32>, div: i32, threshold: i32) -> bool {
        let mut sum = 0;
        for &n in nums {
            sum += (n as f64 / div as f64).ceil() as i32;
        }
        sum <= threshold
    }
    let mut lo = 1;
    let mut hi = nums.iter().copied().max().unwrap();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if check(&nums, mid, threshold) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

// https://leetcode.com/problems/divide-chocolate/description/
pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
    let num_of_ppl = k + 1;
    let mut left = sweetness.iter().copied().min().unwrap();
    let mut right = sweetness.iter().copied().sum::<i32>() / num_of_ppl;

    while left < right {
        let mid = (left + right + 1) / 2;
        let mut cur_sweetness = 0;
        let mut ppl_with_choco = 0;

        for &s in &sweetness {
            cur_sweetness += s;
            if cur_sweetness >= mid {
                ppl_with_choco += 1;
                cur_sweetness = 0;
            }
        }

        if ppl_with_choco >= num_of_ppl {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    right
}

// https://leetcode.com/problems/minimize-max-distance-to-gas-station/description/
pub fn minmax_gas_dist(stations: Vec<i32>, k: i32) -> f64 {
    fn possible(d: f64, stations: &Vec<i32>, k: i32) -> bool {
        let mut used = 0;
        for i in 0..stations.len() - 1 {
            used += ((stations[i + 1] as f64 - stations[i] as f64) / d) as i32;
        }
        used <= k
    }
    let mut lo = 0.0;
    let mut hi = 1e8;
    while hi - lo > 1e-6 {
        let mid = (lo + hi) / 2.0;
        if possible(mid, &stations, k) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    lo
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_444() {
        println!(
            "{}",
            minimum_effort_path(vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]])
        ); // 2
    }

    #[test]
    fn test_445() {
        println!("{}", min_speed_on_time(vec![1, 3, 2], 6.0)); // 1
    }

    #[test]
    fn test_446() {
        println!("{}", smallest_divisor(vec![1, 2, 5, 9], 6)); // 5
        println!("{}", smallest_divisor(vec![44, 22, 33, 11, 1], 5)); // 44
        println!("{}", smallest_divisor(vec![1000000], 2)); // 50000
    }

    #[test]
    fn test_447() {
        println!("{}", maximize_sweetness(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 5)); // 6
    }

    #[test]
    fn test_448() {
        println!(
            "{}",
            minmax_gas_dist(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 9)
        ); // 0.5
    }
}
