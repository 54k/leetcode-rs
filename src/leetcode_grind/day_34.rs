#[derive(Debug)]
struct Stack {
    head: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Stack {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, val: i32) {
        let mut node = Node { val, next: None };
        if let Some(head) = self.head.take() {
            node.next = Some(head);
        }
        self.head = Some(Box::new(node));
    }

    fn pop(&mut self) -> i32 {
        self.head
            .take()
            .map(|head| {
                self.head = head.next;
                head.val
            })
            .unwrap()
    }

    fn peek(&self) -> i32 {
        self.head.as_ref().map(|n| n.val).unwrap()
    }

    fn empty(&self) -> bool {
        self.head.is_none()
    }
}

#[derive(Debug)]
struct MyQueue {
    first: Stack,
    second: Stack,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            first: Stack::new(),
            second: Stack::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.first.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.second.empty() {
            self.drain();
        }
        self.second.pop()
    }

    fn peek(&mut self) -> i32 {
        if self.second.empty() {
            self.drain();
        }
        self.second.peek()
    }

    fn empty(&self) -> bool {
        self.first.empty() && self.second.empty()
    }

    fn drain(&mut self) {
        while !self.first.empty() {
            self.second.push(self.first.pop());
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day_34::*;

    #[test]
    fn test118() {
        let mut my_queue = MyQueue::new();
        my_queue.push(1); // queue is: [1]
        my_queue.push(2);
        println!("{:?}", my_queue); // queue is: [1, 2] (leftmost is front of the queue)
        println!("{}", my_queue.peek()); // return 1
        println!("{}", my_queue.pop()); // return 1, queue is [2]
        println!("{}", my_queue.empty()); // return false
        println!("{:?}", my_queue);
    }
}
