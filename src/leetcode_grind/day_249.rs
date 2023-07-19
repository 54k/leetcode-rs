// https://leetcode.com/problems/partition-labels/description/
pub fn partition_labels(s: String) -> Vec<i32> {
    let s = s.chars().collect::<Vec<_>>();
    let mut last = vec![0; 26];

    for i in 0..s.len() {
        last[s[i] as usize - 'a' as usize] = i;
    }

    let (mut anchor, mut j) = (0, 0); // anchor is start, j is the end
    let mut ans = vec![];

    for i in 0..s.len() {
        j = j.max(last[s[i] as usize - 'a' as usize]);
        if i == j {
            ans.push((i - anchor + 1) as i32);
            anchor = i + 1;
        }
    }

    ans
}

// https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays/description/
pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    fn find_the_prefix_common_array_counting_approach(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut counter = vec![0; 51];
        let mut ans = vec![];
        let mut cnt = 0;

        for i in 0..a.len() {
            let (idx_1, idx_2) = (a[i] as usize, b[i] as usize);
            counter[idx_1] += 1;

            if counter[idx_1] == 2 {
                cnt += 1;
            }

            counter[idx_2] += 1;

            if counter[idx_2] == 2 {
                cnt += 1;
            }

            ans.push(cnt);
        }

        ans
    }
    fn find_the_prefix_common_array_bitsets_approach(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let (mut set_a, mut set_b) = (0u64, 0u64);
        for i in 0..a.len() {
            set_a |= a[i] as u64;
            set_b |= b[i] as u64;

            ans.push((set_a & set_b).count_ones() as i32);
        }
        ans
    }
    find_the_prefix_common_array_bitsets_approach(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_labels() {
        println!("{:?}", partition_labels("abccaddbeffe".to_string()));
    }
}
