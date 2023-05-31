// https://leetcode.com/problems/design-underground-system/description/
use std::collections::HashMap;

struct UndergroundSystem {
    check_in: HashMap<i32, (String, i32)>,
    trips: HashMap<String, (f64, f64)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Self {
            check_in: HashMap::new(),
            trips: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_in.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (start_station, check_in_time) = self.check_in.get(&id).unwrap();
        let route_key = UndergroundSystem::get_station_key(start_station.clone(), station_name);
        let mut trip = self.trips.entry(route_key).or_insert((0.0, 0.0));
        trip.0 += t as f64 - *check_in_time as f64;
        trip.1 += 1.0;
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let route_key = UndergroundSystem::get_station_key(start_station, end_station);
        let route = self.trips[&route_key];
        route.0 / route.1
    }

    fn get_station_key(start_station: String, end_station: String) -> String {
        format!("{}->{}", start_station, end_station)
    }
}

// https://leetcode.com/problems/power-of-two/description/
pub fn is_power_of_two(n: i32) -> bool {
    let n = n as i64;
    if n != 0 {
        n & (-n) == n
    } else {
        false
    }
}

// https://leetcode.com/problems/k-radius-subarray-averages/description/
pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 0 {
        return nums;
    }
    let mut prefix = vec![0];
    for &n in &nums {
        let l = *prefix.last().unwrap();
        prefix.push(l + n as i64);
    }
    let k = k as usize;
    let n = nums.len();
    let mut ans = vec![-1; n];

    if 2 * k + 1 > n {
        return ans;
    }

    for i in k..nums.len() - k {
        ans[i] = ((prefix[i + 1 + k] - prefix[i - k]) / (2 * k as i64 + 1)) as i32;
    }
    ans
}

// https://leetcode.com/problems/stone-game-iv/description/
pub fn winner_square_game(n: i32) -> bool {
    todo!()
}

// https://leetcode.com/problems/stone-game-v/description/
pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-vi/description/
pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-vii/description/
pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/description/
pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/range-sum-query-2d-mutable/description/
mod rmq2d {
    struct NumMatrix {}

    impl NumMatrix {
        fn new(matrix: Vec<Vec<i32>>) -> Self {
            todo!()
        }

        fn update(&self, row: i32, col: i32, val: i32) {
            todo!()
        }

        fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            todo!()
        }
    }
}

// https://leetcode.com/problems/maximize-grid-happiness/description/
pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
    todo!()
}
