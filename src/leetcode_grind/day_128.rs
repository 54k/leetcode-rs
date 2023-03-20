// https://leetcode.com/problems/can-place-flowers/description/
// https://leetcode.com/problems/can-place-flowers/editorial/
pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    let mut placed = 0;
    for i in 0..flowerbed.len() {
        if flowerbed[i] == 1 {
            continue;
        }
        let left_empty = i == 0 || flowerbed[i - 1] == 0;
        let right_empty = i == flowerbed.len() - 1 || flowerbed[i + 1] == 0;
        if left_empty && right_empty {
            placed += 1;
            flowerbed[i] = 1;
            if placed == n {
                return true;
            }
        }
    }
    placed >= n
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test357() {
        println!("{}", can_place_flowers(vec![1, 0, 0, 0, 1], 1)); // true
        println!("{}", can_place_flowers(vec![1, 0, 0, 0, 1], 2)); // false
    }
}
