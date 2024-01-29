package leetcode_grind;

import java.util.Stack;

public class Day443 {
    // https://leetcode.com/problems/implement-queue-using-stacks/description
    class MyQueue {
        Stack<Integer> put = new Stack<>();
        Stack<Integer> get = new Stack<>();

        public MyQueue() {
        }

        public void push(int x) {
            put.push(x);
        }

        public int pop() {
            if (get.isEmpty()) {
                while (!put.isEmpty()) {
                    get.push(put.pop());
                }
            }
            return get.pop();
        }

        public int peek() {
            if (get.isEmpty()) {
                while (!put.isEmpty()) {
                    get.push(put.pop());
                }
            }
            return get.peek();
        }

        public boolean empty() {
            return put.isEmpty() && get.isEmpty();
        }
    }

}
