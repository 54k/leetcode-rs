// https://leetcode.com/problems/sort-array-by-increasing-frequency/
pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
    const SHIFT: i32 = 100;
    let mut freq = vec![0; 201];
    for num in nums.iter() {
        let idx = (num + SHIFT) as usize;
        freq[idx] += 1;
    }
    nums.sort_by(|a, b| {
        let freq_a = freq[(a + SHIFT) as usize];
        let freq_b = freq[(b + SHIFT) as usize];
        if freq_a == freq_b {
            b.cmp(a)
        } else {
            freq_a.cmp(&freq_b)
        }
    });
    nums
}

// https://leetcode.com/problems/percentage-of-letter-in-string/
pub fn percentage_letter(s: String, letter: char) -> i32 {
    let len = s.len();
    let mut count = 0;
    for ch in s.chars() {
        if ch == letter {
            count += 1;
        }
    }
    count * 100 / len as i32
}

// https://leetcode.com/problems/top-k-frequent-elements/description/
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::cmp::*;
    use std::collections::*;
    let mut frequencies = HashMap::new();
    let mut heap = BinaryHeap::new();
    for num in nums {
        *frequencies.entry(num).or_insert(0) += 1;
    }
    for (num, freq) in frequencies {
        heap.push(Reverse((freq, num)));
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    heap.into_vec()
        .into_iter()
        .map(|Reverse((_, v))| v)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test254() {
        println!("{:?}", frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1])); // [5,-1,4,4,-6,-6,1,1,1]
    }

    #[test]
    fn test255() {
        println!("{}", percentage_letter("foobar".to_string(), 'o')); // 33
        println!("{}", percentage_letter("sgawtb".to_string(), 's')); // 16
        println!("{}", percentage_letter("ktaseevvqmbylnrljrkfngzpicdffuimyzqzipspmdimvgqmyqltmwbbpeakifiadaaaiepbgtanfxnndxpeiqwycgbvlpihtfbd".to_string(), 'p'));
        // 33
    }

    #[test]
    fn test256() {
        println!("{:?}", top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)); //[1,2]
    }
}
