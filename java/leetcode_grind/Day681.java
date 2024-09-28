package leetcode_grind;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

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

    // https://leetcode.com/problems/count-of-smaller-numbers-after-self/description/
    static class Solution2 {
        public List<Integer> countSmaller(int[] nums) {
            int offset = 10000;
            int size = 2 * 10000 + 1;
            int[] tree = new int[size * 2];
            List<Integer> result = new ArrayList<Integer>();

            for (int i = nums.length - 1; i >= 0; i--) {
                int smaller_count = query(0, nums[i] + offset, tree, size);
                result.add(smaller_count);
                update(nums[i] + offset, 1, tree, size);
            }
            Collections.reverse(result);
            return result;
        }

        void update(int index, int value, int[] tree, int size) {
            index += size;
            tree[index] += value;
            while (index > 1) {
                index /= 2;
                tree[index] = tree[index * 2] + tree[index * 2 + 1];
            }
        }

        int query(int left, int right, int[] tree, int size) {
            int result = 0;
            left += size;
            right += size;
            while (left < right) {
                if (left % 2 == 1) {
                    result += tree[left];
                    left++;
                }
                left /= 2;
                if (right % 2 == 1) {
                    right--;
                    result += tree[right];
                }
                right /= 2;
            }
            return result;
        }
    }

    static class Solution3 {
        public List<Integer> countSmaller(int[] nums) {
            int offset = 10000;
            int size = 2 * 10000 + 2;
            int[] tree = new int[size];
            List<Integer> result = new ArrayList<Integer>();

            for (int i = nums.length - 1; i >= 0; i--) {
                int smaller_count = query(nums[i] + offset, tree);
                result.add(smaller_count);
                update(nums[i] + offset, 1, tree, size);
            }
            Collections.reverse(result);
            return result;
        }

        void update(int index, int value, int[] tree, int size) {
            index++;
            while (index < size) {
                tree[index] += value;
                index += index & -index;
            }
        }

        int query(int index, int[] tree) {
            int result = 0;
            while (index >= 1) {
                result += tree[index];
                index -= index & -index;
            }
            return result;
        }
    }

    static class Solution4 {
        public List<Integer> countSmaller(int[] nums) {
            int n = nums.length;
            int[] result = new int[n];
            int[] indices = new int[n];
            for (int i = 0; i < n; i++) {
                indices[i] = i;
            }
            mergeSort(indices, 0, n, result, nums);
            List<Integer> resultToReturn = new ArrayList<Integer>(n);
            for (int i : result) {
                resultToReturn.add(i);
            }
            return resultToReturn;
        }

        void mergeSort(int[] indices, int left, int right, int[] result, int[] nums) {
            if (right - left <= 1) {
                return;
            }
            int mid = (left + right) / 2;
            mergeSort(indices, left, mid, result, nums);
            mergeSort(indices, mid, right, result, nums);
            merge(indices, left, right, mid, result, nums);
        }

        void merge(int[] indices, int left, int right, int mid, int[] result, int[] nums) {
            int i = left;
            int j = mid;
            List<Integer> temp = new ArrayList<Integer>(right - left);
            while (i < mid && j < right) {
                if (nums[indices[i]] <= nums[indices[j]]) {
                    result[indices[i]] += j - mid;
                    temp.add(indices[i]);
                    i++;
                } else {
                    temp.add(indices[j]);
                    j++;
                }
            }
            while (i < mid) {
                result[indices[i]] += j - mid;
                temp.add(indices[i]);
                i++;
            }
            while (j < right) {
                temp.add(indices[j]);
                j++;
            }
            for (int k = left; k < right; k++) {
                indices[k] = temp.get(k - left);
            }
        }
    }

    // https://leetcode.com/problems/count-pairs-in-two-arrays/description/
    static class Solution5 {
        public long countPairs(int[] nums1, int[] nums2) {
            int N = nums1.length;
            long[] difference = new long[N];
            for (int i = 0; i < N; i++) {
                difference[i] = nums1[i] - nums2[i];
            }
            Arrays.sort(difference);

            long result = 0;
            for (int i = 0; i < N; i++) {
                if (difference[i] > 0) {
                    result += N - i - 1;
                } else {
                    int left = i + 1;
                    int right = N - 1;
                    while (left <= right) {
                        int mid = left + (right - left) / 2;
                        if (difference[i] + difference[mid] > 0) {
                            right = mid - 1;
                        } else {
                            left = mid + 1;
                        }
                    }
                    result += N - left;
                }
            }
            return result;
        }
    }

    // https://leetcode.com/problems/find-the-lexicographically-smallest-valid-sequence/description/
    static class Solution6 {
        public int[] validSequence(String word1, String word2) {
            int length1 = word1.length();
            int length2 = word2.length();
            int[] indices = new int[length1];
            int j = length2 - 1;

            for (int i = length1 - 1; i >= 0; i--) {
                if (j >= 0 && word1.charAt(i) == word2.charAt(j)) {
                    indices[i] = (i == length1 - 1) ? 1 : indices[i + 1] + 1;
                    j--;
                } else {
                    indices[i] = (i == length1 - 1) ? 0 : indices[i + 1];
                }
            }

            j = 0;
            List<Integer> result = new ArrayList<>();
            int finalIndex = -1;

            for (int i = 0; i < length1; i++) {
                if (word1.charAt(i) == word2.charAt(j)) {
                    result.add(i);
                    j++;
                    if (j == length2) {
                        break;
                    }
                } else {
                    if ((i == length1 - 1 ? 0 : indices[i + 1]) >= length2 - j - 1) {
                        result.add(i);
                        j++;
                        finalIndex = i + 1;
                        break;
                    }
                }
            }

            if (result.size() == length2) {
                return convertToArray(result);
            }

            if (finalIndex == -1) {
                return new int[0];
            }

            for (int i = finalIndex; i < length1; i++) {
                if (word1.charAt(i) == word2.charAt(j)) {
                    result.add(i);
                    j++;
                }
                if (j == length2) {
                    break;
                }
            }

            return result.size() == length2 ? convertToArray(result) : new int[0];
        }

        int[] convertToArray(List<Integer> list) {
            return list.stream().mapToInt(Integer::intValue).toArray();
        }
    }

    // https://leetcode.com/problems/find-the-occurrence-of-first-almost-equal-substring/description/
    static class Solution7 {

        int[] z_function(String s) {
            int n = s.length(), l = 0, r = 0;
            int[] z = new int[n];
            for (int i = 1; i < n; i++) {
                if (i <= r) {
                    z[i] = Math.min(r - i + 1, z[i - l]);
                }
                while (i + z[i] < n && s.charAt(z[i]) == s.charAt(i + z[i])) {
                    ++z[i];
                }
                if (i + z[i] - 1 > r) {
                    l = i;
                    r = i + z[i] - 1;
                }
            }
            return z;
        }

        public int minStartingIndex(String s, String pattern) {
            int n = s.length(), m = pattern.length();
            int[] z1 = z_function(pattern + s);
            int[] z2 = z_function(new StringBuilder(s + pattern).reverse().toString());
            for (int i = 0; i <= n - m; ++i)
                if (z1[m + i] + 1 + z2[n - i] >= m)
                    return i;
            return -1;
        }
    }
}
