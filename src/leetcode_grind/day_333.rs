// https://leetcode.com/problems/number-of-flowers-in-full-bloom/description/
pub fn full_bloom_flowers_heap(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;

    let mut flowers = flowers;
    flowers.sort();

    let mut sorted_people = people.clone();
    sorted_people.sort();

    let mut dic = HashMap::new();
    let mut heap = BinaryHeap::new();

    let mut i = 0;
    for person in sorted_people {
        while i < flowers.len() && flowers[i][0] <= person {
            heap.push(Reverse(flowers[i][1]));
            i += 1;
        }

        while !heap.is_empty() && heap.peek().unwrap().0 < person {
            heap.pop();
        }

        dic.insert(person, heap.len() as i32);
    }

    let mut ans = vec![0; people.len()];
    for j in 0..people.len() {
        ans[j] = dic[&people[j]];
    }

    ans
}

pub fn full_bloom_flowers_binary_search(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
    fn binary_search(positions: &Vec<i32>, target: i32) -> usize {
        let mut left = 0;
        let mut right = positions.len() as i32;

        while left < right {
            let mid = (left + right) / 2;
            if positions[mid as usize] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as usize
    }

    use std::collections::BTreeMap;
    let mut difference = BTreeMap::new();
    difference.insert(0, 0);

    for flower in flowers {
        let start = flower[0];
        let end = flower[1] + 1;

        *difference.entry(start).or_insert(0) += 1;
        *difference.entry(end).or_insert(0) -= 1;
    }

    let mut positions = vec![];
    let mut prefix = vec![];
    let mut curr = 0;

    for &key in difference.keys() {
        positions.push(key);
        curr += difference[&key];
        prefix.push(curr);
    }

    let mut ans = vec![0; people.len()];
    for j in 0..people.len() {
        let i = binary_search(&positions, people[j]) - 1;
        ans[j] = prefix[i];
    }
    ans
}

pub fn full_bloom_flowers_simpler_binary_search(
    flowers: Vec<Vec<i32>>,
    people: Vec<i32>,
) -> Vec<i32> {
    fn binary_search(arr: &Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = arr.len() as i32;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if arr[mid as usize] <= target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }

    let mut starts = vec![];
    let mut ends = vec![];

    for flower in flowers {
        let start = flower[0];
        let end = flower[1];
        starts.push(start);
        ends.push(end + 1);
    }
    starts.sort();
    ends.sort();

    let mut ans = vec![];
    for person in people {
        let s = binary_search(&starts, person);
        let e = binary_search(&ends, person);
        ans.push(s - e);
    }
    ans
}
