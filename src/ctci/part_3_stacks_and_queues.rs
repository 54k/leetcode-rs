// Опишите как можно использовать один одномерный массив для реализации трех стеков
pub trait MultiStack {
    fn push(&mut self, stack_num: usize, val: i32);
    fn pop(&mut self, stack_num: usize) -> i32;
    fn peek(&self, stack_num: usize) -> i32;
    fn is_empty(&self, stack_num: usize) -> bool;
    fn is_full(&self, stack_num: usize) -> bool;
}

#[derive(Debug)]
pub struct FixedMultiStack {
    stack_size: usize,
    values: Vec<i32>,
    sizes: Vec<i32>,
}

impl FixedMultiStack {
    pub fn new(stack_size: usize, num_of_stacks: usize) -> Self {
        Self {
            values: vec![0; stack_size * num_of_stacks],
            sizes: vec![0; num_of_stacks],
            stack_size,
        }
    }
}

impl FixedMultiStack {
    fn index_of_top(&self, stack_num: usize) -> i32 {
        let offset = self.stack_size * stack_num;
        self.sizes[stack_num] - 1 + offset as i32
    }
}

impl MultiStack for FixedMultiStack {
    fn push(&mut self, stack_num: usize, val: i32) {
        let i = self.index_of_top(stack_num);
        self.values[(i + 1) as usize] = val;
        self.sizes[stack_num] += 1;
    }

    fn pop(&mut self, stack_num: usize) -> i32 {
        let i = self.index_of_top(stack_num);
        self.sizes[stack_num] -= 1;
        self.values[i as usize]
    }

    fn peek(&self, stack_num: usize) -> i32 {
        self.values[self.index_of_top(stack_num) as usize]
    }

    fn is_empty(&self, stack_num: usize) -> bool {
        self.sizes[stack_num] == 0
    }

    fn is_full(&self, stack_num: usize) -> bool {
        self.sizes[stack_num] as usize == self.stack_size
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fixed_multistack() {
        let mut stack = FixedMultiStack::new(2, 3);
        stack.push(0, 1);
        stack.push(1, 1);
        stack.push(2, 1);

        stack.push(0, 2);
        stack.push(1, 2);
        stack.push(2, 2);

        assert!(stack.is_full(0));
        assert!(stack.is_full(1));
        assert!(stack.is_full(2));

        assert_eq!(stack.pop(0), 2);
        assert_eq!(stack.pop(0), 1);

        assert_eq!(stack.pop(1), 2);
        assert_eq!(stack.pop(1), 1);

        assert_eq!(stack.pop(2), 2);
        assert_eq!(stack.pop(2), 1);

        assert!(stack.is_empty(0));
        assert!(stack.is_empty(1));
        assert!(stack.is_empty(2));
    }
}
