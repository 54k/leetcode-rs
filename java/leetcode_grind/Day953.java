package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Day953 {
    // https://leetcode.com/problems/longest-harmonious-subsequence/description/?envType=daily-question&envId=2025-06-30
    static class Solution1 {
        public int findLHS(int[] nums) {
            int res = 0;
            for (int i = 0; i < (1 << nums.length); i++) {
                int count = 0, min = Integer.MAX_VALUE, max = Integer.MIN_VALUE;
                for (int j = 0; j < nums.length; j++) {
                    if ((i & (1 << j)) != 0) {
                        min = Math.min(min, nums[j]);
                        max = Math.max(max, nums[j]);
                        count++;
                    }
                }
                if (max - min == 1) {
                    res = Math.max(res, count);
                }
            }
            return res;
        }
    }

    static class Solution2 {
        public int findLHS(int[] nums) {
            int res = 0;
            for (int i = 0; i < nums.length; i++) {
                int count = 0;
                boolean flag = false;
                for (int j = 0; j < nums.length; j++) {
                    if (nums[j] == nums[i]) {
                        count++;
                    } else if (nums[j] + 1 == nums[i]) {
                        count++;
                        flag = true;
                    }
                }
                if (flag) {
                    res = Math.max(count, res);
                }
            }
            return res;
        }
    }

    static class Solution3 {
        public int findLHS(int[] nums) {
            Arrays.sort(nums);
            int prev_count = 1, res = 0;
            for (int i = 0; i < nums.length; i++) {
                int count = 1;
                if (i > 0 && nums[i] - nums[i - 1] == 1) {
                    while (i < nums.length - 1 && nums[i] == nums[i + 1]) {
                        count++;
                        i++;
                    }
                    res = Math.max(res, count + prev_count);
                    prev_count = count;
                } else {
                    while (i < nums.length - 1 && nums[i] == nums[i + 1]) {
                        count++;
                        i++;
                    }
                    prev_count = count;
                }
            }
            return res;
        }
    }

    static class Solution4 {
        public int findLHS(int[] nums) {
            Map<Integer, Integer> map = new HashMap<>();
            int res = 0;
            for (int num : nums) {
                map.put(num, map.getOrDefault(num, 0) + 1);
            }
            for (int key : map.keySet()) {
                if (map.containsKey(key + 1)) {
                    res = Math.max(res, map.get(key) + map.get(key + 1));
                }
            }
            return res;
        }
    }
}
