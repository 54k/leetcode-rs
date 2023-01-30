pub fn tribonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n <= 2 {
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    for _ in 3..=n {
        let d = a + b + c;
        a = b;
        b = c;
        c = d;
    }
    c
}

use std::collections::*;
use std::ptr::null_mut;

struct Node {
    key: i32,
    val: i32,
    prev: *mut Node,
    next: *mut Node,
}

struct Dll {
    head: *mut Node,
    tail: *mut Node,
}

impl Dll {
    fn new() -> Self {
        Self {
            head: null_mut(),
            tail: null_mut(),
        }
    }

    fn push_back(&mut self, key: i32, val: i32) -> *mut Node {
        let new_node = Box::new(Node {
            key,
            val,
            prev: null_mut(),
            next: null_mut(),
        });
        let raw = Box::into_raw(new_node);
        if self.head.is_null() {
            self.head = raw;
        } else {
            unsafe {
                (*raw).prev = self.tail;
                (*self.tail).next = raw;
            }
        }
        self.tail = raw;
        raw
    }

    fn pop_front(&mut self) -> *mut Node {
        if self.head.is_null() {
            return null_mut();
        }
        if self.head == self.tail {
            let old_head = self.head;
            self.head = null_mut();
            self.tail = null_mut();
            return old_head;
        }
        unsafe {
            let old_head = self.head;
            self.head = (*old_head).next;
            (*self.head).prev = null_mut();
            old_head
        }
    }

    fn pop_back(&mut self) {
        if self.head.is_null() {
            return;
        }
        if self.head == self.tail {
            self.head = null_mut();
            self.tail = null_mut();
            return;
        }
        unsafe {
            let old_tail = self.tail;
            self.tail = (*old_tail).prev;
            (*self.tail).next = null_mut();
        }
    }

    fn evict(&mut self, node: *mut Node) -> i32 {
        if node == self.head {
            self.pop_front();
            unsafe { (*node).val }
        } else if node == self.tail {
            self.pop_back();
            unsafe { (*node).val }
        } else {
            unsafe {
                let boxed = Box::from_raw(node);
                (*boxed.prev).next = boxed.next;
                (*boxed.next).prev = boxed.prev;
                boxed.val
            }
        }
    }
}

struct LRUCache {
    node_map: HashMap<i32, *mut Node>,
    dll: Dll,
    capacity: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            node_map: HashMap::new(),
            dll: Dll::new(),
            capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if !self.node_map.contains_key(&key) {
            return -1;
        }
        unsafe {
            let node = self.node_map.remove(&key).unwrap();
            let val = self.dll.evict(node);
            let new_node = self.dll.push_back(key, val);
            self.node_map.insert(key, new_node);
            val
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if !self.node_map.contains_key(&key) && self.capacity as usize == self.node_map.len() {
            unsafe {
                let x = Box::from_raw(self.dll.pop_front());
                self.node_map.remove(&x.key).unwrap();
            }
        }

        if let hash_map::Entry::Vacant(e) = self.node_map.entry(key) {
            let node = self.dll.push_back(key, value);
            e.insert(node);
        } else {
            unsafe {
                (**self.node_map.get_mut(&key).unwrap()).val = value;
            }
            self.get(key);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test181() {
        println!("{}", tribonacci(25)); //1389537
    }

    #[test]
    fn test182() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        println!("{}", cache.get(1)); // 1
        cache.put(3, 3);
        println!("{}", cache.get(2)); // -1
        cache.put(4, 4);
        println!("{}", cache.get(1)); // -1
        println!("{}", cache.get(3)); // 3
        println!("{}", cache.get(4)); // 4

        println!("******************");
        let mut cache = LRUCache::new(2);
        cache.put(2, 1);
        cache.put(2, 2);
        println!("{}", cache.get(2)); // 2
        cache.put(1, 1);
        cache.put(4, 1);
        println!("{}", cache.get(2)); // -1

        println!("******************");
        let mut cache = LRUCache::new(2);
        cache.put(2, 1);
        cache.put(1, 1);
        cache.put(2, 3);
        cache.put(4, 1);
        println!("{}", cache.get(1)); // -1
        println!("{}", cache.get(2)); // 3

        println!("******************");
        let mut cache = LRUCache::new(2);
        println!("{}", cache.get(2)); // -1
        cache.put(2, 6);
        println!("{}", cache.get(1)); // -1
        cache.put(1, 5);
        cache.put(1, 2);
        println!("{}", cache.get(1)); // 2
        println!("{}", cache.get(2)); // 6
    }
}
