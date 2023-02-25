use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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

// Как реализовать стек, в котором кроме стандартных функций push и pop будет поддерживаться
// функция min, возвращающая минимальный элемент?
// Все операции должны выполняться за время О(1)
trait MinStack {
    fn push(&mut self, val: i32);
    fn pop(&mut self) -> i32;
    fn min(&self) -> i32;
}

struct VecMinStack {
    values: Vec<i32>,
    minimums: Vec<i32>,
}

impl VecMinStack {
    fn new() -> Self {
        Self {
            values: vec![],
            minimums: vec![],
        }
    }
}

impl MinStack for VecMinStack {
    fn push(&mut self, val: i32) {
        self.values.push(val);
        if self.minimums.is_empty() || *self.minimums.last().unwrap() > val {
            self.minimums.push(val);
        }
    }

    fn pop(&mut self) -> i32 {
        let val = self.values.pop().unwrap();
        if *self.minimums.last().unwrap() == val {
            self.minimums.pop();
        }
        val
    }

    fn min(&self) -> i32 {
        *self.minimums.last().unwrap()
    }
}

// Реализуйте SetOfStacks
// SetOfStack должна состоять из нескольких стеков, новый стек создается,
// как только предыдущий достигнет порогового значения.
// Методы SetOfStacks push() и pop() должны вести себя так же, как при работе с одним стеком
// (то есть метод pop() должен возвращать те же значения,
// которые бы он возвращал при использовании одного большого стека)

// Дополнительно
// Реализуйте функцию popAt(int index),
// которая осуществляет операцию pop c заданным внутренним стеком

trait SetOfStacks {
    fn push(&mut self, val: i32);
    fn pop(&mut self) -> i32;
    fn pop_at(&mut self, index: usize) -> i32;
}

struct VecSetOfStacks {
    values: Vec<Rc<RefCell<VecDeque<i32>>>>,
    capacity: usize,
}

impl VecSetOfStacks {
    fn new(capacity: usize) -> Self {
        VecSetOfStacks {
            values: vec![],
            capacity,
        }
    }

    fn last_stack(&mut self) -> Rc<RefCell<VecDeque<i32>>> {
        let last_idx = self.values.len() - 1;
        self.values[last_idx].clone()
    }

    fn left_shift(&mut self, index: usize, remove_first: bool) -> i32 {
        let stack = self.values[index].clone();
        let mut stack = stack.borrow_mut();
        let removed = if remove_first {
            stack.pop_back().unwrap()
        } else {
            stack.pop_front().unwrap()
        };

        if stack.is_empty() {
            self.values.remove(index);
        } else if stack.len() > index + 1 {
            stack.push_back(self.left_shift(index + 1, true));
        }
        removed
    }
}

impl SetOfStacks for VecSetOfStacks {
    fn push(&mut self, val: i32) {
        if self.values.is_empty() || self.values.last().unwrap().borrow().len() == self.capacity {
            self.values.push(Rc::new(RefCell::new(VecDeque::new())));
        }
        if let Some(x) = self.values.last() {
            x.borrow_mut().push_back(val)
        }
    }

    fn pop(&mut self) -> i32 {
        let stack = self.last_stack();
        let last = stack.borrow_mut().pop_back().unwrap();
        if stack.borrow().is_empty() {
            self.values.pop();
        }
        last
    }

    fn pop_at(&mut self, index: usize) -> i32 {
        self.left_shift(index, false)
    }
}

// Напишите класс MyQueue, который реализует очередь с использование двух стеков
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedStack {
    head: Option<Box<Node>>,
}

impl LinkedStack {
    fn new() -> Self {
        LinkedStack { head: None }
    }

    fn push(&mut self, value: i32) {
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take(),
        }));
    }

    fn pop(&mut self) -> i32 {
        self.head
            .take()
            .map(|x| {
                self.head = x.next;
                x.value
            })
            .unwrap()
    }

    fn peek(&self) -> i32 {
        self.head.as_ref().unwrap().value
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

struct LinkedQueue {
    left: LinkedStack,
    right: LinkedStack,
}

impl LinkedQueue {
    fn new() -> Self {
        Self {
            left: LinkedStack::new(),
            right: LinkedStack::new(),
        }
    }

    fn pop(&mut self) -> i32 {
        if self.right.is_empty() {
            while !self.left.is_empty() {
                self.right.push(self.left.pop());
            }
        }
        self.right.pop()
    }

    fn push(&mut self, value: i32) {
        self.left.push(value);
    }
}

// Напишите программу сортировки стека,
// в результате которой наименьший элемент оказывается на вершине стека.
// Вы можетет использовать дополнительный стек, но элементы не должны копироваться в другие структуры
// данных (например в массив). Стек должен поддерживать следующие операции - push, pop, peek, is_empty
fn sort_stack(s1: &mut LinkedStack) {
    let mut s2 = LinkedStack::new();
    while !s1.is_empty() {
        let tmp = s1.pop();
        while !s2.is_empty() && s2.peek() > tmp {
            s1.push(s2.pop());
        }
        s2.push(tmp);
    }
    while !s2.is_empty() {
        s1.push(s2.pop());
    }
}

// Реализуйте приют для животных
trait Shelter {
    fn dequeue(&mut self) -> Box<dyn ShelterAnimal>;
}
trait ShelterAnimal {
    fn timestamp(&self) -> i32;
    fn release(self: Box<Self>) -> Box<dyn Animal>;
}
struct ShelterAnimalImpl(Box<dyn Animal>, i32);
impl ShelterAnimal for ShelterAnimalImpl {
    fn timestamp(&self) -> i32 {
        self.1
    }

    //noinspection RsTypeCheck
    // weird shit, it compiles actually
    fn release(self: Box<Self>) -> Box<dyn Animal> {
        self.0
    }
}
trait Animal {
    fn act(&self);
}

struct Cat {
    name: String,
}
impl Animal for Cat {
    fn act(&self) {
        println!("I am a cat {}", self.name);
    }
}

struct Dog {
    name: String,
}
impl Animal for Dog {
    fn act(&self) {
        println!("I am a dog {}", self.name);
    }
}

struct CatDogShelter {
    cats: VecDeque<Box<dyn ShelterAnimal>>,
    dogs: VecDeque<Box<dyn ShelterAnimal>>,
    ts_seq: i32,
}

impl CatDogShelter {
    fn new() -> Self {
        Self {
            cats: VecDeque::new(),
            dogs: VecDeque::new(),
            ts_seq: 0,
        }
    }
    fn enqueue_dog(&mut self, animal: Box<dyn Animal>) {
        self.cats
            .push_back(Box::new(ShelterAnimalImpl(animal, self.ts_seq)));
        self.ts_seq += 1;
    }
    fn enqueue_cat(&mut self, animal: Box<dyn Animal>) {
        self.dogs
            .push_back(Box::new(ShelterAnimalImpl(animal, self.ts_seq)));
        self.ts_seq += 1;
    }
}

impl Shelter for CatDogShelter {
    fn dequeue(&mut self) -> Box<dyn ShelterAnimal> {
        if self.cats.is_empty() {
            self.dogs.pop_front()
        } else if self.dogs.is_empty()
            || self.cats.front().unwrap().timestamp() < self.dogs.front().unwrap().timestamp()
        {
            self.cats.pop_front()
        } else {
            self.dogs.pop_front()
        }
        .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::panic::set_hook;

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

    #[test]
    fn test_min_stack() {
        let mut stack = VecMinStack::new();
        stack.push(5);
        stack.push(6);
        assert_eq!(stack.min(), 5);
        stack.push(3);
        stack.push(7);
        assert_eq!(stack.min(), 3);
        stack.pop(); // 3
        assert_eq!(stack.min(), 3);
        stack.pop(); // 5
        assert_eq!(stack.min(), 5);
    }

    #[test]
    fn test_set_of_stacks() {
        let mut stack = VecSetOfStacks::new(2);
        stack.push(1);
        stack.push(1);
        stack.push(2);

        assert_eq!(stack.values.len(), 2);

        stack.push(2);

        assert_eq!(stack.pop_at(0), 1);

        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);

        assert!(stack.values.is_empty());
    }

    #[test]
    fn test_queue_with_two_stacks() {
        let mut queue = LinkedQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.pop(), 1);
        queue.push(3);
        assert_eq!(queue.pop(), 2);
        queue.push(4);
        assert_eq!(queue.pop(), 3);
        assert_eq!(queue.pop(), 4);
    }

    #[test]
    fn test_sort_stack() {
        let mut stack = LinkedStack::new();
        stack.push(3);
        stack.push(1);
        stack.push(0);
        stack.push(2);
        sort_stack(&mut stack);
        assert_eq!(stack.pop(), 0);
        assert_eq!(stack.pop(), 1);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 3);
    }

    #[test]
    fn test_animal_shelter() {
        let mut shelter = CatDogShelter::new();
        shelter.enqueue_cat(Box::new(Cat {
            name: "Meow".to_string(),
        }));
        shelter.enqueue_dog(Box::new(Dog {
            name: "Bark".to_string(),
        }));
        shelter.enqueue_dog(Box::new(Dog {
            name: "Dark".to_string(),
        }));
        shelter.enqueue_cat(Box::new(Cat {
            name: "Pus in boots".to_string(),
        }));
        shelter.dequeue().release().act();
        shelter.dequeue().release().act();
        shelter.dequeue().release().act();
        shelter.dequeue().release().act();
    }
}
