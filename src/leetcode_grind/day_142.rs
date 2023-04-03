// https://leetcode.com/problems/boats-to-save-people/
// https://leetcode.com/problems/boats-to-save-people/editorial/
pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort();
    let mut ans = 0;
    let mut i = 0;
    let mut j = people.len() as i32 - 1;
    while i <= j {
        ans += 1;
        if people[i as usize] + people[j as usize] <= limit {
            i += 1;
        }
        j -= 1;
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test391() {
        println!("{}", num_rescue_boats(vec![3, 5, 3, 4], 5)); // 4
    }
}
