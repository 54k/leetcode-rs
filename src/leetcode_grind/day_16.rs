// Constraints:
//
// 1 <= matches.length <= 105
// matches[i].length == 2
// 1 <= winneri, loseri <= 105
// winneri != loseri
// All matches[i] are unique.
#[allow(dead_code)]
pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut losses_count = vec![-1; (10i32.pow(5) + 1) as usize];

    for m in matches.iter() {
        let (w, l) = (m[0] as usize, m[1] as usize);
        if losses_count[w] == -1 {
            losses_count[w] = 0;
        }

        if losses_count[l] == -1 {
            losses_count[l] = 1;
        } else {
            losses_count[l] += 1;
        }
    }

    let mut ans = vec![vec![]; 2];

    for (i, v) in losses_count.into_iter().enumerate() {
        if (0..=1).contains(&v) {
            ans[v as usize].push(i as i32);
        }
    }

    ans
}

#[allow(dead_code)]
pub fn check_if_pangram(sentence: String) -> bool {
    fn check_if_pangram_count(sentence: String) -> bool {
        if sentence.len() < 26 {
            return false;
        }
        let mut set = [0; 26];
        for ch in sentence.chars() {
            set[ch as usize - 'a' as usize] += 1;
        }
        for i in set {
            if i == 0 {
                return false;
            }
        }
        true
    }

    fn check_if_pangram_bits(sentence: String) -> bool {
        let mut seen = 0;

        for ch in sentence.chars() {
            let i = ch as i32 - 'a' as i32;
            seen |= 1 << i;
        }

        seen == (1 << 26) - 1
    }

    check_if_pangram_bits(sentence)
}

//https://www.nayuki.io/page/next-lexicographical-permutation-algorithm
#[allow(dead_code)]
pub fn next_permutation(nums: &mut Vec<i32>) {
    let mut pivot = -1;
    let len = nums.len();
    for i in (1..len).rev() {
        if nums[i - 1] < nums[i] {
            pivot = (i - 1) as i32;
            break;
        }
    }

    if pivot == -1 {
        nums.reverse();
        return;
    }

    let pivot = pivot as usize;
    for j in (pivot + 1..len).rev() {
        if nums[j] > nums[pivot] {
            nums.swap(pivot, j);
            nums[pivot + 1..len].reverse();
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day_16::*;

    #[test]
    fn test83() {
        println!(
            "{:?}",
            find_winners(vec![
                vec![1, 3],
                vec![2, 3],
                vec![3, 6],
                vec![5, 6],
                vec![5, 7],
                vec![4, 5],
                vec![4, 8],
                vec![4, 9],
                vec![10, 4],
                vec![10, 9],
            ])
        ); // [[1,2,10],[4,5,7,8]]

        println!(
            "{:?}",
            find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]])
        ); // [[1,2,10],[4,5,7,8]]
    }

    #[test]
    fn test84() {
        println!(
            "{}",
            check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_owned())
        );
    }

    #[test]
    fn test85() {
        let mut v = vec![1, 2, 3];
        next_permutation(&mut v);
        println!("{:?}", v); // 1 3 2

        let mut v = vec![3, 2, 1];
        next_permutation(&mut v);
        println!("{:?}", v); // 1 2 3
    }
}
