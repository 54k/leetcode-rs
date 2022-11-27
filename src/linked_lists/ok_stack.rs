type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Link<T>,
}

#[derive(Debug)]
struct Stack<T> {
    head: Link<T>,
}

struct IntoIter<T>(Stack<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { head: None }
    }

    fn push(&mut self, val: T) {
        let head = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(head);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|n| {
            self.next = n.next.as_deref_mut();
            &mut n.val
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|n| {
            self.next = n.next.as_deref();
            &n.val
        })
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::linked_lists::ok_stack::*;

    #[test]
    fn do_test() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        println!("{:?}", stack);

        assert_eq!(stack.pop(), Some(2));
        println!("{:?}", stack);

        assert_eq!(stack.peek(), Some(&1));
        println!("{:?}", stack);

        stack.peek_mut().map(|e| *e = 42);

        assert_eq!(stack.peek(), Some(&42));
        println!("{:?}", stack);

        assert_eq!(stack.pop(), Some(42));
        println!("{:?}", stack);

        assert_eq!(stack.peek(), None);
        println!("{:?}", stack);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        for x in stack.into_iter() {
            println!("{}", x);
        }

        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        for x in stack.iter() {
            println!("{}", x);
        }

        let mut iter_mut = stack.iter_mut().fuse();
        let mut prev = iter_mut.next();
        while let Some(n) = prev {
            *n = 42;
            prev = iter_mut.next();
        }
        println!("{:?}", stack);
    }
}
