// https://leetcode.com/problems/design-hashmap/description
const SIZE: usize = 4096;

#[derive(Debug, Clone)]
struct Node {
    key: i32,
    val: i32,
    next: Option<Box<Node>>,
}

struct MyHashMap {
    buckets: Vec<Option<Box<Node>>>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            buckets: vec![None; SIZE],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let h = MyHashMap::hash(key);
        let mut head = &mut self.buckets[h];

        if head.is_none() {
            self.buckets[h] = Some(Box::new(Node {
                key,
                val: value,
                next: None,
            }));
            return;
        }

        while let Some(h) = head {
            if h.key == key {
                h.val = value;
                return;
            }
            head = &mut h.next;
        }

        let _ = head.insert(Box::new(Node {
            key,
            val: value,
            next: None,
        }));
    }

    fn get(&self, key: i32) -> i32 {
        let h = MyHashMap::hash(key);
        let mut head = self.buckets[h].as_ref();
        if head.is_none() {
            return -1;
        }
        while let Some(h) = head {
            if h.key == key {
                return h.val;
            }
            head = h.next.as_ref();
        }
        -1
    }

    fn remove(&mut self, key: i32) {
        let h = MyHashMap::hash(key);
        let mut head = self.buckets[h].take();

        let mut new_head = Node {
            key: -1,
            val: -1,
            next: None,
        };
        let mut new_tail = &mut new_head.next;

        while let Some(mut h) = head.take() {
            head = h.next.take();
            if h.key != key {
                let _ = new_tail.insert(h);
                new_tail = &mut new_tail.as_mut().unwrap().next;
            }
        }
        self.buckets[h] = new_head.next.take();
    }

    fn hash(key: i32) -> usize {
        key.abs() as usize % SIZE
    }
}

#[test]
fn test_hash_map() {
    let mut map = MyHashMap::new();
    map.put(1, 1);
    map.put(11, 11);
    map.put(21, 21);
    map.remove(11);
    println!("{}", map.get(21));
    map.remove(1);
    println!("{}", map.get(21));
    map.put(21, 22);
    println!("{}", map.get(21));
}
