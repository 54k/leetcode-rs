// https://leetcode.com/problems/successful-pairs-of-spells-and-potions/
// https://leetcode.com/problems/successful-pairs-of-spells-and-potions/editorial/
pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    fn bin_search_approach(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        fn lower_bound(arr: &[i64], t: i64) -> usize {
            let mut lo = 0;
            let mut hi = arr.len() - 1;
            while lo < hi {
                let mut mid = lo + (hi - lo) / 2;
                if arr[mid] < t {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            lo
        }
        let spells = spells.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut potions = potions.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        potions.sort();
        let mut ans = vec![0; spells.len()];
        let max_potion = potions[potions.len() - 1];
        for i in 0..spells.len() {
            let min_potion = (1.0 * success as f64 / spells[i] as f64).ceil() as i64;
            if max_potion < min_potion {
                ans[i] = 0;
                continue;
            }
            ans[i] = potions.len() as i32 - lower_bound(&potions, min_potion) as i32;
        }
        ans
    }
    fn two_pointers_approach(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let n = spells.len();
        let m = potions.len();

        let mut spells = spells.into_iter().enumerate().map(|(i, x)| (x as i64, i)).collect::<Vec<_>>();
        spells.sort();
        let mut potions = potions.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        potions.sort();
        let mut ans = vec![0; n];
        let mut potion_idx = m as i32 - 1;
        for (spell, s_i) in spells {
            while potion_idx >= 0 && spell * potions[potion_idx as usize] >= success {
                potion_idx -= 1;
            }
            ans[s_i] = m as i32 - (potion_idx + 1);
        }
        ans
    }
    two_pointers_approach(spells, potions, success)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test390() {
        println!("{:?}", successful_pairs(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 2, 3, 4, 5, 6, 7], 25)); // [0, 0, 0, 1, 3, 3, 4]
        println!("{:?}", successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7)); // [4,0,3]
    }
}