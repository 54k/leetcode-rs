use std::collections::HashMap;

const LEN: usize = 1000001;

struct MyHashSet {
    buf: [Option<i32>; LEN],
}

impl MyHashSet {
    fn new() -> Self {
        Self { buf: [None; LEN] }
    }

    fn add(&mut self, key: i32) {
        let mut start = self.hash_key(key);
        while self.buf[start].is_some() && *self.buf[start].as_ref().unwrap() != key {
            start = (start + 1) % LEN;
        }
        if self.buf[start].is_none() {
            self.buf[start] = Some(key);
        }
    }

    fn remove(&mut self, key: i32) {
        let mut start = self.hash_key(key);
        while self.buf[start].is_some() && *self.buf[start].as_ref().unwrap() != key {
            start = (start + 1) % LEN;
        }
        self.buf[start].take();
    }

    fn contains(&self, key: i32) -> bool {
        let mut start = self.hash_key(key);
        while self.buf[start].is_some() && *self.buf[start].as_ref().unwrap() != key {
            start = (start + 1) % LEN;
        }
        self.buf[start].is_some()
    }

    fn hash_key(&self, key: i32) -> usize {
        (key * 427 % 1000007 % LEN as i32).abs() as usize
    }
}

// https://leetcode.com/problems/design-hashmap/description/
struct MyHashMap {
    buf: [Option<(i32, i32)>; LEN],
}

impl MyHashMap {
    fn new() -> Self {
        Self { buf: [None; LEN] }
    }

    fn put(&mut self, key: i32, value: i32) {
        let mut start = self.hash_key(key);
        while self.buf[start].is_some() && self.buf[start].as_ref().unwrap().0 != key {
            start = (start + 1) % LEN;
        }
        if self.buf[start].is_none() {
            self.buf[start] = Some((key, value));
        } else {
            self.buf[start].as_mut().unwrap().1 = value;
        }
    }

    fn get(&self, key: i32) -> i32 {
        let mut start = self.hash_key(key);
        while self.buf[start].is_some() && self.buf[start].as_ref().unwrap().0 != key {
            start = (start + 1) % LEN;
        }
        self.buf[start].as_ref().unwrap_or(&(0, -1)).1
    }

    fn remove(&mut self, key: i32) {
        let mut start = self.hash_key(key);
        while self.buf[start].is_some() && self.buf[start].as_ref().unwrap().0 != key {
            start = (start + 1) % LEN;
        }
        self.buf[start].take();
    }

    fn hash_key(&self, key: i32) -> usize {
        (key * 427 % 1000007 % LEN as i32).abs() as usize
    }
}

// https://leetcode.com/problems/happy-number/description/
pub fn is_happy(mut n: i32) -> bool {
    fn get_next(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 0 {
            ans += (n % 10).pow(2);
            n /= 10;
        }
        ans
    }
    use std::collections::HashSet;
    let mut set = HashSet::new();
    while n != 1 {
        set.insert(n);
        let next = get_next(n);
        if set.contains(&next) {
            return false;
        }
        n = next;
    }
    true
}

// https://leetcode.com/problems/minimum-index-sum-of-two-lists/description/
pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    use std::collections::HashMap;
    #[inline]
    fn to_map(list: Vec<String>) -> HashMap<String, i32> {
        let mut map = HashMap::new();
        for (i, s) in list.into_iter().enumerate() {
            map.insert(s, i as i32);
        }
        map
    }
    let list1 = to_map(list1);
    let mut min_sum = i32::MAX;
    let mut res = vec![];
    for (i, s) in list2.into_iter().enumerate() {
        if list1.contains_key(&s) {
            let sum = list1.get(&s).unwrap() + i as i32;
            if sum < min_sum {
                res.clear();
                res.push(s);
                min_sum = sum;
            } else if (sum == min_sum) {
                res.push(s);
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test490() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        println!("{}", set.contains(1));
        println!("{}", set.contains(2));
        set.add(2);
        println!("{}", set.contains(2));
        set.remove(2);
        println!("{}", set.contains(2));
    }
}
