// Опишите как можно использовать один одномерный массив для реализации трех стеков
fn task_2_3() {
    trait MultiStack {
        fn push(&mut self, stack_num: i32, val: i32);
        fn pop(&mut self, stack_num: i32) -> i32;
        fn peek(&self, stack_num: i32) -> i32;
        fn is_empty(&self, stack_num: i32) -> bool;
        fn is_full(&self, stack_num: i32) -> bool;
    }

    const NUM_OF_STACKS: usize = 3;

    #[derive(Debug)]
    struct FixedMultiStack {
        values: Vec<i32>,
        stack_size: usize,
        sizes: Vec<usize>,
    }

    impl FixedMultiStack {
        fn new(stack_size: usize) -> Self {
            Self {
                values: vec![0; stack_size * NUM_OF_STACKS],
                sizes: vec![0; NUM_OF_STACKS],
                stack_size,
            }
        }
    }

    impl MultiStack for FixedMultiStack {
        fn push(&mut self, stack_num: i32, val: i32) {
            todo!()
        }

        fn pop(&mut self, stack_num: i32) -> i32 {
            todo!()
        }

        fn peek(&self, stack_num: i32) -> i32 {
            todo!()
        }

        fn is_empty(&self, stack_num: i32) -> bool {
            todo!()
        }

        fn is_full(&self, stack_num: i32) -> bool {
            todo!()
        }
    }
}
