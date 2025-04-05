package leetcode_grind;

import java.util.ArrayList;
import java.util.List;

public class Day868 {
    // https://leetcode.com/problems/sum-of-all-subset-xor-totals/description/?envType=daily-question&envId=2025-04-05
    static class Solution1 {
        public int subsetXORSum(int[] nums) {
            List<List<Integer>> subsets = new ArrayList<>();
            generateSubsets(nums, 0, new ArrayList<>(), subsets);

            int result = 0;
            for (List<Integer> subset : subsets) {
                int subsetXORTotal = 0;
                for (int num : subset) {
                    subsetXORTotal ^= num;
                }
                result += subsetXORTotal;
            }
            return result;
        }

        void generateSubsets(int[] nums, int index, List<Integer> subset, List<List<Integer>> subsets) {
            if (index == nums.length) {
                subsets.add(new ArrayList<>(subset));
                return;
            }

            subset.add(nums[index]);
            generateSubsets(nums, index + 1, subset, subsets);
            subset.remove(subset.size() - 1);

            generateSubsets(nums, index + 1, subset, subsets);
        }
    }
}
