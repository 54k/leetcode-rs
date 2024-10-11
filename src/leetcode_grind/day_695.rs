// https://leetcode.com/problems/the-number-of-the-smallest-unoccupied-chair/description/?envType=daily-question&envId=2024-10-11
pub fn smallest_chair(mut times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
    let mut target_time = times[target_friend as usize].clone();
    times.sort_by_key(|a| a[0]);
    let n = times.len();
    let mut chair_time = vec![0; n];
    for time in times {
        for i in 0..n {
            if chair_time[i] <= time[0] {
                chair_time[i] = time[1];
                if time == target_time {
                    return i as i32;
                }
                break;
            }
        }
    }
    0
}
