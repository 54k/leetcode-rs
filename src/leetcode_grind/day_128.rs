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

// https://leetcode.com/problems/number-of-atoms/description/
// https://leetcode.com/problems/number-of-atoms/editorial/
pub fn count_of_atoms(formula: String) -> String {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test357() {
        println!("{}", can_place_flowers(vec![1, 0, 0, 0, 1], 1)); // true
        println!("{}", can_place_flowers(vec![1, 0, 0, 0, 1], 2)); // false
    }

    #[test]
    fn test358() {
        println!("{}", count_of_atoms("Mg(OH)2".to_string())); // "H2MgO2"
        println!("{}", count_of_atoms("K4(ON(SO3)2)2".to_string())); // "K4(ON(SO3)2)2"
    }
}
