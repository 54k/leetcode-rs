package leetcode_grind;

import java.util.*;

public class Day763 {
    // https://leetcode.com/problems/maximum-beauty-of-an-array-after-applying-operation/description/?envType=daily-question&envId=2024-12-11
    static class Solution1 {
        public int maximumBeauty(int[] nums, int k) {
            Arrays.sort(nums);
            int maxBeauty = 0;
            for (int i = 0; i < nums.length; i++) {
                int upperBound = findUpperBound(nums, nums[i] + 2 * k);
                maxBeauty = Math.max(maxBeauty, upperBound - i + 1);
            }
            return maxBeauty;
        }

        int findUpperBound(int[] arr, int val) {
            int low = 0, high = arr.length - 1, result = 0;
            while (low <= high) {
                int mid = low + (high - low) / 2;
                if (arr[mid] <= val) {
                    result = mid;
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
            return result;
        }
    }

    static class Solution2 {
        public int maximumBeauty(int[] nums, int k) {
            Arrays.sort(nums);
            int right = 0;
            int maxBeauty = 0;
            for (int left = 0; left < nums.length; left++) {
                while (right < nums.length && nums[right] - nums[left] <= 2 * k) {
                    right++;
                }
                maxBeauty = Math.max(maxBeauty, right - left);
            }
            return maxBeauty;
        }
    }

    // https://leetcode.com/problems/avoid-flood-in-the-city/description/
    static class Solution3 {
        public int[] avoidFlood(int[] rains) {
            HashMap<Integer, Integer> lastRain = new HashMap<>();
            TreeSet<Integer> dry = new TreeSet<>();

            int n = rains.length;
            int[] ans = new int[n];

            for (int i = 0; i < n; i++) {
                int x = rains[i];
                if (x == 0) {
                    dry.add(i);
                    ans[i] = 1;
                } else {
                    Integer id = lastRain.get(x);
                    if (id == null) {
                        lastRain.put(x, i);
                    } else {
                        Integer dryingDay = dry.higher(id);
                        if (dryingDay == null) {
                            return new int[0];
                        }
                        ans[dryingDay] = x;
                        dry.remove(dryingDay);
                        lastRain.put(x, i);
                    }
                    ans[i] = -1;
                }
            }
            return ans;
        }
    }
}
