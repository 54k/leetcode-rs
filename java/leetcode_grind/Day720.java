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

    static class Solution2 {
        public boolean canSortArray(int[] nums) {
            int numOfSetBits = Integer.bitCount(nums[0]);
            int maxOfSegment = nums[0];
            int minOfSegment = nums[0];

            int maxOfPrevSegment = Integer.MIN_VALUE;

            for (int i = 1; i < nums.length; i++) {
                if (Integer.bitCount(nums[i]) == numOfSetBits) {
                    maxOfSegment = Math.max(maxOfSegment, nums[i]);
                    minOfSegment = Math.min(minOfSegment, nums[i]);
                } else {
                    if (minOfSegment < maxOfPrevSegment) {
                        return false;
                    }

                    maxOfPrevSegment = maxOfSegment;
                    maxOfSegment = nums[i];
                    minOfSegment = nums[i];
                    numOfSetBits = Integer.bitCount(nums[i]);
                }
            }
            return (minOfSegment < maxOfPrevSegment) ? false : true;
        }
    }

    static class Solution3 {
        public boolean canSortArray(int[] nums) {
            int n = nums.length;
            int[] values = Arrays.copyOf(nums, n);
            for (int i = 0; i < n - 1; i++) {
                if (values[i] <= values[i + 1]) {
                    continue;
                } else {
                    if (Integer.bitCount(values[i]) == Integer.bitCount(values[i + 1])) {
                        int temp = values[i];
                        values[i] = values[i + 1];
                        values[i + 1] = temp;
                    } else {
                        return false;
                    }
                }
            }
            for (int i = n - 1; i >= 1; i--) {
                if (values[i] >= values[i - 1]) {
                    continue;
                } else {
                    if (Integer.bitCount(values[i]) == Integer.bitCount(values[i - 1])) {
                        int temp = values[i];
                        values[i] = values[i - 1];
                        values[i - 1] = temp;
                    } else {
                        return false;
                    }
                }
            }
            return true;
        }
    }
}
