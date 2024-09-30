package leetcode_grind;

public class Day683 {
    // https://leetcode.com/problems/design-a-stack-with-increment-operation/description/?envType=daily-question&envId=2024-09-30
    static class CustomStack1 {
        int[] stackArray;
        int topIndex;

        public CustomStack1(int maxSize) {
            stackArray = new int[maxSize];
            topIndex = -1;
        }

        public void push(int x) {
            if (topIndex < stackArray.length - 1) {
                stackArray[++topIndex] = x;
            }
        }

        public int pop() {
            if (topIndex >= 0) {
                return stackArray[topIndex--];
            }
            return -1;
        }

        public void increment(int k, int val) {
            int limit = Math.min(k, topIndex + 1);
            for (int i = 0; i < limit; i++) {
                stackArray[i] += val;
            }
        }
    }

    static class CustomStack2 {

        List<Integer> stack;
        int maxSize;

        public CustomStack2(int maxSize) {
            stack = new LinkedList<>();
            this.maxSize = maxSize;
        }

        public void push(int x) {
            if (stack.size() < maxSize) {
                stack.addLast(x);
            }
        }

        public int pop() {
            if (stack.isEmpty())
                return -1;
            return stack.removeLast();
        }

        public void increment(int k, int val) {
            ListIterator<Integer> iterator = stack.listIterator();
            while (iterator.hasNext() && k > 0) {
                int current = iterator.next();
                iterator.set(current + val);
                k--;
            }
        }
    }
}
