package leetcode_grind;

public class Day1089 {
    // https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/description/?envType=daily-question&envId=2025-11-12
    static class Solution1 {
        public int minOperations(int[] nums) {
            int n = nums.length;
            int num1 = 0;
            int g = 0;
            for (int x : nums) {
                if (x == 1) {
                    num1++;
                }
                g = gcd(g, x);
            }
            if (num1 > 0) {
                return n - num1;
            }
            if (g > 1) {
                return -1;
            }

            int minLen = n;
            for (int i = 0; i < n; i++) {
                int currentGcd = 0;
                for (int j = i; j < n; j++) {
                    currentGcd = gcd(currentGcd, nums[j]);
                    if (currentGcd == 1) {
                        minLen = Math.min(minLen, j - i + 1);
                        break;
                    }
                }
            }
            return minLen + n - 2;
        }

        int gcd(int a, int b) {
            while (b != 0) {
                int temp = b;
                b = a % b;
                a = temp;
            }
            return a;
        }
    }
}
