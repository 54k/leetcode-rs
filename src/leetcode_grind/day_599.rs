// https://leetcode.com/problems/water-bottles/description/
pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
    let mut consumed_bottles = 0;
    while num_bottles >= num_exchange {
        let k = num_bottles / num_exchange;
        consumed_bottles += num_exchange * k;
        num_bottles -= num_exchange * k;
        num_bottles += k;
    }
    consumed_bottles + num_bottles
}
