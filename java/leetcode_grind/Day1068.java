package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;
import java.util.TreeSet;
import java.util.function.Consumer;
import java.util.function.IntUnaryOperator;

public class Day1068 {
    // https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-ii/description/?envType=daily-question&envId=2025-10-22
    static class Solution1 {
        public int maxFrequency(int[] nums, int k, int numOperations) {
            Arrays.sort(nums);
            int ans = 0;
            Map<Integer, Integer> numCount = new HashMap<>();
            Set<Integer> modes = new TreeSet<>();

            Consumer<Integer> addMode = value -> {
                modes.add(value);
                if (value - k >= nums[0]) {
                    modes.add(value - k);
                }
                if (value + k <= nums[nums.length - 1]) {
                    modes.add(value + k);
                }
            };

            int lastNumIndex = 0;
            for (int i = 0; i < nums.length; i++) {
                if (nums[i] != nums[lastNumIndex]) {
                    numCount.put(nums[lastNumIndex], i - lastNumIndex);
                    ans = Math.max(ans, i - lastNumIndex);
                    addMode.accept(nums[lastNumIndex]);
                    lastNumIndex = i;
                }
            }

            numCount.put(nums[lastNumIndex], nums.length - lastNumIndex);
            ans = Math.max(ans, nums.length - lastNumIndex);
            addMode.accept(nums[lastNumIndex]);

            IntUnaryOperator leftBound = value -> {
                int left = 0;
                int right = nums.length - 1;
                while (left < right) {
                    int mid = (left + right) / 2;
                    if (nums[mid] < value) {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }
                return left;
            };

            IntUnaryOperator rightBound = value -> {
                int left = 0;
                int right = nums.length - 1;
                while (left < right) {
                    int mid = (left + right + 1) / 2;
                    if (nums[mid] > value) {
                        right = mid - 1;
                    } else {
                        left = mid;
                    }
                }
                return left;
            };

            for (int mode : modes) {
                int l = leftBound.applyAsInt(mode - k);
                int r = rightBound.applyAsInt(mode + k);
                int tempAns;
                if (numCount.containsKey(mode)) {
                    tempAns = Math.min(r - l + 1, numCount.get(mode) + numOperations);
                } else {
                    tempAns = Math.min(r - l + 1, numOperations);
                }
                ans = Math.max(ans, tempAns);
            }
            return ans;
        }
    }

}
