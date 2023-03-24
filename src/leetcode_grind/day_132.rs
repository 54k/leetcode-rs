// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/
// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/editorial/
pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    fn make_adj(n: i32, connections: &Vec<Vec<i32>>) -> Vec<Vec<(usize, i32)>> {
        let mut adj = vec![vec![]; n as usize];
        for c in connections {
            let from = c[0] as usize;
            let to = c[1] as usize;
            adj[from].push((to, 1));
            adj[to].push((from, 0));
        }
        adj
    }
    fn using_dfs(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        fn dfs(v: usize, parent: usize, adj: &Vec<Vec<(usize, i32)>>, count: &mut i32) {
            for (u, sign) in &adj[v] {
                if *u != parent {
                    *count += sign;
                    dfs(*u, v, adj, count);
                }
            }
        }
        let adj = make_adj(n, &connections);
        let mut count = 0;
        dfs(0, 0, &adj, &mut count);
        count
    }
    fn using_bfs(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let mut count = 0;
        let mut visited = vec![false; n as usize];
        let adj = make_adj(n, &connections);
        let mut queue = VecDeque::new();
        queue.push_back(0);
        visited[0] = true;
        while let Some(v) = queue.pop_front() {
            for (u, sign) in &adj[v] {
                if !visited[*u] {
                    visited[*u] = true;
                    count += *sign;
                    queue.push_back(*u);
                }
            }
        }
        count
    }
    using_bfs(n, connections)
}

// https://leetcode.com/problems/smallest-missing-non-negative-integer-after-operations/description/
// https://leetcode.com/problems/smallest-missing-non-negative-integer-after-operations/solutions/3314226/java-c-python-count-remainders/
pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
    let mut remainders = vec![0; value as usize + 1];
    for i in 0..nums.len() {
        let rem = ((nums[i] % value + value) % value) as usize;
        remainders[rem] += 1;
    }
    let mut stop = 0;
    for i in 0..value as usize {
        if remainders[i] < remainders[stop] {
            stop = i;
        }
    }
    value * remainders[stop] + stop as i32
}

// https://leetcode.com/problems/design-circular-queue/
struct MyCircularQueue {
    buf: Vec<i32>,
    start: i32,
    end: i32,
    capacity: i32,
    size: i32,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            buf: vec![0; k as usize],
            start: 0,
            end: 0,
            capacity: k,
            size: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.buf[self.end as usize] = value;
        self.end = (self.end + 1) % self.capacity;
        self.size += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.start = (self.start + 1) % self.capacity;
        self.size -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.buf[self.start as usize]
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let i = ((self.end - 1) % self.capacity + self.capacity) % self.capacity;
        self.buf[i as usize]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

// https://leetcode.com/problems/running-sum-of-1d-array/
// https://leetcode.com/problems/running-sum-of-1d-array/editorial/
pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }
    nums
}

// https://leetcode.com/problems/design-circular-deque/
struct MyCircularDeque {
    start: i32,
    end: i32,
    capacity: i32,
    size: i32,
    buf: Vec<i32>,
}
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            start: 0,
            end: 0,
            capacity: k,
            size: 0,
            buf: vec![0; k as usize],
        }
    }
    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.start = ((self.start - 1) % self.capacity + self.capacity) % self.capacity;
        self.buf[self.start as usize] = value;
        self.size += 1;
        true
    }
    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.buf[self.end as usize] = value;
        self.end = (self.end + 1) % self.capacity;
        self.size += 1;
        true
    }
    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.start = (self.start + 1) % self.capacity;
        self.size -= 1;
        true
    }
    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.end = ((self.end - 1) % self.capacity + self.capacity) % self.capacity;
        self.size -= 1;
        true
    }
    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.buf[self.start as usize]
    }
    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let i = ((self.end - 1) % self.capacity + self.capacity) % self.capacity;
        self.buf[i as usize]
    }
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test369() {
        println!(
            "{}",
            min_reorder(
                6,
                vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]
            )
        ); // 3
    }

    #[test]
    fn test370() {
        println!("{}", find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 5)); // 4
        println!("{}", find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 7)); // 2
    }

    #[test]
    fn test371() {
        // MyCircularQueue myCircularQueue = new MyCircularQueue(3);
        // myCircularQueue.enQueue(1); // return True
        // myCircularQueue.enQueue(2); // return True
        // myCircularQueue.enQueue(3); // return True
        // myCircularQueue.enQueue(4); // return False
        // myCircularQueue.Rear();     // return 3
        // myCircularQueue.isFull();   // return True
        // myCircularQueue.deQueue();  // return True
        // myCircularQueue.enQueue(4); // return True
        // myCircularQueue.Rear();     // return 4

        let mut queue = MyCircularQueue::new(3);
        println!("{}", queue.en_queue(1)); // true
        println!("{}", queue.en_queue(2)); // true
        println!("{}", queue.en_queue(3)); // true
        println!("{}", queue.en_queue(4)); // false
        println!("{}", queue.rear()); // 3
        println!("{}", queue.is_full()); // true
        println!("{}", queue.de_queue()); // true
        println!("{}", queue.en_queue(4)); // true
        println!("{}", queue.rear()); // 4

        let mut queue = MyCircularQueue::new(6);
        println!("{}", queue.en_queue(6)); // true
        println!("{}", queue.rear()); // 6
        println!("{}", queue.rear()); // 6
        println!("{}", queue.de_queue()); // true
        println!("{}", queue.en_queue(5)); // false
        println!("{}", queue.rear()); // 4
        println!("{}", queue.de_queue()); // true
        println!("{}", queue.front()); // -1
        println!("{}", queue.de_queue()); // false
        println!("{}", queue.de_queue()); // false
        println!("{}", queue.de_queue()); // false
    }

    #[test]
    fn test372() {
        println!("{:?}", running_sum(vec![1, 2, 3, 4])); // [1,3,6,10]
    }

    #[test]
    fn test373() {
        // MyCircularDeque myCircularDeque = new MyCircularDeque(3);
        // myCircularDeque.insertLast(1);  // return True
        // myCircularDeque.insertLast(2);  // return True
        // myCircularDeque.insertFront(3); // return True
        // myCircularDeque.insertFront(4); // return False, the queue is full.
        // myCircularDeque.getRear();      // return 2
        // myCircularDeque.isFull();       // return True
        // myCircularDeque.deleteLast();   // return True
        // myCircularDeque.insertFront(4); // return True
        // myCircularDeque.getFront();     // return 4
        let mut deque = MyCircularDeque::new(3);
        println!("{}", deque.insert_last(1)); // true
        println!("{}", deque.insert_last(2)); // true
        println!("{}", deque.insert_front(3)); // true
        println!("{}", deque.insert_last(4)); // false
        println!("{}", deque.get_rear()); // 2
        println!("{}", deque.get_front()); // 3
        println!("{}", deque.is_full()); // true
        println!("{}", deque.delete_last()); // true
        println!("{}", deque.insert_front(4)); // true
        println!("{}", deque.get_front()); // 4
    }
}
