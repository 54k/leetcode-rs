package leetcode_grind;

import java.util.Arrays;

public class Day1064 {
    // https://leetcode.com/problems/maximum-number-of-distinct-elements-after-operations/description/?envType=daily-question&envId=2025-10-18
    static class Solution1 {
        public int maxDistinctElements(int[] nums, int k) {
            Arrays.sort(nums);
            int cnt = 0;
            int prev = Integer.MIN_VALUE;
            for (int num : nums) {
                int curr = Math.min(Math.max(num - k, prev + 1), num + k);
                if (curr > prev) {
                    cnt++;
                    prev = curr;
                }
            }
            return cnt;
        }
    }

    // https://leetcode.com/problems/reverse-integer/description/
    static class Solution2 {
        public int reverse(int x) {
            int rev = 0;
            while (x != 0) {
                int pop = x % 10;
                x /= 10;
                if (rev > Integer.MAX_VALUE / 10 ||
                        (rev == Integer.MAX_VALUE / 10 && pop > 7)) {
                    return 0;
                }
                if (rev < Integer.MIN_VALUE / 10 ||
                        (rev == Integer.MIN_VALUE / 10 && pop < -8)) {
                    return 0;
                }
                rev = rev * 10 + pop;
            }
            return rev;
        }
    }

}
