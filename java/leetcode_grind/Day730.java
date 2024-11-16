package leetcode_grind;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.HashMap;
import java.util.Map;

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

    // https://leetcode.com/problems/number-of-unique-flavors-after-sharing-k-candies/description/?envType=weekly-question&envId=2024-11-15
    static class Solution3 {
        public int shareCandies(int[] candies, int k) {
            int uniqueFlav = 0;
            Map<Integer, Integer> flavFreq = new HashMap<>();
            for (int c : candies) {
                flavFreq.put(c, flavFreq.getOrDefault(c, 0) + 1);
                if (flavFreq.get(c) == 1) {
                    uniqueFlav++;
                }
            }

            int usedInWindow = 0;
            for (int i = 0; i < k; i++) {
                flavFreq.put(candies[i], flavFreq.get(candies[i]) - 1);
                if (flavFreq.get(candies[i]) == 0) {
                    usedInWindow++;
                }
            }

            int maxFlav = uniqueFlav - usedInWindow;

            for (int i = k; i < candies.length; i++) {
                flavFreq.put(candies[i - k], flavFreq.get(candies[i - k]) + 1);
                if (flavFreq.get(candies[i - k]) == 1) {
                    usedInWindow--;
                }

                flavFreq.put(candies[i], flavFreq.get(candies[i]) - 1);
                if (flavFreq.get(candies[i]) == 0) {
                    usedInWindow++;
                }

                maxFlav = Math.max(maxFlav, uniqueFlav - usedInWindow);
            }

            return maxFlav;
        }
    }
}
