package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day820 {
    // https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/description/?envType=daily-question&envId=2025-02-16
    static class Solution1 {
        public int[] constructDistancedSequence(int n) {
            int[] resultSequence = new int[n * 2 - 1];
            boolean[] isNumberUsed = new boolean[n + 1];
            findLexicographicallyLargestSequence(0, resultSequence, isNumberUsed, n);
            return resultSequence;
        }

        boolean findLexicographicallyLargestSequence(
                int currentIndex,
                int[] resultSequence,
                boolean[] isNumberUsed,
                int targetNumber) {
            if (currentIndex == resultSequence.length) {
                return true;
            }

            if (resultSequence[currentIndex] != 0) {
                return findLexicographicallyLargestSequence(currentIndex + 1, resultSequence, isNumberUsed,
                        targetNumber);
            }

            for (int numberToPlace = targetNumber; numberToPlace >= 1; numberToPlace--) {
                if (isNumberUsed[numberToPlace])
                    continue;

                isNumberUsed[numberToPlace] = true;
                resultSequence[currentIndex] = numberToPlace;

                if (numberToPlace == 1) {
                    if (findLexicographicallyLargestSequence(currentIndex + 1, resultSequence, isNumberUsed,
                            targetNumber)) {
                        return true;
                    }
                } else if (currentIndex + numberToPlace < resultSequence.length
                        && resultSequence[currentIndex + numberToPlace] == 0) {
                    resultSequence[currentIndex + numberToPlace] = numberToPlace;

                    if (findLexicographicallyLargestSequence(currentIndex + 1, resultSequence, isNumberUsed,
                            targetNumber)) {
                        return true;
                    }

                    resultSequence[currentIndex + numberToPlace] = 0;
                }
                resultSequence[currentIndex] = 0;
                isNumberUsed[numberToPlace] = false;
            }
            return false;
        }
    }

    // https://leetcode.com/problems/design-most-recently-used-queue/description/?envType=weekly-question&envId=2025-02-15
    static class MRUQueue1 {
        List<Integer> queue = new ArrayList<>();

        public MRUQueue1(int n) {
            for (int number = 1; number <= n; number++) {
                queue.add(number);
            }
        }

        public int fetch(int k) {
            int value = queue.get(k - 1);
            queue.remove(k - 1);
            queue.add(value);
            return value;
        }
    }

    static class MRUQueue2 {
        static class ListNode {
            int val;
            ListNode next;

            ListNode() {
            }

            ListNode(int val) {
                this.val = val;
            }

            ListNode(int val, ListNode next) {
                this.val = val;
                this.next = next;
            }
        }

        ListNode head;
        ListNode tail;

        public MRUQueue2(int n) {
            this.head = new ListNode(0, null);
            ListNode current = head;
            for (int number = 1; number <= n; ++number) {
                current.next = new ListNode(number, null);
                current = current.next;
            }
            tail = current;
        }

        public int fetch(int k) {
            ListNode current = head;
            for (int index = 1; index < k; index++) {
                current = current.next;
            }
            int value = current.next.val;

            tail.next = current.next;
            tail = tail.next;
            current.next = tail.next;
            tail.next = null;
            return value;
        }
    }

    static class MRUQueue3 {
        int totalElements;
        int BUCKET_SIZE;
        List<List<Integer>> data = new ArrayList<>();
        List<Integer> index = new ArrayList<>();

        public MRUQueue3(int n) {
            totalElements = n;
            BUCKET_SIZE = (int) Math.sqrt(n);
            for (int number = 1; number <= n; number++) {
                int bucketIndex = (number - 1) / BUCKET_SIZE;
                if (bucketIndex == data.size()) {
                    data.add(new ArrayList<>());
                    index.add(number);
                }
                data.get(data.size() - 1).add(number);
            }
        }

        public int fetch(int k) {
            int bucketIndex = upperBound(index, k) - 1;
            int element = data.get(bucketIndex).get(k - index.get(bucketIndex));
            data.get(bucketIndex).remove(k - index.get(bucketIndex));

            for (int i = bucketIndex + 1; i < index.size(); i++) {
                index.set(i, index.get(i) - 1);
            }

            if (data.get(data.size() - 1).size() >= BUCKET_SIZE) {
                data.add(new ArrayList<>());
                index.add(totalElements);
            }
            data.get(data.size() - 1).add(element);

            if (data.get(bucketIndex).isEmpty()) {
                data.remove(bucketIndex);
                index.remove(bucketIndex);
            }
            return element;
        }

        int upperBound(List<Integer> nums, int target) {
            int left = 0, right = nums.size();
            while (left < right) {
                int mid = (left + right) / 2;
                if (nums.get(mid) > target) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            return left;
        }
    }

    static class MRUQueue4 {
        static class FenwickTree {
            int[] tree;

            FenwickTree(int size) {
                this.tree = new int[size + 1];
            }

            int sum(int index) {
                int result = 0;
                while (index > 0) {
                    result += tree[index];
                    index = index & (index - 1);
                }
                return result;
            }

            void insert(int index, int value) {
                index += 1;
                while (index < tree.length) {
                    tree[index] += value;
                    index += index & -index;
                }
            }
        }

        int size;
        FenwickTree tree;
        int[] values;

        public MRUQueue4(int n) {
            size = n;
            tree = new FenwickTree(n + 2000);
            values = new int[n + 2000];
            for (int i = 0; i < n; i++) {
                tree.insert(i, 1);
                values[i] = i + 1;
            }
        }

        public int fetch(int k) {
            int low = 0, high = size;
            while (low < high) {
                int mid = (low + high) >> 1;
                if (tree.sum(mid) < k) {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }

            tree.insert(low - 1, -1);
            tree.insert(size, 1);
            values[size] = values[low - 1];
            size++;
            return values[low - 1];
        }
    }

}
