// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/description
pub fn sort_by_bits_built_in_comparator(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    arr.sort_by(|x, y| {
        if x.count_ones() == y.count_ones() {
            return x.cmp(&y);
        }
        return x.count_ones().cmp(&y.count_ones());
    });
    arr
}

pub fn sort_by_bits_bit_manipulation(arr: Vec<i32>) -> Vec<i32> {
    fn find_weight(mut num: i32) -> i32 {
        let mut mask = 1;
        let mut weight = 0;

        while num > 0 {
            if (num & mask) > 0 {
                weight += 1;
                num ^= mask;
            }

            mask <<= 1;
        }

        weight
    }

    let mut arr = arr;
    arr.sort_by(|x, y| {
        if find_weight(*x) == find_weight(*y) {
            return x.cmp(&y);
        }
        find_weight(*x).cmp(&find_weight(*y))
    });
    arr
}

pub fn sort_by_bits_brian_kernigan(arr: Vec<i32>) -> Vec<i32> {
    fn find_weight(mut num: i32) -> i32 {
        let mut count = 0;
        while num > 0 {
            num &= num - 1;
            count += 1;
        }
        count
    }
    let mut arr = arr;
    arr.sort_by(|x, y| {
        if find_weight(*x) == find_weight(*y) {
            return x.cmp(&y);
        }
        find_weight(*x).cmp(&find_weight(*y))
    });
    arr
}
