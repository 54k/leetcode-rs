package leetcode_grind;

import java.util.Arrays;

public class Day983 {
    // https://leetcode.com/problems/smallest-subarrays-with-maximum-bitwise-or/description/?envType=daily-question&envId=2025-07-29
    static class Solution1 {
        public int[] smallestSubarrays(int[] nums) {
            int n = nums.length;
            int[] pos = new int[31];
            Arrays.fill(pos, -1);
            int[] ans = new int[n];

            for (int i = n - 1; i >= 0; i--) {
                int j = i;
                for (int bit = 0; bit < 31; bit++) {
                    if ((nums[i] & (1 << bit)) == 0) {
                        if (pos[bit] != -1) {
                            j = Math.max(j, pos[bit]);
                        }
                    } else {
                        pos[bit] = i;
                    }
                }
                ans[i] = j - i + 1;
            }

            return ans;
        }
    }
}
