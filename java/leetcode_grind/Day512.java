package leetcode_grind;

import java.util.LinkedList;
import java.util.Queue;
import java.util.Stack;

public class Day512 {
    // https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/description
    static class Solution1 {
        public int countStudents(int[] students, int[] sandwiches) {
            int len = students.length;
            Queue<Integer> studentQueue = new LinkedList<>();
            Stack<Integer> sandwichStack = new Stack<>();

            for (int i = 0; i < len; i++) {
                sandwichStack.push(sandwiches[len - i - 1]);
                studentQueue.offer(students[i]);
            }

            int lastServed = 0;
            while (studentQueue.size() > 0 && lastServed < studentQueue.size()) {
                if (sandwichStack.peek() == studentQueue.peek()) {
                    sandwichStack.pop();
                    studentQueue.poll();
                    lastServed = 0;
                } else {
                    studentQueue.offer(studentQueue.poll());
                    lastServed++;
                }
            }

            return studentQueue.size();
        }
    }

    // https://leetcode.com/problems/verify-preorder-sequence-in-binary-search-tree/description
    static class Solution2 {
        public boolean verifyPreorder(int[] preorder) {
            int minLimit = Integer.MIN_VALUE;
            Stack<Integer> stack = new Stack<>();

            for (int num : preorder) {
                while (!stack.isEmpty() && stack.peek() < num) {
                    minLimit = stack.pop();
                }

                if (num <= minLimit) {
                    return false;
                }

                stack.push(num);
            }

            return true;
        }
    }
}
