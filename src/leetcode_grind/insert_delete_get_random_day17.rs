use std::collections::HashMap;

extern "C" {
    fn rand() -> i32;
}

struct RandomizedSet {
    buf: Vec<i32>,
    map: HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            buf: vec![],
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.get(&val).is_some() {
            return false;
        }
        self.buf.push(val);
        self.map.insert(val, self.buf.len() - 1);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.map.get(&val).is_none() {
            return false;
        }
        let idx = self.map.remove(&val).unwrap();
        let len = self.buf.len();

        if idx == len - 1 {
            self.buf.pop();
            return true;
        }

        self.buf.swap(idx, len - 1);
        self.buf.pop();
        self.map.insert(self.buf[idx], idx);

        true
    }

    fn get_random(&self) -> i32 {
        self.buf[unsafe { (rand() % self.buf.len() as i32) as usize }]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut set = RandomizedSet::new();
        assert!(!set.remove(0));
        assert!(set.insert(0));
        assert!(!set.insert(0));
        assert_eq!(0, set.get_random());
        assert!(set.remove(0));
        assert!(!set.remove(0));
    }
}
