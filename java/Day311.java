import java.util.Arrays;
import java.util.HashSet;

public class Day311 {
    static class Solution {
        public int findDuplicateSort(int[] nums) {
            Arrays.sort(nums, 0, nums.length);
            for (var i = 1; i < nums.length; i++) {
                if (nums[i] == nums[i - 1]) {
                    return nums[i];
                }
            }
            return -1;
        }

        public int findDuplicateSet(int[] nums) {
            var set = new HashSet<Integer>();
            for (var n : nums) {
                if (set.contains(n)) {
                    return n;
                }
                set.add(n);
            }
            return -1;
        }
    }
}
