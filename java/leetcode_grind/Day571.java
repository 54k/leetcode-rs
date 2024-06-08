package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;

public class Day571 {
    // https://leetcode.com/problems/continuous-subarray-sum/description
    static class Solution1 {
        public boolean checkSubarraySum(int[] nums, int k) {
            int prefixMod = 0;
            HashMap<Integer, Integer> modSeen = new HashMap<>();
            modSeen.put(0, -1);

            for (int i = 0; i < nums.length; i++) {
                prefixMod = (prefixMod + nums[i]) % k;

                if (modSeen.containsKey(prefixMod)) {
                    if (i - modSeen.get(prefixMod) > 1) {
                        return true;
                    }
                } else {
                    modSeen.put(prefixMod, i);
                }
            }

            return false;
        }
    }

    // https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous/description/
    static class Solution2 {
        public int minOperations(int[] nums) {
            int n = nums.length;
            int ans = n;

            HashSet<Integer> unique = new HashSet<>();
            for (int num : nums) {
                unique.add(num);
            }
            int[] newNums = new int[unique.size()];
            int index = 0;
            for (int num : unique) {
                newNums[index++] = num;
            }

            Arrays.sort(newNums);

            for (int i = 0; i < newNums.length; i++) {
                int left = newNums[i];
                int right = left + n - 1;
                int j = binarySearch(newNums, right);
                int count = j - i;
                ans = Math.min(ans, n - count);
            }

            return ans;
        }

        int binarySearch(int[] newNums, int target) {
            int left = -1;
            int right = newNums.length;

            while (left + 1 < right) {
                int mid = (left + right) / 2;
                if (target < newNums[mid]) {
                    right = mid;
                } else {
                    left = mid;
                }
            }

            return right;
        }
    }
}
