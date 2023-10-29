// https://leetcode.com/problems/poor-pigs/description/
pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
    let mut states = minutes_to_test / minutes_to_die + 1;
    ((buckets as f64).log2() / (states as f64).log2()).ceil() as i32
}

// https://leetcode.com/problems/eliminate-maximum-number-of-monsters/description/
pub fn eliminate_maximum_sort_by_arrival_time(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut arrivals = vec![];
    for i in 0..dist.len() {
        arrivals.push(dist[i] as f64 / speed[i] as f64);
    }

    arrivals.sort_by_key(|x| x.to_string());

    let mut ans = 0;
    for i in 0..dist.len() {
        if arrivals[i] <= i as f64 {
            break;
        }
        ans += 1;
    }
    ans
}

// https://leetcode.com/problems/brick-wall/description/
pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
    
}
