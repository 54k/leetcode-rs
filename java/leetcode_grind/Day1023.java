package leetcode_grind;

public class Day1023 {
    // https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/description/?envType=daily-question&envId=2025-09-07
    static class Solution1 {
        public int[] sumZero(int n) {
            int[] ans = new int[n];
            int index = 0;
            for (int i = 1; i <= n / 2; i++) {
                ans[index++] = i;
                ans[index++] = -i;
            }
            if (n % 2 == 1) {
                ans[index] = 0;
            }
            return ans;
        }
    }
}
