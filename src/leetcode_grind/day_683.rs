mod cstack1 {
    struct CustomStack {
        stack_array: Vec<i32>,
        increment_array: Vec<i32>,
        top_index: i32,
    }

    impl CustomStack {
        fn new(maxSize: i32) -> Self {
            let max_size = maxSize as usize;
            Self {
                stack_array: vec![0; max_size],
                increment_array: vec![0; max_size],
                top_index: -1,
            }
        }

        fn push(&mut self, x: i32) {
            if self.top_index < self.stack_array.len() as i32 - 1 {
                self.top_index += 1;
                self.stack_array[self.top_index as usize] = x;
            }
        }

        fn pop(&mut self) -> i32 {
            if self.top_index < 0 {
                return -1;
            }
            let result = self.stack_array[self.top_index as usize]
                + self.increment_array[self.top_index as usize];
            if self.top_index > 0 {
                self.increment_array[self.top_index as usize - 1] +=
                    self.increment_array[self.top_index as usize];
            }
            self.increment_array[self.top_index as usize] = 0;
            self.top_index -= 1;
            result
        }

        fn increment(&mut self, k: i32, val: i32) {
            if self.top_index >= 0 {
                let increment_index = self.top_index.min(k - 1);
                self.increment_array[increment_index as usize] += val;
            }
        }
    }
}
