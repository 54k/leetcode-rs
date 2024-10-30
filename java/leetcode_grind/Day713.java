package leetcode_grind;

import java.util.*;

public class Day713 {
    // https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/description/?envType=daily-question&envId=2024-10-30
    static class Solution1 {
        public int minimumMountainRemovals(int[] nums) {
            int N = nums.length;
            int[] lisLength = new int[N];
            int[] ldsLength = new int[N];

            Arrays.fill(lisLength, 1);
            Arrays.fill(ldsLength, 1);

            for (int i = 0; i < N; i++) {
                for (int j = i - 1; j >= 0; j--) {
                    if (nums[i] > nums[j]) {
                        lisLength[i] = Math.max(lisLength[i], lisLength[j] + 1);
                    }
                }
            }

            for (int i = N - 1; i >= 0; i--) {
                for (int j = i + 1; j < N; j++) {
                    if (nums[i] > nums[j]) {
                        ldsLength[i] = Math.max(ldsLength[i], ldsLength[j] + 1);
                    }
                }
            }

            int minRemovals = Integer.MAX_VALUE;
            for (int i = 0; i < N; i++) {
                if (lisLength[i] > 1 && ldsLength[i] > 1) {
                    minRemovals = Math.min(minRemovals, N - lisLength[i] - ldsLength[i] + 1);
                }
            }

            return minRemovals;
        }
    }
}
