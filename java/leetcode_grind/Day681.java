package leetcode_grind;

public class Day681 {

    // https://leetcode.com/problems/design-circular-deque/description/?envType=daily-question&envId=2024-09-28
    static class Node {

        public int val;
        public Node next;
        public Node prev;

        public Node(int val, Node next, Node prev) {
            this.val = val;
            this.next = next;
            this.prev = prev;
        }
    }

    static class MyCircularDeque1 {

        Node head;
        Node rear;
        int size;
        int capacity;

        public MyCircularDeque1(int k) {
            size = 0;
            capacity = k;
        }

        public boolean insertFront(int value) {
            if (isFull())
                return false;
            if (head == null) {
                // first element in list
                head = new Node(value, null, null);
                rear = head;
            } else {
                // add new head
                Node newHead = new Node(value, head, null);
                head.prev = newHead;
                head = newHead;
            }
            size++;
            return true;
        }

        public boolean insertLast(int value) {
            if (isFull())
                return false;
            if (head == null) {
                // first element in list
                head = new Node(value, null, null);
                rear = head;
            } else {
                // add new element to end
                rear.next = new Node(value, null, rear);
                rear = rear.next;
            }
            size++;
            return true;
        }

        public boolean deleteFront() {
            if (isEmpty())
                return false;
            if (size == 1) {
                head = null;
                rear = null;
            } else {
                head = head.next;
            }
            size--;
            return true;
        }

        public boolean deleteLast() {
            if (isEmpty())
                return false;
            if (size == 1) {
                head = null;
                rear = null;
            } else {
                // update rear to the previous node
                rear = rear.prev;
            }
            size--;
            return true;
        }

        public int getFront() {
            if (isEmpty())
                return -1;
            return head.val;
        }

        public int getRear() {
            if (isEmpty())
                return -1;
            return rear.val;
        }

        public boolean isEmpty() {
            return size == 0;
        }

        public boolean isFull() {
            return size == capacity;
        }
    }

    class MyCircularDeque2 {

        int[] array;
        int front;
        int rear;
        int size;
        int capacity;

        public MyCircularDeque2(int k) {
            array = new int[k];
            size = 0;
            capacity = k;
            front = 0;
            rear = k - 1;
        }

        public boolean insertFront(int value) {
            if (isFull())
                return false;
            front = (front - 1 + capacity) % capacity;
            array[front] = value;
            size++;
            return true;
        }

        public boolean insertLast(int value) {
            if (isFull())
                return false;
            rear = (rear + 1) % capacity;
            array[rear] = value;
            size++;
            return true;
        }

        public boolean deleteFront() {
            if (isEmpty())
                return false;
            front = (front + 1) % capacity;
            size--;
            return true;
        }

        public boolean deleteLast() {
            if (isEmpty())
                return false;
            rear = (rear - 1 + capacity) % capacity;
            size--;
            return true;
        }

        public int getFront() {
            if (isEmpty())
                return -1;
            return array[front];
        }

        public int getRear() {
            if (isEmpty())
                return -1;
            return array[rear];
        }

        public boolean isEmpty() {
            return size == 0;
        }

        public boolean isFull() {
            return size == capacity;
        }
    }

    // Solution for 493. Reverse Pairs
    // https://leetcode.com/problems/reverse-pairs/description/
    static class Solution1 {
        private int count = 0;
        private int[] nums;

        public int reversePairs(int[] nums) {
            this.nums = nums;
            mergeSort(0, nums.length - 1);

            return count;
        }

        private void mergeSort(int low, int high) {
            if (low >= high)
                return;
            int mid = low + (high - low) / 2;

            mergeSort(low, mid);
            mergeSort(mid + 1, high);

            int j = mid + 1;
            for (int i = low; i <= mid; i++) {
                while (j <= high && nums[i] > (long) nums[j] * 2)
                    j++;
                count += (j - mid - 1);
            }

            merge(low, mid, high);
        }

        private void merge(int low, int mid, int high) {
            int[] helper = new int[high - low + 1];
            for (int i = low; i <= high; i++) {
                helper[i - low] = nums[i];
            }

            int i = low, j = mid + 1;
            int idx = low;

            while (i <= mid && j <= high) {
                if (helper[i - low] < helper[j - low]) {
                    nums[idx++] = helper[i++ - low];
                } else {
                    nums[idx++] = helper[j++ - low];
                }
            }

            while (i <= mid) {
                nums[idx++] = helper[i++ - low];
            }
        }
    }
}
