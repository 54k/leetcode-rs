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

// https://leetcode.com/problems/queue-reconstruction-by-height/description
pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people = people;
    people.sort_by(|x, y| {
        if x[0] == y[0] {
            return x[1].cmp(&y[1]);
        }
        y[0].cmp(&x[0])
    });

    let mut queue = vec![vec![0, 0]; people.len()];
    for peep in &people {
        queue.insert(peep[1] as usize, peep.clone());
    }
    queue[0..people.len()].to_vec()
}

// https://leetcode.com/problems/minimum-number-of-frogs-croaking
pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
    let mut c = 0;
    let mut r = 0;
    let mut o = 0;
    let mut a = 0;
    let mut k = 0;

    for ch in croak_of_frogs.chars() {
        match ch {
            'c' => c += 1,
            'r' => r += 1,
            'o' => o += 1,
            'a' => a += 1,
            'k' => k += 1,
            _ => return -1,
        }

        if c < r || r < o || o < a || a < k {
            return -1;
        }
    }

    println!("{} {} {} {} {}", c, r, o, a, k);

    if c != r || r != o || o != a || a != k {
        return -1;
    }

    let mut temp_c = 0;
    let mut max_c = 0;
    for ch in croak_of_frogs.chars() {
        if ch == 'c' {
            temp_c += 1;
            max_c = max_c.max(temp_c);
        } else if ch == 'k' {
            temp_c -= 1;
        }
    }
    max_c
}

// https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/
pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;
    let mut start = vec![];
    let mut end = vec![];

    for interval in intervals {
        start.push(interval[0]);
        end.push(interval[1]);
    }

    start.sort();
    end.sort();

    let mut s_ptr = 0;
    let mut e_ptr = 0;

    let mut crossed = 0;
    while s_ptr < start.len() {
        if start[s_ptr] > end[e_ptr] {
            e_ptr += 1;
            crossed -= 1;
        }
        s_ptr += 1;
        crossed += 1;
    }
    crossed // intervealed intervals is a number of groups
}
