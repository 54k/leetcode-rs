package leetcode_grind;

import java.util.*;

public class Day724 {
    // https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-ii/description/?envType=daily-question&envId=2024-11-10
    static class Solution1 {
        boolean check(int[] nums, int targetSum, int windowSize) {
            int arrayLength = nums.length;
            int[] bitCounts = new int[32];
            for (int right = 0; right < arrayLength; right++) {
                updateBitCounts(bitCounts, nums[right], 1);
                if (right >= windowSize) {
                    updateBitCounts(bitCounts, nums[right - windowSize], -1);
                }
                if (right >= windowSize - 1 && convertBitCountsToNumber(bitCounts) >= targetSum) {
                    return true;
                }
            }
            return false;
        }

        void updateBitCounts(int[] bitCounts, int number, int delta) {
            for (int bitPosition = 0; bitPosition < 32; bitPosition++) {
                if (((number >> bitPosition) & 1) != 0) {
                    bitCounts[bitPosition] += delta;
                }
            }
        }

        int convertBitCountsToNumber(int[] bitCounts) {
            int number = 0;
            for (int bitPosition = 0; bitPosition < 32; bitPosition++) {
                if (bitCounts[bitPosition] != 0) {
                    number |= 1 << bitPosition;
                }
            }
            return number;
        }

        public int minimumSubarrayLength(int[] nums, int k) {
            int lo = 1, hi = nums.length;
            int ans = -1;
            while (lo <= hi) {
                int mid = lo + (hi - lo) / 2;
                if (check(nums, k, mid)) {
                    ans = mid;
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            }
            return ans;
        }
    }

    static class Solution2 {
        public int minimumSubarrayLength(int[] nums, int k) {
            int minLength = Integer.MAX_VALUE;
            int windowStart = 0;
            int windowEnd = 0;
            int[] bitCounts = new int[32];

            while (windowEnd < nums.length) {
                updateBitCounts(bitCounts, nums[windowEnd], 1);

                while (windowStart <= windowEnd && convertBitCountsToNumber(bitCounts) >= k) {
                    minLength = Math.min(minLength, windowEnd - windowStart + 1);

                    updateBitCounts(bitCounts, nums[windowStart], -1);
                    windowStart++;
                }

                windowEnd++;
            }

            return minLength == Integer.MAX_VALUE ? -1 : minLength;
        }

        void updateBitCounts(int[] bitCounts, int number, int delta) {
            for (int bitPosition = 0; bitPosition < 32; bitPosition++) {
                if (((number >> bitPosition) & 1) != 0) {
                    bitCounts[bitPosition] += delta;
                }
            }
        }

        int convertBitCountsToNumber(int[] bitCounts) {
            int result = 0;
            for (int bitPosition = 0; bitPosition < 32; bitPosition++) {
                if (bitCounts[bitPosition] != 0) {
                    result |= 1 << bitPosition;
                }
            }
            return result;
        }
    }

    // https://leetcode.com/problems/intersection-of-three-sorted-arrays/description/?envType=weekly-question&envId=2024-11-08
    static class Solution3 {
        public List<Integer> arraysIntersection(int[] arr1, int[] arr2, int[] arr3) {
            List<Integer> ans = new ArrayList<>();
            Map<Integer, Integer> counter = new TreeMap<>();

            for (Integer e : arr1) {
                counter.put(e, counter.getOrDefault(e, 0) + 1);
            }
            for (Integer e : arr2) {
                counter.put(e, counter.getOrDefault(e, 0) + 1);
            }
            for (Integer e : arr3) {
                counter.put(e, counter.getOrDefault(e, 0) + 1);
            }

            for (Integer item : counter.keySet()) {
                if (counter.get(item) == 3) {
                    ans.add(item);
                }
            }
            return ans;
        }
    }

    
}
