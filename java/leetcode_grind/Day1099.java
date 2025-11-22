package leetcode_grind;

import java.util.Arrays;

public class Day1099 {
    // https://leetcode.com/problems/find-minimum-operations-to-make-all-elements-divisible-by-three/description/?envType=daily-question&envId=2025-11-22
    static class Solution1 {
        public int minimumOperations(int[] nums) {
            return Arrays.stream(nums).map(n -> Math.min(n % 3, 1)).sum();
        }
    }
}
