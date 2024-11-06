package leetcode_grind;

import java.util.Arrays;

public class Day720 {
    // https://leetcode.com/problems/find-if-array-can-be-sorted/description/?envType=daily-question&envId=2024-11-06
    static class Solution1 {
        public boolean canSortArray(int[] nums) {
            int[] values = Arrays.copyOf(nums, nums.length);
            int n = values.length;

            for (int i = 0; i < n; i++) {
                for (int j = 0; j < n - i - 1; j++) {
                    if (values[j] <= values[j + 1]) {
                        continue;
                    } else {
                        if (Integer.bitCount(values[j]) == Integer.bitCount(values[j + 1])) {
                            int temp = values[j];
                            values[j] = values[j + 1];
                            values[j + 1] = temp;
                        } else {
                            return false;
                        }
                    }
                }
            }

            return true;
        }
    }
}
