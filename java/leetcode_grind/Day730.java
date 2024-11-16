package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Deque;

public class Day730 {
    // https://leetcode.com/problems/find-the-power-of-k-size-subarrays-i/description/?envType=daily-question&envId=2024-11-16
    static class Solution1 {
        public int[] resultsArray(int[] nums, int k) {
            int length = nums.length;
            int[] result = new int[length - k + 1];

            for (int start = 0; start <= length - k; start++) {
                boolean isConsecutiveAndSorted = true;

                for (int index = start; index < start + k - 1; index++) {
                    if (nums[index + 1] != nums[index] + 1) {
                        isConsecutiveAndSorted = false;
                        break;
                    }
                }

                if (isConsecutiveAndSorted) {
                    result[start] = nums[start + k - 1];
                } else {
                    result[start] = -1;
                }
            }

            return result;
        }
    }

    static class Solution2 {
        public int[] resultsArray(int[] nums, int k) {
            int length = nums.length;
            int[] result = new int[length - k + 1];
            Deque<Integer> indexDeque = new ArrayDeque<>();

            for (int currentIndex = 0; currentIndex < length; currentIndex++) {
                if (!indexDeque.isEmpty() && indexDeque.peekFirst() < currentIndex - k + 1) {
                    indexDeque.pollFirst();
                }

                if (!indexDeque.isEmpty() && nums[currentIndex] != nums[currentIndex - 1] + 1) {
                    indexDeque.clear();
                }

                indexDeque.offerLast(currentIndex);

                if (currentIndex >= k - 1) {
                    if (indexDeque.size() == k) {
                        result[currentIndex - k + 1] = nums[indexDeque.peekLast()];
                    } else {
                        result[currentIndex - k + 1] = -1;
                    }
                }
            }

            return result;
        }
    }

    

}
