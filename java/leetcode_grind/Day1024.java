package leetcode_grind;

import java.util.TreeMap;

public class Day1024 {
    // https://leetcode.com/problems/number-of-subarrays-with-bounded-maximum/description/
    static class Solution1 {
        public int numSubarrayBoundedMax(int[] nums, int left, int right) {
            return count(nums, right) - count(nums, left - 1);
        }

        int count(int[] A, int bound) {
            int ans = 0, cur = 0;
            for (int x : A) {
                cur = x <= bound ? cur + 1 : 0;
                ans += cur;
            }
            return ans;
        }
    }

    // https://leetcode.com/problems/largest-unique-number/description/?envType=weekly-question&envId=2025-09-08
    static class Solution2 {
        public int largestUniqueNumber(int[] nums) {
            TreeMap<Integer, Integer> frequencyMap = new TreeMap<>();
            for (int num : nums) {
                frequencyMap.put(num, frequencyMap.getOrDefault(num, 0) + 1);
            }
            int largestUnique = -1;
            for (Integer num : frequencyMap.descendingKeySet()) {
                if (frequencyMap.get(num) == 1) {
                    largestUnique = num;
                    break;
                }
            }
            return largestUnique;
        }
    }

    // https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/description/?envType=daily-question&envId=2025-09-08
    static class Solution3 {
        public int[] getNoZeroIntegers(int n) {
            for (int A = 1; A < n; A++) {
                int B = n - A;
                if (!String.valueOf(A).contains("0") && !String.valueOf(B).contains("0")) {
                    return new int[] { A, B };
                }
            }
            return new int[0];
        }
    }
}
