package leetcode_grind;

public class Day1117 {
    // https://leetcode.com/problems/count-the-number-of-computer-unlocking-permutations/description/?envType=daily-question&envId=2025-12-10
    static class Solution1 {
        public int countPermutations(int[] complexity) {
            int n = complexity.length;
            for (int i = 1; i < n; i++) {
                if (complexity[i] <= complexity[0]) {
                    return 0;
                }
            }

            int ans = 1;
            int mod = 1000000007;
            for (int i = 2; i < n; i++) {
                ans = (int) (((long) ans * i) % mod);
            }
            return ans;
        }
    }
}
