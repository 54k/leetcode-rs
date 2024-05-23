package leetcode_grind;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Day555 {
    // https://leetcode.com/problems/the-number-of-beautiful-subsets/description/
    static class Solution1 {
        public int beautifulSubsets(int[] nums, int k) {
            return countBeautifulSubsets(nums, k, 0, 0);
        }

        private int countBeautifulSubsets(int[] nums, int difference, int index, int mask) {
            if (index == nums.length) {
                return mask > 0 ? 1 : 0;
            }

            boolean isBeautiful = true;

            for (int j = 0; j < index && isBeautiful; ++j) {
                isBeautiful = ((1 << j) & mask) == 0 || Math.abs(nums[j] - nums[index]) != difference;
            }

            int skip = countBeautifulSubsets(nums, difference, index + 1, mask);
            int take;
            if (isBeautiful) {
                take = countBeautifulSubsets(nums, difference, index + 1, mask + (1 << index));
            } else {
                take = 0;
            }
            return skip + take;
        }
    }

    // https://leetcode.com/problems/the-number-of-beautiful-subsets/description
    static class Solution2 {
        public int beautifulSubsets(int[] nums, int k) {
            Map<Integer, Integer> freqMap = new HashMap<>();
            Arrays.sort(nums);
            return countBeautifulSubsets(nums, k, freqMap, 0) - 1;
        }

        private int countBeautifulSubsets(int[] nums, int difference, Map<Integer, Integer> freqMap, int i) {
            if (i == nums.length) {
                return 1;
            }

            int totalCount = countBeautifulSubsets(nums, difference, freqMap, i + 1);

            if (!freqMap.containsKey(nums[i] - difference)) {
                freqMap.put(nums[i], freqMap.getOrDefault(nums[i], 0) + 1);

                totalCount += countBeautifulSubsets(nums, difference, freqMap, i + 1);
                freqMap.put(nums[i], freqMap.get(nums[i]) - 1);

                if (freqMap.get(nums[i]) == 0) {
                    freqMap.remove(nums[i]);
                }
            }

            return totalCount;
        }
    }
}
