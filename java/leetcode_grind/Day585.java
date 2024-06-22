package leetcode_grind;

import java.util.ArrayDeque;
import java.util.HashMap;
import java.util.Map;

public class Day585 {
    // https://leetcode.com/problems/count-number-of-nice-subarrays/description
    static class Solution1 {
        public int numberOfSubarrays(int[] nums, int k) {
            int currSum = 0, subarrays = 0;
            Map<Integer, Integer> prefixSum = new HashMap<>();
            prefixSum.put(currSum, 1);

            for (int i = 0; i < nums.length; i++) {
                currSum += nums[i] % 2;
                if (prefixSum.containsKey(currSum - k)) {
                    subarrays += prefixSum.get(currSum - k);
                }
                prefixSum.put(currSum, prefixSum.getOrDefault(currSum, 0) + 1);
            }
            return subarrays;
        }
    }

    // https://leetcode.com/problems/count-number-of-nice-subarrays/description/
    static class Solution2 {
        public int numberOfSubarrays(int[] nums, int k) {
            int subarrays = 0, lastPopped = -1, initialGap = -1;
            var oddIndices = new ArrayDeque<Integer>();
            for (int i = 0; i < nums.length; i++) {
                if (nums[i] % 2 == 1) {
                    oddIndices.addLast(i);
                }

                if (oddIndices.size() > k) {
                    lastPopped = oddIndices.removeFirst();
                }

                if (oddIndices.size() == k) {
                    initialGap = oddIndices.peekFirst() - lastPopped;
                    subarrays += initialGap;
                }

            }
            return subarrays;
        }
    }

    // https://leetcode.com/problems/count-number-of-nice-subarrays/description
    static class Solution3 {
        public int numberOfSubarrays(int[] nums, int k) {
            int subrrays = 0, initialGap = 0, qsize = 0, start = 0;
            for (int end = 0; end < nums.length; end++) {
                if (nums[end] % 2 == 1) {
                    qsize++;
                }
                if (qsize == k) {
                    initialGap = 0;

                    while (qsize == k) {
                        qsize -= nums[start] % 2;
                        initialGap++;
                        start++;
                    }
                }
                subrrays += initialGap;
            }
            return subrrays;
        }
    }
}
