// Given an array of integers temperatures represents the daily temperatures,
// return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature.
// If there is no future day for which this is possible, keep answer[i] == 0 instead.
#[allow(dead_code)]
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; temperatures.len()];
    let mut mono_stack: Vec<(i32, usize)> = vec![];
    for (i, t) in temperatures.into_iter().enumerate() {
        while !mono_stack.is_empty() && mono_stack[mono_stack.len() - 1].0 < t {
            let e = mono_stack.pop().unwrap();
            ans[e.1] = i as i32 - e.1 as i32;
        }
        mono_stack.push((t, i));
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test120() {
        println!(
            "{:?}",
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
        ); // [1,1,4,2,1,1,0,0]
        println!("{:?}", daily_temperatures(vec![30, 40, 50, 60])); // [1,1,1,0]
    }
}
